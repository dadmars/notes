# 并发程序

## 硬件

## 架构

```bash
---------  --------
| cpu0  |  | cpu1 |
| cache |  |cache |
-------------------
|  interconnect   |
-------------------
        ^
        |
        v
-----------------------     ----------
| system interconnect | <-> | memory |
-----------------------     ----------
        ^
        |
        v
-------------------
|  interconnect   |
---------  --------
| cpu2  |  | cpu3 |
| cache |  |cache |
-------------------
```

### cpu pipeline

cpu 处理过程：得到指令，解析指令，执行指令。为提高性能，当处理当前指令时，会将下一条要处理的指令进行预加载和预解析。如果预加载的指令错误，则要刷新 pipeline cache。性能会下降。

### memory references

### atomic 操作

### memory barriers

### cache misses

### I/O

### 硬件优化

1. 大的 cacheline
2. 大的 cache

## 计数

### 频繁写，少量读

1. 每个线程定义一个线程变量，在线程内部对其更新
2. 最后把各线程变量相加。此过程要进行同步

### 计数，不超过一定的限制

1. 每个线程定义一个线程变量，在线程内部对其更新
2. 对每个线程定义一个限额, 将总限额平均分配至每个线程
3. 动态平衡，当线程达到最大值，移动一定限额到总限额。当达到接近0时，向总限额得到一定的数量。此过程要进行同步。

```c
unsigned long __thread counter = 0;     // 线程本地计数器
unsigned long __thread countermax = 0;  // 线程本地最大值

unsigned long globalcountmax = 10000;   // 最大限额
unsigned long globalcount = 0;          // 当前数量
unsigned long globalreserve = 0;        // 剩下数量

unsigned long *counterp[NR_THREADS] = { NULL }; // 每个线程的计数器

// opt1
// 每个线程的最大值。 如果没有此限制
// 当 counter 接近 globalcountmax 时 add_count 会失败
// 当 counter 接近 0 时 sub_count 会失败
//
// 但是当负载较大，会带来性能问题
#define MAX_COUNTERMAX 100

DEFINE_SPINLOCK(gblcnt_mutex); // 锁

void balance_count(void) {
    countermax = globalcountmax - globalcount - globalreserve;
    countermax /= num_online_threads();

    // opt1
    // 限制每个线程的最大值
    if (countermax > MAX_COUNTERMAX)
        countermax = MAX_COUNTERMAX;

    globalreserve += countermax;

    counter = countermax / 2;
    if (counter > globalcount)
        counter = globalcount;

    globalcount -= counter;
}

int add_count(unsigned long delta) {
    if (countermax - counter >= delta) {
        WRITE_ONCE(counter, counter + delta);
        return 1;
    }

    spin_lock(&gblcnt_mutex); // 加锁

    balize_count();
    if (globalcountmax - globalcount - globalreserve < delta) {
        spin_unlock(&gblcnt_mutex);
        return 0;
    }

    globalcount += delta;
    balance_count();

    spin_unlock(&gblcnt_mutex); // 解锁
    return 1;
}

int sub_count(unsigned long delta) {
    if (counter >= delta) {
        WRITE_ONCE(counter, counter - delta);
        return 1;
    }

    spin_lock(&gblcnt_mutex); // 加锁

    globalize_count();
    if (globalcount < delta) {
        spin_unlock(&gblcnt_mutex);
        return 0;
    }
    globalcount -= delta;
    balance_count();

    spin_unlock(&gblcnt_mutex); // 解锁
    return 1;
}

unsigned long read_count(void) {
    int t;
    unsigned long sum;

    spin_lock(&gblcnt_mutex);
    sum = globalcount;
    for_each_thread(t)
        if (counterp[t] != NULL)
            sum += READ_ONCE(*counterp[t]);
    spin_unlock(&gblcnt_mutex);

    return sum;
}

void globalize_count(void) {
    globalcount += counter;
    counter = 0;
    globalreserve -= countermax;
    countermax = 0;
}

void count_register_thread(void) {
    int idx = smp_thread_id();

    spin_lock(&gblcnt_mutex);
    counterp[idx] = &counter;
    spin_unlock(&gblcnt_mutex);
}

void count_unregister_thread(int nthreadsexpected) {
    int idx = smp_thread_id();

    spin_lock(&gblcnt_mutex);
    globalize_count();
    counterp[idx] = NULL;
    spin_unlock(&gblcnt_mutex);
}
```

1. 使用 atomic
2. counter 和 countermax 合并到一起， 32位的值，高16位为counter，低16位为countermax
3. 但是会因为 atomic 使性能下降，可以使用信息队列

```c
atomic_t __thread counterandmax = ATOMIC_INIT(0);

unsigned long globalcountmax = 1 << 25;
unsigned long globalcount = 0;
unsigned long globalreserve = 0;

atomic_t *counterp[NR_THREADS] = { NULL };

DEFINE_SPINLOCK(gblcnt_mutex);

#define CM_BITS (sizeof(atomic_t) * 4)
#define MAX_COUNTERMAX ((1 << CM_BITS) - 1)

void split_counterandmax_int(int cami, int *c, int *cm) {
    *c = (cami >> CM_BITS) & MAX_COUNTERMAX;
    *cm = cami & MAX_COUNTERMAX;
}

void split_counterandmax(atomic_t *cam, int *old, int *c, int *cm) {
    unsigned int cami = atomic_read(cam);

    *old = cami;
    split_counterandmax_int(cami, c, cm);
}

int merge_counterandmax(int c, int cm) {
    unsigned int cami;
    cami = (c << CM_BITS) | cm;
    return ((int)cami);
}

int add_count(unsigned long delta) {
    int c;
    int cm;
    int old;
    int new;

    do {
        split_counterandmax(&counterandmax, &old, &c, &cm);
        if (delta > MAX_COUNTERMAX || c + delta > cm)
            goto slowpath;
        new = merge_counterandmax(c + delta, cm);
    } while (atomic_cmpxchg(&counterandmax, old, new) != old);

    return 1;

slowpath:
    spin_lock(&gblcnt_mutex);
    globalize_count();
    if (globalcountmax - globalcount - globalreserve < delta) {
        flush_local_count();
        if (globalcountmax - globalcount - globalreserve < delta) {
            spin_unlock(&gblcnt_mutex);
            return 0;
        }
    }
    globalcount += delta;
    balance_count();
    spin_unlock(&gblcnt_mutex);
    return 1;
}

int sub_count(unsigned long delta) {
    int c;
    int cm;
    int old;
    int new;

    do {
        split_counterandmax(&counterandmax, &old, &c, &cm);
        if (delta > c)
            goto slowpath;
        new = merge_counterandmax(c - delta, cm);
    } while (atomic_cmpxchg(&counterandmax, old, new) != old);

    return 1;

slowpath:
    spin_lock(&gblcnt_mutex);
    globalize_count();

    if (globalcount < delta) {
        flush_local_count();
        if (globalcount < delta) {
            spin_unlock(&gblcnt_mutex);
            return 0;
        }
    }

    globalcount -= delta;
    balance_count();
    spin_unlock(&gblcnt_mutex);
    return 1;
}

unsigned long read_count(void) {
    int c;
    int cm;
    int old;
    int t;
    unsigned long sum;

    spin_lock(&gblcnt_mutex);

    sum = globalcount;

    for_each_thread(t)
        if (counterp[t] != NULL) {
            split_counterandmax(counterp[t], &old, &c, &cm);
            sum += c;
        }

    spin_unlock(&gblcnt_mutex);
    return sum;
}

void globalize_count(void) {
    int c;
    int cm;
    int old;

    split_counterandmax(&counterandmax, &old, &c, &cm);
    globalcount += c;
    globalreserve -= cm;
    old = merge_counterandmax(0, 0);
    atomic_set(&counterandmax, old);
}

void flush_local_count(void) {
    int c;
    int cm;
    int old;
    int t;
    int zero;

    if (globalreserve == 0)
        return;

    zero = merge_counterandmax(0, 0);
    for_each_thread(t)
        if (counterp[t] != NULL) {
            old = atomic_xchg(counterp[t], zero);
            split_counterandmax_int(old, &c, &cm);
            globalcount += c;
            globalreserve -= cm;
        }
}

static void balance_count(void)
{
    int c;
    int cm;
    int old;
    unsigned long limit;

    limit = globalcountmax - globalcount - globalreserve;
    limit /= num_online_threads();
    if (limit > MAX_COUNTERMAX)
        cm = MAX_COUNTERMAX;
    else
        cm = limit;
    globalreserve += cm;
    c = cm / 2;
    if (c > globalcount)
    c = globalcount;
    globalcount -= c;
    old = merge_counterandmax(c, cm);
    atomic_set(&counterandmax, old);
}

void count_register_thread(void) {
    int idx = smp_thread_id();

    spin_lock(&gblcnt_mutex);
    counterp[idx] = &counterandmax;
    spin_unlock(&gblcnt_mutex);
}

void count_unregister_thread(int nthreadsexpected) {
    int idx = smp_thread_id();

    spin_lock(&gblcnt_mutex);
    globalize_count();
    counterp[idx] = NULL;
    spin_unlock(&gblcnt_mutex);
}
```

1. 当要和 globecounter 交互时，使用信号和状态机

```c
#define THEFT_IDLE 0 // need flush -> req
#define THEFT_REQ 1 // counting -> ack   !counting -> ready
#define THEFT_ACK 2 // done counting -> ready
#define THEFT_READY 3 // flush 后 -> idle

int __thread theft = THEFT_IDLE;  // 当前线程的状态
int __thread counting = 0;        // 当前线程是否在计数

unsigned long __thread counter = 0;     // 当前线程计数器
unsigned long __thread countermax = 0;  // 当前线程最大值

unsigned long globalcountmax = 10000;
unsigned long globalcount = 0;
unsigned long globalreserve = 0;

unsigned long *counterp[NR_THREADS] = { NULL };         // 每个线程的计数
unsigned long *countermaxp[NR_THREADS] = { NULL };      // 每个线程的最大值

int *theftp[NR_THREADS] = { NULL };  // 每个线程的状态

DEFINE_SPINLOCK(gblcnt_mutex);

#define MAX_COUNTERMAX 100

void globalize_count(void) {
    globalcount += counter;
    counter = 0;
    globalreserve -= countermax;
    countermax = 0;
}

// 处理SIGUSR1信号
void flush_local_count_sig(int unused) {
    if (READ_ONCE(theft) != THEFT_REQ)
        return;

    smp_mb(); // 内存序不作优化
    WRITE_ONCE(theft, THEFT_ACK);
    if (!counting) {
        WRITE_ONCE(theft, THEFT_READY);
    }
    smp_mb();
}

void flush_local_count(void) {
    int t;
    thread_id_t tid;

    // 对每个线程
    for_each_tid(t, tid)
        // 状态不为空
        if (theftp[t] != NULL) {
            // 线程最大值为0,状态设为THEFT_READY
            if (*countermaxp[t] == 0) {
                WRITE_ONCE(*theftp[t], THEFT_READY);
                continue;
            }

            // 线程最大值不为,状态设为THEFT_REQ
            WRITE_ONCE(*theftp[t], THEFT_REQ);

            // 发送信号
            pthread_kill(tid, SIGUSR1);
        }

    // 对每个线程
    for_each_tid(t, tid) {
        // 状态不为空
        if (theftp[t] == NULL)
            continue;

        // 一直等待当前状态变为THEFT_READY
        while (READ_ONCE(*theftp[t]) != THEFT_READY) {
            // 等待一毫秒
            poll(NULL, 0, 1);

            // 状态为THEFT_REQ, 重新发送信号
            if (READ_ONCE(*theftp[t]) == THEFT_REQ)
                pthread_kill(tid, SIGUSR1);
        }

        globalcount += *counterp[t];
        *counterp[t] = 0;
        globalreserve -= *countermaxp[t];
        *countermaxp[t] = 0;
        WRITE_ONCE(*theftp[t], THEFT_IDLE);
}

void balance_count(void) {
    countermax = globalcountmax - globalcount - globalreserve;
    countermax /= num_online_threads();

    if (countermax > MAX_COUNTERMAX)
        countermax = MAX_COUNTERMAX;

    globalreserve += countermax;
    counter = countermax / 2;

    if (counter > globalcount)
        counter = globalcount;
    globalcount -= counter;
}

int add_count(unsigned long delta) {
    int fastpath = 0;

    // 开始计数
    WRITE_ONCE(counting, 1);

    barrier();

    if (READ_ONCE(theft) <= THEFT_REQ && countermax - counter >= delta) {
        WRITE_ONCE(counter, counter + delta);
        fastpath = 1;
    }

    barrier();

    // 结束计数
    WRITE_ONCE(counting, 0);

    barrier();

    if (READ_ONCE(theft) == THEFT_ACK) {
        smp_mb();
        WRITE_ONCE(theft, THEFT_READY);
    }

    if (fastpath)
        return 1;

    spin_lock(&gblcnt_mutex);
    globalize_count();

    if (globalcountmax - globalcount - globalreserve < delta) {
        // 发送信号
        flush_local_count();

        if (globalcountmax - globalcount - globalreserve < delta) {
            spin_unlock(&gblcnt_mutex);
            return 0;
        }
    }

    globalcount += delta;
    balance_count();
    spin_unlock(&gblcnt_mutex);
    return 1;
}

int sub_count(unsigned long delta) {
    int fastpath = 0;

    WRITE_ONCE(counting, 1);

    barrier();

    if (READ_ONCE(theft) <= THEFT_REQ && counter >= delta) {
        WRITE_ONCE(counter, counter - delta);
        fastpath = 1;
    }

    barrier();

    WRITE_ONCE(counting, 0);

    barrier();

    if (READ_ONCE(theft) == THEFT_ACK) {
        smp_mb();
        WRITE_ONCE(theft, THEFT_READY);
    }

    if (fastpath)
        return 1;

    spin_lock(&gblcnt_mutex);

    globalize_count();

    if (globalcount < delta) {
        flush_local_count();
        if (globalcount < delta) {
            spin_unlock(&gblcnt_mutex);
            return 0;
        }
    }

    globalcount -= delta;
    balance_count();
    spin_unlock(&gblcnt_mutex);
    return 1;
}

unsigned long read_count(void) {
    int t;
    unsigned long sum;

    spin_lock(&gblcnt_mutex);

    sum = globalcount;

    for_each_thread(t)
        if (counterp[t] != NULL)
            sum += READ_ONCE(*counterp[t]);

    spin_unlock(&gblcnt_mutex);
    return sum;
}

void count_init(void) {
    // 设置信号处理函数
    struct sigaction sa;
    sa.sa_handler = flush_local_count_sig;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = 0;

    // 处理SIGUSR1信号
    if (sigaction(SIGUSR1, &sa, NULL) != 0) {
        perror("sigaction");
        exit(EXIT_FAILURE);
    }
}

void count_register_thread(void) {
    int idx = smp_thread_id();
    spin_lock(&gblcnt_mutex);
    counterp[idx] = &counter;
    countermaxp[idx] = &countermax;
    theftp[idx] = &theft;
    spin_unlock(&gblcnt_mutex);
}

void count_unregister_thread(int nthreadsexpected) {
    int idx = smp_thread_id();
    spin_lock(&gblcnt_mutex);
    globalize_count();
    counterp[idx] = NULL;
    countermaxp[idx] = NULL;
    theftp[idx] = NULL;
    spin_unlock(&gblcnt_mutex);
}
```

1. 将问题分隔提高性能和扩展性