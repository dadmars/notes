# 安装

## Installation

```bash
curl https://sh.rustup.rs -sSf | sh

rustup update
rustup self uninstall
rustup doc
rustup doc --book

rustc --version

cargo --version
```

## Hello, world

```rust
Filename: main.rs

fn main() {
    println!("Hello, world!");
}
```

rustc main.rs

```bash
cargo new hello_cargo

cargo check
cargo run
cargo build
cargo build --release
```

# 特性

* expression-based 的语言。rust 中的语句分为两种，expression 和 statement:
  * expression: 执行指令，计算出一个结果，并返回。语句不以 ; 结束。{}, if 是 expression
  * statement: 执行指令，但是不返回任何结果。语句以 ; 结束。let, fn 是statement
* 变量默认不可编辑
* 强类型静态语言：意味着编译时，要知道每一个变量的类型（占用多少内存）
* 变量赋值时默认使用 move 语意：意味着函数参数传值和返回值都是move语意

# 生存期

引用使用 lifetime ，其它变量使用 ownership，来确定各自的生存范围。

## ownership

* 每个值都有一个 owner
  * 在退出作用域后，自动释放(调用 drop )
* 一个值同一时刻只能有一个 owner
  * 赋值操作是 move 语意
  * 函数参数是 move 语意
  * 函数返回值是 move 语意

类型如果实现了 Copy trait ，则不会有 move 语意。Copy trait 与 Drop trait 不能同时实现。

含有 Copy trait 的有： int, float, bool, char

## Lifetimes

* 每个引用都有一个 lifetime ,指明引用的作用域
* 在函数参数中用来表明多个引用的作用域关系

```rust
fn lo<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    }
    else {
        b
    }
}
```

# 输出

```rust
println!("{:>5}", 100);
println!("{:>5}", 1000);
println!("{:>5}", 10000);
/*
  100
 1000
10000
*/

println!("{:<5} padded", 100);
println!("{:<5} padded", 1000);
println!("{:<5} padded", 10000);
/*
100   padded
1000  padded
10000 padded
*/

println!("[{:^8}]", 100);
println!("[{:^8}]", 1000);
println!("[{:^8}]", 10000);
/*
[    100    ]
[    1000   ]
[   10000   ]
*/

println!("[{:_^8}]", "t");
println!("[{:^8}]", 100);
println!("[{:^8}]", 1000);
println!("[{:^8}]", 10000);
/*
[_____t_____]
[    100    ]
[    1000   ]
[   10000   ]
*/
```

# 调试

```rust
let x = 3;
dbg!(x);
```

# 注释

```rust
// xxxxx

/// 函数说明
///
/// # Example
///
/// ```
/// let a = 5;
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

//! 加到模块头
//!
```

# 对内存的表示

## 连续的内存

### 整形

#### 1个字节(8位)

```rust
let v: i8 = 8;
let v: u8 = b'A'; // Byte (u8 only)

// bool
let t = true;
let f: bool = false; // with explicit type annotation
```

#### 2个字节(16位)

```rust
let v: i16 = 1_000; // Decimal
let v: u16 = 8u16;
```

#### 4个字节(32位)

```rust
let v: i32 = 0xFF; // Hex
let v: u32 = 0o77; // Octal
```

#### 8个字节(64位)

```rust
let v: i64 = 8; // 64-bit
let v: u64 = 0b1111_0000; // Binary
```

#### 16个字节(128位)

```rust
let v: i128 = 8;
let v: u128 = 8;
```

#### 平台相关

```rust
let v: isize = 8;
let v: usize = 8;
```

#### 字符类型( char )

4个字节，unicode 码

```rust
let c: char = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

### 浮点形

#### 32位

```rust
let y: f32 = 3.0; // f32
```

#### 64位

```rust
let x = 2.0; // f64
```

### 数组

程序运行发现数组越界时，程序会崩溃。保证了程序的安全性。

```rust
let a = [1, 2, 3, 4, 5];
let a = [3; 5]; // [3, 3, 3, 3, 3]

let a: [i32; 5] = [1, 2, 3, 4, 5];

let x = a[0]; // 返回类型为 T
let x = &a[1]; // 返回类型为 &T
let x = &mut a[1]; // 返回类型为 &mut T
```

### 字符串

不支持索引：[]，可以通过bytes(), chars()，来分别以 u8 和 utf8 进行索引。

```rust
/////////////////////////////
let mut v = String::new();
v.push_str("aaa");
v.push('b');

/////////////////////////////
let a = "aaa";
let v = a.to_string();
let v = "aaa".to_string();

/////////////////////////////
let s1 = String.from("aaa");
let s2 = String.from("bbb");
let s3 = String.from("ccc");

let s = s1 + &s2 + "-" + &s3;
let s = String::fromat!("{}-{}-{}", s1, s2, s3);

/////////////////////////////
for c in "aaa".chars() {
    println!("{}", c);
}

/////////////////////////////
for c in "aaa".bytes() {
    println!("{}", c);
}

/////////////////////////////
let b = s.as_bytes();
for (i, &item) in b.iter().enumerate() {
    if item == b' ' {
        println!("{}", i);
    }
}

/////////////////////////////
s.split_whitespace();
```

### 向量

```rust
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];

let mut v = Vec::new();
v.push(2);

let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];
match v.get(2) {
    Some(third) => println!("{}", third),
    None => println!(""),
}

let v = vec![1, 2, 3, 4, 5];
for i in &v {
    println!("{}", i);
}

for i in &mut v {
    *i += 5;
}
```

## 内存数据的读写

### 直接读写

```rust
let mut x: i32 = 5;
x = 6;
```

### 只读

```rust
let x = 5;
```

### shadwing

可以改变类型

一个名字被 const 限定后，不能通过 let 进行 shadwing。但是可以通过 const shadwing。

```rust
let x = 5;
let x = "now x's type is &str";

const B: i32 = 7;
const B: u32 = 8;
```

下面的情况都不允许：

```rust
const B: i32 = 7;
let B = 4; // error

let B = 4; // error
const B: i32 = 7;
```

### 间接读写

#### 指针

##### 指针类型

unsafe, 分为两种： *const T, *mut T

```rust
let v: i32 = 0;
let p: *const i32 = &v;

let mut v: i32 = 0;
let p: *mut i32 = &v;

// To get a pointer to a boxed value, dereference the box
let v: Box<i32> = Box::new(10);
let p: *const i32 = &*v;

let mut v: Box<i32> = Box::new(88);
let p: *mut i32 = &mut *v;
```

##### 空指针

rust 没有空值的概念，所以，也没有空指针的概念。而是要调用函数创建一个空指针。

内部实现时，空指针还是为0

```rust
pub const fn null<T>() -> *const T { 0 as *const T }
pub const fn null_mut<T>() -> *mut T { 0 as *mut T }
```

只能通过函数来判断指针是否为空，不同的类型就可以有自己的实现。所以，两个is_null都为true的指针不一定相等。

```rust
use std::ptr;

let p: *const i32 = ptr::null();
assert!(p.is_null());

let p: *mut i32 = ptr::null_mut();
assert!(p.is_null());
```

##### 得到指向的值

```rust
pub unsafe fn as_ref<'a>(self) -> Option<&'a T>

// as_ref_unchecked, dereference the pointer
let ptr: *const u8 = &10u8 as *const u8;

unsafe {
    let val_back = &*ptr;
    println!("We got back the value: {}!", val_back);
}
```

##### 指针运算

只能调用相应的函数，不能直接用 +， - 等运算符。

#### 智能指针

实现了 Deref, DerefMut 和 Drop 的结构。

* *y ====> *(y.deref())
* 当传递引用到函数时，会自动进行 deref 的转换
* Deref 从一种类型到另一种类型的转换
  * &T, &mutT 到 &U， T: Deref<Target=U>
  * &mut T 到 &mut U， T: DerefMut<Target=U>
* std::mem::drop 可手工调用 drop

##### Box<T>

* 线程不安全
* 使用编译时不确定大小的类型
* 占用大量内存，不希望进行 copy
* 只关心类型实现的功能( Trait )，不关心具体类型

```rust
let val: u8 = 5;
let boxed: Box<u8> = Box::new(val);
drop(boxed);

let boxed: Box<u8> = Box::new(5);
let val: u8 = *boxed;
```

##### 引用计数

* Rc 线程不安全
* Arc 线程安全

```rust
use std::rc::Rc;

////////////////////////////////////////
let a = Rc::new(4);
let b = Rc::clone(&a);
let c = Rc::clone(&a);
println!("count after creating a = {}", Rc::strong_count(&a));

let d = Rc::downgrade(&a); // Weak<T>
match d.upgrade() {
    Some(v) => println!("a = {}", v),
    None => ()
}

////////////////////////////////////////
let a = Rc::new(RefCell::new(4));

*a.borrow_mut() += 10;

////////////////////////////////////////
struct Node {
    val: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>
}

let leaf = Rc::new(Node{
    val: 4,
    parent: RefCell::new(Weak::new()),
    child: RefCell::new(vec![])
});

let branch = Rc::new(Node{
    val: 5,
    parent: RefCell::new(Weak::new()),
    child: RefCell::new(vec![Rc::clone(&leaf)])
});

*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

println!("{}", leaf.parent.borrow().upgrade());
```

##### 写时copy

```rust
use std::borrow::Cow;
```

##### interior mutability

在有多个引用的情况下对值进行修改。
* RefCell 线程不安全
* Mutex 线程安全

RefCell<T> 与 Box<T> 相似，区别前者在运行时，后者在编译时。

* borrow() 返回 Ref<T>
* borrow_mut() 返回 RefMut<T>
* 或者有多个 Ref<T>， 或者有一个 RefMut<T>

```rust
use std::cell::RefCell;

struct Mock {
    me: RefCell<Vec<String>>
}

impl Mock {
    fn send(&self, s: &str) {
        self.me.borrow_mut().push(String::from(s);
    }
}
```

#### 引用

引用不发生 ownership 的转移，引用必须有效。

同一个作用域内，对同一个值，只能有一个 mut 引用或多个 immutable 引用。

引用就是一个指针（和c++里的引用有所不同）。对引用的值修改要使用 *

```rust
let v: i32 = 0;
let r = &v;

let mut v: i32 = 0;
let r = &mut v;
*r = 3;

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

##### 连续空间部分内容的引用( Slice )

###### String Slice

是按照 byte 进行索引的。如果字符串不是ascii，[a..b]如果范围不在字符边界，程序会崩溃。

```rust
let s = String::from("你好吧");
let hello = &s[0..3]; // 你
let hello = &s[0..2]; // error
let hello = &s[0..1]; // error

let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

let slice = &s[..2];

let len = s.len();
let slice = &s[3..];

let slice = &s[..];

fn first_word(s: &str) -> &str {
    ...
}
```

###### Other Slice

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];

fn first_word(s: &[i32]) -> &[i32] {
    ...
}
```

## 类型转换

### as

```rust
let a: u8 = 89.0 as u8;
assert_eq!('B' as u32, 66);
assert_eq!(a as char, 'Y');
let b: f32 = a as f32 + 10.5;
assert_eq!(true as u8 + b as u8, 100);
```

### 通过函数

```rust
let guess = String::new();
let guess: u32 = guess.trim().parse().expect("Not a number!");
```

## destructuring

```rust
// tuple
let t = (500, 6.4, 1);
let (x, y, z) = t;
println!("{}", y);

// struct
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

let Point { x: a, y: b } = p;
assert_eq!(0, a);
assert_eq!(7, b);

// shorthand for patterns that match struct fields
let Point { x, y } = p;
assert_eq!(0, x);
assert_eq!(7, y);

/////
match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x), // match any x and y=0
    Point { x: 0, y } => println!("On the y axis at {}", y), // match x=0 and any y
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),// mathc any x and y
}

// ignoring remaining parts of a value with ..
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}

let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Some numbers: {}, {}", first, last);
    },
}

// match guard
let num = Some(4);
match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}

let x = 4;
let y = false;
match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}

// @ bindings
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3...7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10...12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

## 非连续的内存

### map

```rust
use std::collections::HashMap;

//////////////////////////////////
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Blue")).or_insert(10);

//////////////////////////////////
let a = vec!["a", "b"];
let b = vec![1, 2];

let v: HashMap<_, _> = a.iter().zip(b.iter()).collect();

//////////////////////////////////
let v = v.get("a");

//////////////////////////////////
for (k, val) in &v {
    println!("{}-{}", k, val);
}

//////////////////////////////////
let text = "aaa bbb aaa bbb";

let mut map = HashMap::new();
for w in map.split_whitespace() {
    let c = map.entry(w).or_insert(0);
    *c += 1;
}
```

### 链表

### 树

# 空值

没有空值，使用 Option<T>

```rust
let b: Option<i32> = None;

let b = Some(4);

let a = match b {
    Some(v) => v,
    None => 0,
};
```

# 空对象

## unit type

```rust
()
```

# 结构化

## 元组( tuple )

```rust
let t: (i32, f64, u8) = (500, 6.4, 1);
let x = t.0;
let y = t.1;
```

## 结构( struct )

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//////////////////////////////
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

//////////////////////////////
user1.active = false;

//////////////////////////////
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//////////////////////////////
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

//////////////////////////////
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};

//////////////////////////////
struct User<'a> {
    name: &'a str
}

impl<'a> User<'a> {
    fn num(&self) -> i32 {
        3
    }
}
```

### Tuple Structs

```rust
struct Color(i32, i32, i32);

let black = Color(0, 0, 0);
```

### Unit-Like Structs

```rust
struct Color;
```

## 枚举( enum )

```rust
//////////////////////////////
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_kind: IpAddrKind) { }
route(IpAddrKind::V4);

//////////////////////////////
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

# 扩展

## Range

```rust
for i in 1..4 {
    println!("{}", i);
}
```

## Struct init

```rust
#[derive(Debug, Default)]
Struct Foo {
    x: i32,
    y: i32
}

let a = Foo { x: 1, y: 2 };
let b = Foo { x: 1, ..a };
let c = Foo { x: 1, ..Default::default() };
```

# 程序指令的执行

## 主函数（ 程序进入点 ）

```rust
fn main() {
}

use std::error::Error   ;
fn main() -> Result<(), Box<dyn Error>> {
}
```

## 函数

默认返回最后一句 expression（无 ; 结尾） 的值，为返回值

## 一般函数

```rust
fn plus_two(){
    // the return is empty tuple: ()
}

fn plus_two(x: i32, y: i32){
    // the return is empty tuple: ()
}

fn plus_two(x: i32, y: i32) -> i32 {
    if x {
        return y + 1;
    }

    x + 1 // no ;
}
```

## 分支

### if

```rust
//////////////////////////////
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
}
else if number % 2 == 0 {
    println!("number is divisible by 2");
}
else {
    println!("number is not divisible by 4, 3, or 2");
}

//////////////////////////////
let condition = true;

let number = if condition {
    5
} else {
    6
};
```

## match

```rust
enum UsState {
    Alabama,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

let a = match coin {
    Coin::Penny => {
        println!("Lucky penny!");
        1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
    },
    _ => (),
};

//////////////////////////////////
let x = 1;
match x => {
    1 | 2 => println!("xxx"),
    1...5 => println!("xxx"),   // 1, 2, 3, 4, 5
    'a'...'c' => println!("xxx"),   // a, b, c
    _ => println!("xxx"),
}

//////////////////////////////////
fn divide(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    match (x, y) => {
        (Some(i), Some(j)) if j != 0 => Some(i / j),
        _ => None
    }
}
```

### if let

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

## 循环

### loop

```rust
loop {
    println!("again!");
}

let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
```

### for

```rust
//////////////////////////////
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}

//////////////////////////////
for number in (1..4).rev() {
    println!("{}!", number);
}

//////////////////////////////
for (index, number) in a.iter().enumerate() {
    println!("{}!", number);
}
```

### while

```rust
let mut number = 3;

while number != 0 {
    number -= 1;
}

////////////////////////
let mut val = Vec::new();
val.push(1);
val.push(2);
val.push(3);

while let Some(t) = val.pop() {
    println!("{}", t);
}
```

# 范围

```rust
for i in 2..5 {
    println!("{}", i);
}

for i in 2..=5 {
    println!("{}", i);
}
```

# 常量

只有变量可设为常量。不管在何处定义，总在全局分配，并且在整个程序生命周期内都是存在的。不能动态生成，只能在编译时确定，所以必须给出类型。

```rust
const MAX_POINTS: u32 = 100_000;
```

## 字符和字符串常量

```rust
let a = 'a'; // 字符常量

let a = "abc"; // 字符串常量
```

## 数字字面常量

数字可以 _ 分隔，可以加后缀。Byte类型不可以加后缀，可以加前缀。

```rust
b'A' // Byte (u8 only)
89u8
98_222 // 10进制
0xff  // 16进制
0o77  // 8进制
0b1111_0000  // 2进制
```

# 数据封装

## struct

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // no self, 关联函数，通过 :: 调用
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

let sq = Rectangle::square(3);
let a = sq.can_hold(&rect1);
```

# 错误处理

Result<T, E>, Ok(_), Err(_)

? : 语法糖，返回错误时，会将错误类型向返回值的错误类型转换。

panic! 宏直接退出程序，默认进行清理。可以配置，退出时不进行清理。

RUST_BACKTRACE=1 cargo run  查看堆栈

```rust
/////////////////////////////////
eprintln!("Problem parsing arguments: {}", err);

/////////////////////////////////
panic!("xxx")

[profile.release]
panic = 'abort'

/////////////////////////////////
let f = File.open("s.txt").unwrap();
let f = File.open("s.txt").expect("hhh, error");

/////////////////////////////////
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let f = match File::open("a.txt") {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

/////////////////////////////////
fn read_file() -> Result<String, io::Error> {
    let f = File::open("a.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

/////////////////////////////////
fn read_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("a.txt")?
        .read_to_string(&mut s)?;

    Ok(s)
}
```

# 代码组织( 模块化 )

## 工具

cargo 进行程序的管理

```bash
cargo login abcdefghijklmnopqrstuvwxyz012345
cargo publish
cargo yank --vers 1.0.1
cargo yank --vers 1.0.1 --undo
cargo install ripgrep

cargo new xx
cargo new --lib xx

cargo-something // command in $PATH
cargo something // run it by cargo
```

## workspace

```rust
Filename: Cargo.toml

[workspace]

members = [
    "adder",
]
```

## package

src/main.rs  与 package 同名的可执行程序
src/lib.rs  与 package 同名的lib

src/bin/  下面多个文件，表示多个可执行程序

```rust
Filename: Cargo.toml

[dependencies]
rand = "0.3.14"
add-one = { path = "../add-one" }

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

这里要注意，当 cargo 第一次下载依赖时，将下载版本号为 0.3.x 的依赖，这个 x 是当前最新的版本。不一定和指定的版本号相同。

当运行 cargo update，rand将自动更新到大于 0.3.14 小于 0.4.0 的最新版本。如果要更新到 0.4 ，要手动修改 Cargo.toml 文件。

实现原理是存在 Cargo.lock 文件，记录当前的版本号。

```rust
rand = "0.4.0"
```

## key: mod, pub

mod 中默认为 private

```rust
Filename: src/lib.rs

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_payment() {}
    }
}
```

## key: crate, self, super

```rust
crate::front_of_house::hosting::add_to_waitlist(); // Absolute path
front_of_house::hosting::add_to_waitlist(); // Relative path
super::hosting::add_to_waitlist(); // Relative path
```

## key: use, as

```rust
//////////////////////////////
use crate::front_of_house::hosting;
use self::front_of_house::hosting;
use super::hosting;

//////////////////////////////
use std::fmt::Result;
use std::io::Result as IoResult;

//////////////////////////////
pub use crate::front_of_house::hosting;

//////////////////////////////
use std::{cmp::Ordering, io};
use std::io::{self, Write};
use std::collections::*;
```

## External Packages

```rust
Filename: Cargo.toml

[dependencies]
rand = "0.5.5"

use rand::Rng;
```

## Separating Modules into Different Files

```rust
mod lib -> mod front_of_house -> mod hosting

  lib.rs
  front_of_house.rs
  front_of_house
      |
      |-- hosting.rs

//////////////////////////////
Filename: src/lib.rs

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//////////////////////////////
Filename: src/front_of_house.rs

pub mod hosting;

//////////////////////////////
Filename: src/front_of_house/hosting.rs

pub fn add_to_waitlist() {}
```

# 自动化测试

```rust
cargo test
cargo test --help
cargo test -- --help

cargo test -- --test-threads=1
cargo test -- --nocapture
cargo test -- --ignored

cargo test no_work2 // 单元测试
cargo test no_w // 单元测试

cargo test --test integration_test // 集成测试： 一个文件

cargo test -p add-one // for workspace
```

## 单元测试

```rust
Filename: src/lib.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic] // 如果 no_work panic，此测试通过
    fn no_work() {
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn no_work2() {
    }

    #[test]
    #[ignore]
    fn it_work2() -> Result<(), String> { // 返回 Result 时不能使用 should_panic
        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err(String::from("error"))
        }
    }

    #[test]
    fn it_work() {
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 3, 4);
        assert!(2 + 2 == 4);

        assert_eq!(2 + 2, 4, "xxxxx:{:?}", 4);
        assert_ne!(2 + 3, 4, "xxxxx:{:?}", 4);
        assert!(2 + 2 == 4, "xxxxx:{:?}", 4);
    }
}
```

## 集成测试

新建目录 tests ，与 src 在同一层

```rust
/////////////////////////
Filename: tests/integration_test.rs

use adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

/////////////////////////
// 公用函数放在这里，不会被当作测试用例
Filename: tests/common/mod.rs

pub fn setup() {
}
```

# 修饰器编程( Decorator )

## Display

* println!() 宏要求类型必须要实现 Display trait。
* to_string() 转换为字符串 String

```rust
#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

let rect1 = Rectangle { width: 30, height: 50 };
println!("rect1 is {:?}", rect1);
println!("rect1 is {:#?}", rect1);
```

# 面向对象

## trait

```rust
///////////////////////////////
trait Summary {
    fn summarize(&self) -> String;
    fn title(&self) -> String {
        String::from("title")
    }
}

struct Article {
    title: String
}

impl Summary for Article {
    fn summarize(&self) -> String {
        self.title
    }
}

///////////////////////////////
fn title(a: impl Summary) {
    a.summarize();
}

fn title<T: Summary>(a: T) {
    a.summarize();
}

///////////////////////////////
fn title(a: impl Summary + Display) {
    a.summarize();
}

fn title<T: Summary + Display>(a: T) {
    a.summarize();
}

///////////////////////////////
fn title<T, U>(a: T, b: U)
    where T: Summary + Display,
          U: Summary + Clone
{
    a.summarize();
}

///////////////////////////////
fn title() -> impl Summary {
    Article {
        title: "aaa".to_string()
    }
}
```

## trait object

* 所有函数不能使用泛型
* 返回类型不能为 Self

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    com: Box<dyn Draw>
}
```

## 动态类型检测

```rust
use std::any::Any;
```

# 函数式编程

* FnOnce 得到外部变量的 ownership, 使用 move
* FnMut 外部变量的 &mut
* Fn 外部变量的 &

```rust
/////////////////////////////////////////
fn add_one(x: i32) -> i32 { x + 1}
let add_one = |x: i32| -> i32 { x + 1 };
let add_one = |x| { x + 1 };
let add_one = |x| x + 1;

/////////////////////////////////////////
let x = 3;

let f = |num| {
    thread::sleep(Duration::from_secs(2));
    num * x
};

let v = f(2);

/////////////////////////////////////////
let f = move |num| {
    thread::sleep(Duration::from_secs(2));
    num * x
};

let v = f(2);

/////////////////////////////////////////
struct Cache<T>
    where T: Fn(i32) -> i32
{
    cal: T,
    val: Option<i32>
}

impl<T> Cache<T>
    where T: Fn(i32) -> i32
{
    fn new(v: T) -> Cache<T> {
        cal: T,
        val: None
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.val {
            Some(v) => v,
            None => {
                let v = (self.cal)(arg);
                self.val = Some(v);
                v
            }
        }
    }
}

let mut v = Cache::new(|num| => num);
v.value(4);
```

# 泛型

为了避免重复。告诉编译器哪些参数是 generic (<>)

```rust
///////////////////////////////////
fn gen<T>(in: &[T]) -> T {
}

///////////////////////////////////
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32> {
    fn dis(&self) -> i32 {
        self.x * self.y
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn display(&self) -> {
        if self.x > self.y {
            println!("xxx: {:?}", self.x);
        }
        else {
            println!("xxx: {:?}", self.y);
        }
    }
}

///////////////////////////////////
struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn mix<V, W>(&self, other: &Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

///////////////////////////////////
enum Option<T> {
    Some(T),
    None
}
```

## 迭代器( Iterators )

遍历collections，包含数组，向量，列表，map...

三种类型：
* iter(), 遍历对角为 &T
* iter_mut(), 遍历对角为 &mut T
* into_iter(), 遍历对角为 T

# 命令行

## 命令行参数

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
}
```

## 用户输入

```rust
use std::io::stdin;

let mut s = String::new();
stdin().read_line(&s).expect("error");
```

# 文件处理

## 打开文件

```rust
use std::fs::{self, File};
use std::io::ErrorKind;

let f = match File::open("a.txt") {
    Ok(v) => v,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => (),
        e => panic!("open file error: {}", e)
    }
}
```

## 新建文件

```rust
let f = match File::create("a.txt").unwrap_or_else(|e| {
    panic!("create file error: {}", e);
});
```

## 读写

```rust
use std::io::BufReader;

let s = fs::read_to_string("a.txt")
    .expect("read error");

//////////////////////////////////
let f = File::open("a.txt").unwrap();
let mut b = BufReader::new(f);

let mut buffer = Vec::new();
b.read_to_end(&mut buffer).unwrap();

let f = File::create("b.txt").unwrap();
f.write_all(&buffer).unwrap()?;
```

# 环境参数

CASE_XXX=1 cargo run

```rust
use std::env;

let a = env::var("CASE_XXX").is_err(); // the CASE_XXX is set or not
```

# 进程

```rust
process::exit(1);
```

# 时间

```rust
use std::thread;
use std::time::Duration;

thread::sleep(Duration::from_secs(2));

/////////////////////////////////////////
use chrono::{SecondsFormat, Duration, DateTime};

let tm ="2019-03-04T20:10:20.000Z"
let tmp = DateTime::parse_from_rfc3339(tm).ok()?;

let diff = Duration::hours(8);
let tmp = tmp.checked_sub_signed(diff)?;

let v = tmp.to_rfc3339_opts(SecondsFormat::Millis, true);
```

# 线程

* Send trait: 类型是线程安全的
* Sync trait: 类型的引用是线程安全的

## 创立线程

```rust
use std::thread;
use std::time::Duration;

let v = vec![1, 2, 3];

thread::spawn(move || {
    for i in 1..10 {
        println!("from thread:{:?}", v);
        thread::sleep(Duration::from_millis(i));
    }
});

for i in 1..5 {
    println!("from main");
    thread::sleep(Duration::from_millis(i));
}
```

## 线程结束

```rust
use std::thread;
use std::time::Duration;

let h = thread::spawn(|| {
    for i in 1..10 {
        println!("from thread");
        thread::sleep(Duration::from_millis(i));
    }
});

h.join().unwrap();

for i in 1..5 {
    println!("from main");
    thread::sleep(Duration::from_millis(i));
}
```

## 线程间通迅

channel 发送者的数据通过 send() 发送后，数据的 ownership 会转移到接收端。

```rust
use std::sync::mpsc;

///////////////////////////////////
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("aa");
    tx.send(val).unwrap();
});

let v = rx.recv().unwrap();
println!("{:?}", v);

///////////////////////////////////
thread::spawn(move || {
    let vals = vec![
        String::from("aa"),
        String::from("bb"),
        String::from("cc"),
    ];

    // 不能使用 &vals ，因为send 要进行 ownership 的转移
    for v in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(1));
    }
});

for v in rx {
    println!("{:?}", v);
}

///////////////////////////////////
let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("aa"),
        String::from("bb"),
        String::from("cc"),
    ];

    for v in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_millis(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("1"),
        String::from("2"),
        String::from("3"),
    ];

    for v in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(1));
    }
});

for v in rx {
    println!("{:?}", v);
}
```

## 线程间同步

```rust
////////////////////////////
use std::sync::Mutex;

let m = Mutex::new(5);

{
    let mut a = m.lock().unwrap();
    *a = 6;
}

println!("{:?}", m);

////////////////////////////
use std::sync::{ Mutex, Arc };

let m = Arc::new(Mutex::new(0));

let mut hs = Vec::new();

for _ in 0..10 {
    let mt = Arc::clone(&m);
    let h = thread::spawn(move ||{
        let mut a = mt.lock().unwrap();
        *a += 1;
    });

    hs.push(h);
}

for h in hs {
    h.join().unwrap;
}

println!("{}", *m.lock().unwrap())
```

# 异步

# 网络

# 调用外部程序

# 宏

## declarative macros

对宏的参数进行模式匹配，生成不同的代码

```rust
Filename: src/lib.rs

#[macro_export]
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! mainfun {
    ($x:expr) => {
        use hyper::service::service_fn;

        let addr = ([127, 0, 0, 1], $x).into();

        ...

        hyper::rt::run(server);
    };
}

#[macro_export]
macro_rules! baseuse {
    () => {
        use serde_json::Value;
    };
}
```

# Unsafe Rust
