# epoll

侦听多个文件描述符，如果有指定事件发生，对其进行响应。

* interest list     要侦听的文件描述符放入此队列
* ready list        I/O就绪，放入此列表，但是不会从 interest list 中删除． 

## Level-triggered and edge-triggered

1. rfd 为从 pipe 中读数据的文件描述符， 注册到了 epoll 实例
2. 向 pip 写入 2kB 数据
3. epoll_wait 返回 rfd
4. 从 rfd 读取 1kB 数据
5. epoll_wait 返回

如果 rfd 使用 edge-triggered , 第 5 步一直阻塞。原因是 edge-triggered 只有在变化产生时才产生 events。在上面的例子，一个事件在第 2 步产生，在第 3 步被消费。在第 4 步没有读完全部数据。在第 5 步将阻塞。

当采用边缘触发时，应该使用 nonblocking file descriptors

1. with nonblocking file descriptors
2. by waiting for an event only after read or write return EAGAIN.

使用 level-triggered interface, epoll 相当于 faster poll

使用 edge-triggered, 会产生多个相同的事件，可以使用 EPOLLONESHOT 选项, 在 epoll_wait 收到一个事件返回时 disable 相关的文件描述符。此时，调用者控制描述符是否继续接收事件 epoll_ctl with EPOLL_CTL_MOD

如果多个线程或进程侦听同一个文件描述符，当一个事件到来时，只有一个线程或进程被唤醒。

## Interaction with autosleep

/sys/power/autosleep 系统处于 sleep ，当事件产生，系统被唤醒，当事件进入到队列中，系统继续 sleep。如果要让系统等到事件处理完成， 设置 epoll_ctl EPOLLWAKEUP 

## /proc interfaces

/proc/sys/fs/epoll/max_user_watches  当前用户 epool 能同时侦听多少个文件描述符(per real user ID)

/proc/[pid]/fdinfo  当前进程被侦听的文件描述符

kcmp KCMP_EPOLL_TFD     一个文件描述符是否被侦听

# epoll_create epoll_create1()

返回文件描述符，使用 close 关闭

## EPOLL_CLOEXEC ( close-on-exec )

调用 execve 成功后，文件描述符自动关闭，不成功，不关闭。多用于多线程程序中

# epoll_ctl

对 the interest list 进行增，删，改

* EPOLL_CTL_ADD
* EPOLL_CTL_MOD
* EPOLL_CTL_DEL

## The struct epoll_event 

```
typedef union epoll_data {
    void        *ptr;
    int          fd;
    uint32_t     u32;
    uint64_t     u64;
} epoll_data_t;

struct epoll_event {
    uint32_t     events;      /* Epoll events */
    epoll_data_t data;        /* User data variable */
};
```

events 是一个 bit mask:

* EPOLLIN       读
* EPOLLOUT      写
* EPOLLRDHUP
    Stream socket peer 关闭连接，or shut down writing half of connection. (在边缘触发时，此标志可用于探测 peer shutdown)
* EPOLLPRI      文件描述符产生 exceptional 
* EPOLLERR
    epoll_wait 始终会等待此消息。没必要进行设置
* EPOLLERR
   epoll_wait 始终会等待此消息。没必要进行设置
   当从 pipe or a stream socket 读数据, 此事件表明对方关闭连接。当通道中所有数据都被读完，read 返回  0 (end of file)
* EPOLLET   边缘触发，默认为 Level Trig‐gered.  
* EPOLLONESHOT
    在 epoll_wait 收到一个事件返回时 disable 相关的文件描述符。此时，调用者控制描述符是否继续接收事件 epoll_ctl with EPOLL_CTL_MOD
* EPOLLWAKEUP
    EPOLLONESHOT and EPOLLET 被 clear, 进程有 CAP_BLOCK_SUSPEND 的能力。当事件产生时，确保系统不进入 "suspend" or "hibernate"

    在使用此标志时，必须检测进程是否有 CAP_BLOCK_SUSPEND 能力，如果没有，则不进行设置，否则程序会出错。
* EPOLLEXCLUSIVE
    防止惊群: 同一文件描述符在多个 epoll 实例中, 只有部分设置了 EPOLLEXCLUSIVE 。事件提交给所有未设置 EPOLLEXCLUSIVE 的实例。设置了 EPOLLEXCLUSIVE 的实例，至少有一个收到此事件。

    可以和 EPOLLEXCLUSIVE 一同设置的标志: EPOLLIN, EPOLLOUT, EPOLLWAKEUP, and EPOLLET. EPOLLHUP and EPOLLERR 所有情况下都会进行提交。其它标志会报错。

    EPOLLEXCLUSIVE 只能通过 EPOLL_CTL_ADD 进行设置。通过 EPOLL_CTL_MOD 设置报错。

# epoll_wait

阻塞，直到：

* a file descriptor delivers an event;
* the call is interrupted by a signal handler; or
* the timeout expires.

* timeout = -1 block indefinitely
* timeout = 0 return immediately, even if no events are available.

```
typedef union epoll_data {
       void    *ptr;
       int      fd;
       uint32_t u32;
       uint64_t u64;
} epoll_data_t;

struct epoll_event {
       uint32_t     events;    /* Epoll events */
       epoll_data_t data;      /* User data variable */
};
```

# epoll_pwait()

线程安全

```bash
ready = epoll_pwait(epfd, &events, maxevents, timeout, &sigmask);
```

相当于

```c
sigset_t origmask;
// The sigmask argument may be specified as NULL, in which case
pthread_sigmask(SIG_SETMASK, &sigmask, &origmask);
ready = epoll_wait(epfd, &events, maxevents, timeout);
pthread_sigmask(SIG_SETMASK, &origmask, NULL);
```