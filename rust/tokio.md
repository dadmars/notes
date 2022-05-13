# tokio 源码分析

<!-- TOC -->

- [tokio 源码分析](#tokio-%E6%BA%90%E7%A0%81%E5%88%86%E6%9E%90)
    - [PhantomData](#phantomdata)
    - [内存结构 repr](#%E5%86%85%E5%AD%98%E7%BB%93%E6%9E%84-repr)
    - [Atomics 内存序](#atomics-%E5%86%85%E5%AD%98%E5%BA%8F)
        - [Sequentially Consistent SeqCst](#sequentially-consistent-seqcst)
        - [Acquire-Release](#acquire-release)
        - [Relaxed](#relaxed)
    - [条件变量 std::sync::Condvar](#%E6%9D%A1%E4%BB%B6%E5%8F%98%E9%87%8F-stdsynccondvar)
    - [Mutex](#mutex)
    - [运行时](#%E8%BF%90%E8%A1%8C%E6%97%B6)
        - [任务队列](#%E4%BB%BB%E5%8A%A1%E9%98%9F%E5%88%97)
        - [多线程运行时](#%E5%A4%9A%E7%BA%BF%E7%A8%8B%E8%BF%90%E8%A1%8C%E6%97%B6)
        - [运行时 handle](#%E8%BF%90%E8%A1%8C%E6%97%B6-handle)
        - [运行时内容](#%E8%BF%90%E8%A1%8C%E6%97%B6%E5%86%85%E5%AE%B9)
    - [主线程的管理](#%E4%B8%BB%E7%BA%BF%E7%A8%8B%E7%9A%84%E7%AE%A1%E7%90%86)
    - [工作线程的管理](#%E5%B7%A5%E4%BD%9C%E7%BA%BF%E7%A8%8B%E7%9A%84%E7%AE%A1%E7%90%86)

<!-- /TOC -->

## PhantomData

```rust
pub struct PhantomData<T>
 where
    T: ?Sized;
```

一个结构中加入成员 PhantomData<T>，告诉编译器此结构包含一个T类型的成员，但实际并不包含。此成员大小为0

PhantomData<T> 表示类型包含类型 T，当类型删除时，要进行 drop check

如果不想进行 drop check，使用引用，如果有 lifetime，使用 PhantomData<&'a T>，如果没有 lifetime，使用 PhantomData<*const T>

```rust
// Unused lifetime parameters
use std::marker::PhantomData;

struct Slice<'a, T: 'a> {
    start: *const T, // 下面两个成员都没有用到 'a
    end: *const T,
    phantom: PhantomData<&'a T>, // 标识 'a 被使用
}

fn borrow_vec<T>(vec: &Vec<T>) -> Slice<'_, T> {
    let ptr = vec.as_ptr();
    Slice {
        start: ptr,
        end: unsafe { ptr.add(vec.len()) },
        phantom: PhantomData,
    }
}

// Unused type parameters
use std::marker::PhantomData;
use std::mem;

struct ExternalResource<R> {
   // R 没有被此成员使用
   // *mut () 表示此指针可指向不同的类型
   resource_handle: *mut (),
   resource_type: PhantomData<R>, // 标识R被使用
}

impl<R: ResType> ExternalResource<R> {
    fn new() -> Self {
        let size_of_res = mem::size_of::<R>();
        Self {
            resource_handle: foreign_lib::new(size_of_res),
            resource_type: PhantomData,
        }
    }

    fn do_stuff(&self, param: ParamType) {
        let foreign_params = convert_params(param);
        foreign_lib::do_stuff(self.resource_handle, foreign_params);
    }
}
```

## 内存结构 repr

```rust
/*
只能用于只有一个成员(大小不为0)的 structs (可以包含大小为 0 的成员)
整个结构的 layout 和 ABI 与此成员的相同
目的是为了在成员和结构这间进行转换, 如: UnsafeCell, which can be transmuted into the type it wraps.
另外，通过 FFI 传递此结构, 内部成员类型另外一边也可以正常工作。
特别的，struct Foo(f32) 与 f32 总是有相同的 ABI
*/
#[repr(transparent)]
```

## Atomics 内存序

编译器和硬件会对程序进行优化，会对指令的顺序进行调整。这种情况在多线程程序下会产生问题。

```rust
初始: x = 0, y = 1

线程1            线程2
y = 3;          if x == 1 {
x = 1;              y *= 2;
                }

此程序有两种结果

    y = 3: (线程2在线程1完成之前进行了判断)
    y = 6: (线程2在线程1完成之后进行了判断)

第三种情况是由硬件引起的:

    y = 2: (线程2看到 x = 1, 但是没看到 y = 3, 对 y = 3 进行了覆盖)
```

为了防止上面的优化，在进行原子操作时，有三种方式

### Sequentially Consistent (SeqCst)

不进行任何优化

### Acquire-Release

Acquire 和 Release 经常成对使用。

acquire 在其之后的语句保证在后面执行，在其之前的语句无限制。release 在其之前的语句保证在之前执行，在其之后的无限制。

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

fn main() {
    let lock = Arc::new(AtomicBool::new(false)); // value answers "am I locked?"

    // ... distribute lock to threads somehow ...

    // Try to acquire the lock by setting it to true
    while lock.compare_and_swap(false, true, Ordering::Acquire) { }
    // broke out of the loop, so we successfully acquired the lock!

    // ... scary data accesses ...

    // ok we're done, release the lock
    lock.store(false, Ordering::Release);
}
```

### Relaxed

无限制

## 条件变量 std::sync::Condvar

阻塞线程，等待一个事件的发生。与一个 mutex 同时使用

```rust
pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> LockResult<MutexGuard<'a, T>>
```

自动对 mutex unlock 并阻塞线程。此函数会发生 spurious wakeups。要对条件进行检测。

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

let pair = Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = Arc::clone(&pair);

thread::spawn(move|| {
    let (lock, cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *started = true;
    // We notify the condvar that the value has changed.
    cvar.notify_one();
});

// Wait for the thread to start up.
let (lock, cvar) = &*pair;
let mut started = lock.lock().unwrap();
// As long as the value inside the `Mutex<bool>` is `false`, we wait.
// while 处理 spurious wakeups
while !*started {
    started = cvar.wait(started).unwrap();
}
```

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

let pair = Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = Arc::clone(&pair);

thread::spawn(move|| {
    let (lock, cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *started = true;
    // We notify the condvar that the value has changed.
    cvar.notify_one();
});

// wait for the thread to start up
let (lock, cvar) = &*pair;
let mut started = lock.lock().unwrap();
// as long as the value inside the `Mutex<bool>` is `false`, we wait
loop {
    let result = cvar.wait_timeout(started, Duration::from_millis(10)).unwrap();
    // 10 milliseconds have passed, or maybe the value changed!
    started = result.0;
    if *started == true {
        // We received the notification and the value has been updated, we can leave.
        break
    }
}
```

## Mutex

```rust
use std::sync::{Arc, Mutex};
use std::thread;

const N: usize = 3;

let data_mutex = Arc::new(Mutex::new(vec![1, 2, 3, 4]));
let res_mutex = Arc::new(Mutex::new(0));

let mut threads = Vec::with_capacity(N);

(0..N).for_each(|_| {
    let data_mutex_clone = Arc::clone(&data_mutex);
    let res_mutex_clone = Arc::clone(&res_mutex);

    threads.push(thread::spawn(move || {
        let mut data = data_mutex_clone.lock().unwrap();
        // This is the result of some important and long-ish work.
        let result = data.iter().fold(0, |acc, x| acc + x * 2);
        data.push(result);
        drop(data);
        *res_mutex_clone.lock().unwrap() += result;
    }));
});

let mut data = data_mutex.lock().unwrap();

// This is the result of some important and long-ish work.
let result = data.iter().fold(0, |acc, x| acc + x * 2);
data.push(result);

// We drop the `data` explicitly because it's not necessary anymore and the
// thread still has work to do. This allow other threads to start working on
// the data immediately, without waiting for the rest of the unrelated work
// to be done here.
//
// It's even more important here than in the threads because we `.join` the
// threads after that. If we had not dropped the mutex guard, a thread could
// be waiting forever for it, causing a deadlock.
drop(data);

// Here the mutex guard is not assigned to a variable and so, even if the
// scope does not end after this line, the mutex is still released: there is
// no deadlock.
*res_mutex.lock().unwrap() += result;

threads.into_iter().for_each(|thread| {
    thread
        .join()
        .expect("The thread creating or execution failed !")
});

assert_eq!(*res_mutex.lock().unwrap(), 800);
```

## 运行时

创建运行

```rust
runtime -> builder.rs
fn build_threaded_runtime(&mut self) -> io::Result<Runtime>
```

1. 创建线程池
2. 在阻塞线程池中创建一定数量的线程(cup内核数 + 最大线程数)

运行任务

1. tokio::spawn() 最后调用运行时的 spawner->spawn()

### 任务队列

```rust
runtime -> queue.rs
```

### 多线程运行时

```rust
runtime -> thread_pool -> worker.rs
```

### 运行时 handle

Handle: 指向运行时的handle, 一个引用计数的指针, 线程安全。 成员:

```rust
blocking_spawner: blocking::Spawner    阻塞线程池的 spawner
spawner: Spawner::ThreadPool(scheduler.spawner().clone())   线程池的 spawner
```

### 运行时内容

```rust
// 线程本地变量，指向当前的运行时handle
static CONTEXT: RefCell<Option<Handle>> = RefCell::new(None)
```

## 主线程的管理

1. 运行 main 函数，当前线程为主线程
2. 在线程中创建一个线程本地变量，标识是否有工作线程在此线程中工作
3. 调用block_on(f), 其中 f 为要执行的 future, 此函数是调用 tokio::spawn() 函数将任务分配到工作线程中处理

## 工作线程的管理

1. 一个线程池有一个对象 Spawn 来管理任务的创建，tokio::spawn() 最后调用的就是此对象的对应函数
