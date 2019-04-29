# epoll
Monitoring multiple file descriptors to see if I/O is possible on any of them.

The epoll API can be used either as an edge-triggered or a level-triggered interface and scales well to large numbers of watched file descriptors.

The central concept of the epoll API is the epoll instance, an in-kernel data structure which, can be considered as a container for two lists:

* The interest list: the set of file descriptors that the process has registered an interest in monitoring.
* ready list: the set of file descriptors that are "ready" for I/O.  The ready list is a subset of the file descriptors in the interest list that is dynamically populated by the kernel as a result of I/O activity on those file descriptors.

##Level-triggered and edge-triggered
1. The file descriptor that represents the read side of a pipe (rfd) is registered on the epoll instance.
2. A pipe writer writes 2 kB of data on the write side of the pipe.
3. A call to epoll_wait(2) is done that will return rfd as a ready file descriptor.
4. The pipe reader reads 1 kB of data from rfd.
5. A call to epoll_wait(2) is done.

If the rfd file descriptor has been added to the epoll interface using the EPOLLET (edge-triggered) flag, the call to epoll_wait done in step 5 will probably hang despite the available data still present in the file input buffer; meanwhile the remote peer might be expecting a response based on the data it already sent.  The reason for this is that edge-triggered mode delivers events only when changes occur on the monitored file descriptor.  So, in step 5 the caller might end up waiting for some data that is already present inside the input buffer.  In the above example, an event on rfd will be generated because of the write done in 2 and the event is consumed in 3.  Since the read operation done in 4 does not consume the whole buffer data, the call to epoll_wait done in step 5 might block indefinitely.

An application that employs the EPOLLET flag should use nonblocking file descriptors to avoid having a blocking read or write starve a task that is handling multiple file descriptors.  The suggested way to use epoll as an edge-triggered (EPOLLET) interface is as follows:

1. with nonblocking file descriptors; and
2. by waiting for an event only after read or write return EAGAIN.

When used as a level-triggered interface, epoll is simply a faster poll.

Since even with edge-triggered epoll, multiple events can be generated upon receipt of multiple chunks of data, the caller has the option to specify the EPOLLONESHOT flag, to tell epoll to disable the associated file descriptor after the receipt of an event with epoll_wait.  When the EPOLLONESHOT flag is specified, it is the caller's responsibility to rearm the file descriptor using epoll_ctl with EPOLL_CTL_MOD.

If multiple threads (or processes, if child processes have inherited the epoll file descriptor across fork) are blocked in epoll_wait waiting on the same epoll file descriptor and a file descriptor in the interest list that is marked for edge- triggered (EPOLLET) notification becomes ready, just one of the threads (or processes) is awoken from epoll_wait.  

##Interaction with autosleep
If the system is in autosleep mode via /sys/power/autosleep and an event happens which wakes the device from sleep, the device driver will keep the device awake only until that event is queued.  To keep the device awake until the event has been processed, it is necessary to use the epoll_ctl EPOLLWAKEUP flag.

When the EPOLLWAKEUP flag is set, the system will be kept awake from the moment the event is queued, through the epoll_wait call which returns the event until the subsequent epoll_wait call.  If the event should keep the system awake beyond that time, then a separate wake_lock should be taken before the second epoll_wait(2) call.

##/proc interfaces
The following interfaces can be used to limit the amount of kernel memory consumed by epoll:

/proc/sys/fs/epoll/max_user_watches This specifies a limit on the total number of file descriptors that a user can register across all epoll instances on the system.  The limit is per real user ID.

When used as an edge-triggered interface, for performance reasons, it is possible to add the file descriptor inside the epoll interface (EPOLL_CTL_ADD) once by specifying (EPOLLIN|EPOLLOUT).  This allows you to avoid continuously switching between EPOLLIN and EPOLLOUT calling epoll_ctl(2) with EPOLL_CTL_MOD.

##Questions and answers
0. What is the key used to distinguish the file descriptors regis‐ tered in an interest list?

   The key is the combination of the file descriptor number and the open file description.

1. What happens if you register the same file descriptor on an epoll instance twice?

   You will probably get EEXIST.  However, it is possible to add a duplicate (dup(2), dup2(2), fcntl(2) F_DUPFD) file descriptor to the same epoll instance.  This can be a useful technique for fil‐ tering events, if the duplicate file descriptors are registered with different events masks.

2. Can two epoll instances wait for the same file descriptor?  If so, are events reported to both epoll file descriptors?

   Yes, and events would be reported to both.  However, careful pro‐ gramming may be needed to do this correctly.

3. Is the epoll file descriptor itself poll/epoll/selectable?

   Yes.  If an epoll file descriptor has events waiting, then it will indicate as being readable.

4. What happens if one attempts to put an epoll file descriptor into its own file descriptor set?

   The epoll_ctl(2) call fails (EINVAL).  However, you can add an epoll file descriptor inside another epoll file descriptor set.

5. Can I send an epoll file descriptor over a UNIX domain socket to another process?

   Yes, but it does not make sense to do this, since the receiving process would not have copies of the file descriptors in the interest list.

6. Will closing a file descriptor cause it to be removed from all epoll interest lists?

   Yes, but be aware of the following point.  A file descriptor is a reference to an open file description (see open(2)).  Whenever a file descriptor is duplicated via dup(2), dup2(2), fcntl(2) F_DUPFD, or fork(2), a new file descriptor referring to the same open file description is created.  An open file description continues to exist until all file descriptors referring to it have been closed.

   This means that even after a file descriptor that is part of an interest list has been closed, events may be reported for that file descriptor if other file descriptors referring to the same underlying file description remain open.  To prevent this happening, the file descriptor must be explicitly removed from the interest list (using epoll_ctl(2) EPOLL_CTL_DEL) before it is duplicated.  Alternatively, the application must ensure that all file descriptors are closed (which may be difficult if file descriptors were duplicated behind the scenes by library functions that used dup(2) or fork(2)).

7. If more than one event occurs between epoll_wait(2) calls, are they combined or reported separately?

   They will be combined.

8. Does an operation on a file descriptor affect the already collected but not yet reported events?

   You can do two operations on an existing file descriptor.  Remove would be meaningless for this case.  Modify will reread available I/O.

9. Do I need to continuously read/write a file descriptor until EAGAIN when using the EPOLLET flag (edge-triggered behavior)?

   Receiving an event from epoll_wait(2) should suggest to you that such file descriptor is ready for the requested I/O operation. You must consider it ready until the next (nonblocking) read/write yields EAGAIN.  When and how you will use the file descriptor is entirely up to you.
   For packet/token-oriented files (e.g., datagram socket, terminal in canonical mode), the only way to detect the end of the read/write I/O space is to continue to read/write until EAGAIN.
   For stream-oriented files (e.g., pipe, FIFO, stream socket), the condition that the read/write I/O space is exhausted can also be detected by checking the amount of data read from / written to the target file descriptor.  For example, if you call read(2) by asking to read a certain amount of data and read(2) returns a lower number of bytes, you can be sure of having exhausted the read I/O space for the file descriptor.  The same is true when writing using write(2).  (Avoid this latter technique if you cannot guarantee that the monitored file descriptor always refers to a stream-oriented file.)
   Possible pitfalls and ways to avoid them o Starvation (edge-triggered)

   If there is a large amount of I/O space, it is possible that by trying to drain it the other files will not get processed causing starvation.  (This problem is not specific to epoll.)

   The solution is to maintain a ready list and mark the file descriptor as ready in its associated data structure, thereby allowing the application to remember which files need to be processed but still round robin amongst all the ready files.  This also supports ignoring subsequent events you receive for file descriptors that are already ready.

   * If using an event cache...
      If you use an event cache or store all the file descriptors returned from epoll_wait(2), then make sure to provide a way to mark its closure dynamically (i.e., caused by a previous event's processing). Suppose you receive 100 events from epoll_wait(2), and in event #47 a condition causes event #13 to be closed.  If you remove the structure and close(2) the file descriptor for event #13, then your event cache might still say there are events waiting for that file descriptor causing confusion.

      One solution for this is to call, during the processing of event 47, epoll_ctl(EPOLL_CTL_DEL) to delete file descriptor 13 and close(2), then mark its associated data structure as removed and link it to a cleanup list.  If you find another event for file descriptor 13 in your batch processing, you will discover the file descriptor had been previously removed and there will be no confusion.

##NOTES
The set of file descriptors that is being monitored via an epoll file descriptor can be viewed via the entry for the epoll file descriptor in the process's /proc/[pid]/fdinfo directory.  

The kcmp(2) KCMP_EPOLL_TFD operation can be used to test whether a file descriptor is present in an epoll instance.

#epoll_create
```
int epoll_create(int size);
int epoll_create1(int flags);
```
creates a new epoll instance.  returns a file descriptor referring to the new epoll instance. When no longer required, the file descriptor should be closed by using close.

##epoll_create1()
If flags is 0, epoll_create1() is the same as epoll_create().

###EPOLL_CLOEXEC
Set the close-on-exec (FD_CLOEXEC) flag on the new file descriptor.

If the FD_CLOEXEC bit is set, the file descriptor will automatically be closed during a successful execve.  (If the execve fails, the file descriptor is left open.)  If the FD_CLOEXEC bit is not set, the file descriptor will remain open across an execve.

Note that the use of this flag is essential in some multithreaded programs, because using a separate fcntl F_SETFD operation to set the FD_CLOEXEC flag does not suffice to avoid race conditions where one thread opens a file descriptor and attempts to set its close-on-exec flag using fcntl at the same time as another thread does a fork plus execve.  Depending on the order of execution, the race may lead to the file descriptor returned by open() being unintentionally leaked to the program executed by the child process created by fork(2).  (This kind of race is in principle possible for any system call that creates a file descriptor whose close-on-exec flag should be set)

###NOTES
the size argument is no longer required, but size must still be greater than zero, in order to ensure backward compatibility

#epoll_ctl
```
int epoll_ctl(int epfd, int op, int fd, struct epoll_event *event);
```
This system call is used to add, modify, or remove entries in the interest list of the epoll instance referred to by the file descriptor epfd.  

##EPOLL_CTL_ADD
Add fd to the interest list and associate the settings specified in event with the internal file linked to fd.

##EPOLL_CTL_MOD
Change the settings associated with fd in the interest list to the new settings specified in event.

##EPOLL_CTL_DEL
Remove (deregister) the target file descriptor fd from the interest list.  The event argument is ignored and can be NULL (but see BUGS below).

##The struct epoll_event 
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

The events member is a bit mask composed by ORing together zero or more of the following available event types:
* EPOLLIN
    The associated file is available for read(2) operations.
* EPOLLOUT
    The associated file is available for write(2) operations.
* EPOLLRDHUP (since Linux 2.6.17)
    Stream socket peer closed connection, or shut down writing half of connection.  (This flag is especially useful for writing simple code to detect peer shutdown when using Edge Trig‐gered monitoring.)
* EPOLLPRI
    There is an exceptional condition on the file descriptor.  See the discussion of POLLPRI in poll(2).
* EPOLLERR
    Error condition happened on the associated file descriptor. This event is also reported for the write end of a pipe when the read end has been closed.  epoll_wait(2) will always report for this event; it is not necessary to set it in events.
* EPOLLERR
    Hang up happened on the associated file descriptor. epoll_wait(2) will always wait for this event; it is not necessary to set it in events.

   Note that when reading from a channel such as a pipe or a stream socket, this event merely indicates that the peer closed its end of the channel.  Subsequent reads from the channel will return 0 (end of file) only after all outstanding data in the channel has been consumed.
* EPOLLET
    Sets the Edge Triggered behavior for the associated file descriptor.  The default behavior for epoll is Level Trig‐gered.  
* EPOLLONESHOT (since Linux 2.6.2)
    Sets the one-shot behavior for the associated file descriptor. This means that after an event is pulled out with epoll_wait(2) the associated file descriptor is internally disabled and no other events will be reported by the epoll interface.  The user must call epoll_ctl() with EPOLL_CTL_MOD to rearm the file descriptor with a new event mask.
* EPOLLWAKEUP (since Linux 3.5)
    If EPOLLONESHOT and EPOLLET are clear and the process has the CAP_BLOCK_SUSPEND capability, ensure that the system does not enter "suspend" or "hibernate" while this event is pending or being processed.  The event is considered as being "processed" from the time when it is returned by a call to epoll_wait(2) until the next call to epoll_wait(2) on the same epoll(7) file descriptor, the closure of that file descriptor, the removal of the event file descriptor with EPOLL_CTL_DEL, or the clearing of EPOLLWAKEUP for the event file descriptor with EPOLL_CTL_MOD.
* EPOLLEXCLUSIVE (since Linux 4.5)
    Sets an exclusive wakeup mode for the epoll file descriptor that is being attached to the target file descriptor, fd. When a wakeup event occurs and multiple epoll file descriptors are attached to the same target file using EPOLLEXCLUSIVE, one or more of the epoll file descriptors will receive an event with epoll_wait(2).  The default in this scenario (when EPOLLEXCLUSIVE is not set) is for all epoll file descriptors to receive an event.  EPOLLEXCLUSIVE is thus useful for avoiding thundering herd problems in certain scenarios.

    If the same file descriptor is in multiple epoll instances, some with the EPOLLEXCLUSIVE flag, and others without, then events will be provided to all epoll instances that did not specify EPOLLEXCLUSIVE, and at least one of the epoll instances that did specify EPOLLEXCLUSIVE.

    The following values may be specified in conjunction with EPOLLEXCLUSIVE: EPOLLIN, EPOLLOUT, EPOLLWAKEUP, and EPOLLET. EPOLLHUP and EPOLLERR can also be specified, but this is not required: as usual, these events are always reported if they occur, regardless of whether they are specified in events. Attempts to specify other values in events yield the error EINVAL.

    EPOLLEXCLUSIVE may be used only in an EPOLL_CTL_ADD operation; attempts to employ it with EPOLL_CTL_MOD yield an error.  If EPOLLEXCLUSIVE has been set using epoll_ctl(), then a subsequent EPOLL_CTL_MOD on the same epfd, fd pair yields an error. A call to epoll_ctl() that specifies EPOLLEXCLUSIVE in events and specifies the target file descriptor fd as an epoll instance will likewise fail.  The error in all of these cases is EINVAL.

##RETURN VALUE
       When successful, epoll_ctl() returns zero.  When an error occurs,
       epoll_ctl() returns -1 and errno is set appropriately.

##ERRORS
       EBADF  epfd or fd is not a valid file descriptor.

       EEXIST op was EPOLL_CTL_ADD, and the supplied file descriptor fd is
              already registered with this epoll instance.

       EINVAL epfd is not an epoll file descriptor, or fd is the same as
              epfd, or the requested operation op is not supported by this
              interface.

       EINVAL An invalid event type was specified along with EPOLLEXCLUSIVE
              in events.

       EINVAL op was EPOLL_CTL_MOD and events included EPOLLEXCLUSIVE.

       EINVAL op was EPOLL_CTL_MOD and the EPOLLEXCLUSIVE flag has
              previously been applied to this epfd, fd pair.

       EINVAL EPOLLEXCLUSIVE was specified in event and fd refers to an
              epoll instance.

       ELOOP  fd refers to an epoll instance and this EPOLL_CTL_ADD
              operation would result in a circular loop of epoll instances
              monitoring one another.

       ENOENT op was EPOLL_CTL_MOD or EPOLL_CTL_DEL, and fd is not
              registered with this epoll instance.

       ENOMEM There was insufficient memory to handle the requested op
              control operation.

       ENOSPC The limit imposed by /proc/sys/fs/epoll/max_user_watches was
              encountered while trying to register (EPOLL_CTL_ADD) a new
              file descriptor on an epoll instance.  See epoll(7) for
              further details.

       EPERM  The target file fd does not support epoll.  This error can
              occur if fd refers to, for example, a regular file or a
              directory.
BUGS
    If EPOLLWAKEUP is specified in flags, but the caller does not have the CAP_BLOCK_SUSPEND capability, then the EPOLLWAKEUP flag is silently ignored.  This unfortunate behavior is necessary because no validity checks were performed on the flags argument in the original implementation, and the addition of the EPOLLWAKEUP with a check that caused the call to fail if the caller did not have the CAP_BLOCK_SUSPEND capability caused a breakage in at least one existing user-space application that happened to randomly (and uselessly) specify this bit.  A robust application should therefore double check that it has the CAP_BLOCK_SUSPEND capability if attempting to use the EPOLLWAKEUP flag.

#epoll_wait
```
#include <sys/epoll.h>

int epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout);
int epoll_pwait(int epfd, struct epoll_event *events,
                int maxevents, int timeout,
                const sigset_t *sigmask);
```
The epoll_wait() system call waits for events on the epoll(7) instance referred to by the file descriptor epfd.  The memory area pointed to by events will contain the events that will be available for the caller.  Up to maxevents are returned by epoll_wait().  The maxevents argument must be greater than zero.

The timeout argument specifies the number of milliseconds that epoll_wait() will block.  Time is measured against the CLOCK_MONOTONIC clock.  The call will block until either:
* a file descriptor delivers an event;
* the call is interrupted by a signal handler; or
* the timeout expires.

Note that the timeout interval will be rounded up to the system clock granularity, and kernel scheduling delays mean that the blocking interval may overrun by a small amount.  Specifying a timeout of -1 causes epoll_wait() to block indefinitely, while specifying a timeout equal to zero cause epoll_wait() to return immediately, even if no events are available.

The struct epoll_event is defined as:
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
The data field of each returned structure contains the same data as was specified in the most recent call to epoll_ctl(2) (EPOLL_CTL_ADD, EPOLL_CTL_MOD) for the corresponding open file description.  The events field contains the returned event bit field.

   epoll_pwait()
       epoll_pwait() allows an application to safely wait until
       either a file descriptor becomes ready or until a signal is caught.

       The following epoll_pwait() call:

           ready = epoll_pwait(epfd, &events, maxevents, timeout, &sigmask);

       is equivalent to atomically executing the following calls:

           sigset_t origmask;

           pthread_sigmask(SIG_SETMASK, &sigmask, &origmask);
           ready = epoll_wait(epfd, &events, maxevents, timeout);
           pthread_sigmask(SIG_SETMASK, &origmask, NULL);

       The sigmask argument may be specified as NULL, in which case
       epoll_pwait() is equivalent to epoll_wait().

RETURN VALUE         top

       When successful, epoll_wait() returns the number of file descriptors
       ready for the requested I/O, or zero if no file descriptor became
       ready during the requested timeout milliseconds.  When an error
       occurs, epoll_wait() returns -1 and errno is set appropriately.

ERRORS         top

       EBADF  epfd is not a valid file descriptor.

       EFAULT The memory area pointed to by events is not accessible with
              write permissions.

       EINTR  The call was interrupted by a signal handler before either (1)
              any of the requested events occurred or (2) the timeout
              expired; see signal(7).

       EINVAL epfd is not an epoll file descriptor, or maxevents is less
              than or equal to zero.
NOTES
       While one thread is blocked in a call to epoll_pwait(), it is
       possible for another thread to add a file descriptor to the waited-
       upon epoll instance.  If the new file descriptor becomes ready, it
       will cause the epoll_wait() call to unblock.

       If more than maxevents file descriptors are ready when epoll_wait()
       is called, then successive epoll_wait() calls will round robin
       through the set of ready file descriptors.  This behavior helps avoid
       starvation scenarios, where a process fails to notice that additional
       file descriptors are ready because it focuses on a set of file
       descriptors that are already known to be ready.

       Note that it is possible to call epoll_wait() on an epoll instance
       whose interest list is currently empty (or whose interest list
       becomes empty because file descriptors are closed or removed from the
       interest in another thread).  The call will block until some file
       descriptor is later added to the interest list (in another thread)
       and that file descriptor becomes ready.


注意一下，EPOLLHUP并不代表对端结束了连接，这一点需要和EPOLLRDHUP区分。通常情况下EPOLLHUP表示的是本端挂断，造成这种事件出现的原因有很多，其中一种便是出现错误，更加细致的应该是和RST联系在一起，不过目前相关文档并不是很全面，本文会进一步跟进。

epfd = epoll_init1(0);
event.events = EPOLLET | EPOLLIN;
event.data.fd = serverfd;
epoll_ctl(epfd, EPOLL_CTL_ADD, serverfd, &event);
// 主循环
while(true) {
    // 这里的timeout很重要，实际使用中灵活调整
    count = epoll_wait(epfd, &events, MAXEVENTS, timeout);
    for(i = 0; i < count; ++i) {
        if(events[i].events & EPOLLERR || events[i].events & EPOLLHUP)
            // 处理错误
        if(events[i].data.fd == serverfd)
            // 为接入的连接注册事件
        else if(events[i].events & EPOLLIN)
            // 处理可读的缓冲区
            read(events[i].data.fd, buf, len);
            event.events = EPOLLET | EPOLLOUT;
            event.data.fd = events[i].data.fd;
            epoll_ctl(epfd, EPOLL_CTL_MOD, events[i].data.fd, &event);
        else
            // 处理可写的缓冲区
            write(events[i].data.fd, buf, len);
            // 后续可以关闭fd或者MOD至EPOLLOUT
    }
}


接下来，让我们来看个示例吧。这只是一个hello world级别的代码，无论是你发送什么数据给它，它只会回复“it's echo man”。使用的是ET模式，相信对于大家应该有些许参考价值。

#include <stdio.h>  
#include <stdlib.h>  
#include <unistd.h>  
#include <errno.h>  
#include <sys/socket.h>  
#include <netinet/in.h>
#include <arpa/inet.h>
#include <fcntl.h>  
#include <netdb.h>
#include <sys/epoll.h>  
#include <string.h>  
#define MAXEVENTS 64
int create_and_bind (int port) {
    int sfd = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if(sfd == -1) {
        return -1;
    }
    struct sockaddr_in sa;
    bzero(&sa, sizeof(sa));
    sa.sin_family = AF_INET;
    sa.sin_port   = htons(port);
    sa.sin_addr.s_addr = htonl(INADDR_ANY);
    if(bind(sfd, (struct sockaddr*)&sa, sizeof(struct sockaddr)) == -1) {
        return -1;
    }
    return sfd;
}
int make_socket_non_blocking (int sfd) {
    int flags = fcntl (sfd, F_GETFL, 0);
    if (flags == -1) {
        return -1;
    }
    if(fcntl (sfd, F_SETFL, flags | O_NONBLOCK) == -1) {
        return -1;
    }
    return 0;
}
/* 此函数用于读取参数或者错误提示 */
int read_param(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf (stderr, "Usage: %s [port]\n", argv[0]);
        exit (EXIT_FAILURE);
    }
    return atoi(argv[1]);
}
int main (int argc, char *argv[]) {
    int sfd, s;
    int efd;
    struct epoll_event event;
    struct epoll_event *events;
    int port = read_param(argc, argv);
    /* 创建并绑定socket */
    sfd = create_and_bind (port);
    if (sfd == -1) {
        perror("create_and_bind");
        abort ();
    }
    /* 设置sfd为非阻塞 */
    s = make_socket_non_blocking (sfd);
    if (s == -1) {
        perror("make_socket_non_blocking");
        abort ();
    }
    /* SOMAXCONN 为系统默认的backlog */
    s = listen (sfd, SOMAXCONN);
    if (s == -1) {
        perror ("listen");
        abort ();
    }
    efd = epoll_create1 (0);
    if (efd == -1) {
        perror ("epoll_create");
        abort ();
    }
    event.data.fd = sfd;
    /* 设置ET模式 */
    event.events = EPOLLIN | EPOLLET;
    s = epoll_ctl (efd, EPOLL_CTL_ADD, sfd, &event);
    if (s == -1) {
        perror ("epoll_ctl");
        abort ();
    }
    /* 创建事件数组并清零 */
    events = calloc (MAXEVENTS, sizeof event);
    /* 开始事件循环 */
    while (1) {
        int n, i;
        n = epoll_wait (efd, events, MAXEVENTS, -1);
        for (i = 0; i < n; i++) {
            if (events[i].events & (EPOLLERR | EPOLLHUP)) {
                /* 监控到错误或者挂起 */
                fprintf (stderr, "epoll error\n");
                close (events[i].data.fd);
                continue;
            } 
            if(events[i].events & EPOLLIN) {
                if (sfd == events[i].data.fd) {
                    /* 处理新接入的socket */
                    while (1) {
                        struct sockaddr_in sa;
                        socklen_t len = sizeof(sa);
                        char hbuf[INET_ADDRSTRLEN];
                        int infd = accept (sfd, (struct sockaddr*)&sa, &len);
                        if (infd == -1) {
                            if ((errno == EAGAIN) || (errno == EWOULDBLOCK)) {
                                /* 资源暂时不可读，再来一遍 */
                                break;
                            } else {
                                perror ("accept");
                                break;
                            }
                        }
                        inet_ntop(AF_INET, &sa.sin_addr, hbuf, sizeof(hbuf));
                        printf("Accepted connection on descriptor %d "
                                    "(host=%s, port=%d)\n", infd, hbuf, sa.sin_port);
                        /* 设置接入的socket为非阻塞 */
                        s = make_socket_non_blocking (infd);
                        if (s == -1) abort ();
                        /* 为新接入的socket注册事件 */
                        event.data.fd = infd;
                        event.events = EPOLLIN | EPOLLET;
                        s = epoll_ctl (efd, EPOLL_CTL_ADD, infd, &event);
                        if (s == -1) {
                            perror ("epoll_ctl");
                            abort ();
                        }
                    }
                    //continue;
                } else {
                    /* 接入的socket有数据可读 */
                    while (1) {
                        ssize_t count;
                        char buf[512];
                        count = read (events[i].data.fd, buf, sizeof buf);
                        if (count == -1) {
                            if (errno != EAGAIN) {
                                perror ("read");
                                close(events[i].data.fd);
                            }
                            break;
                        } else if (count == 0) {
                            /* 数据读取完毕，结束 */
                            close(events[i].data.fd);
                            printf ("Closed connection on descriptor %d\n", events[i].data.fd);
                            break;
                        }
                        /* 输出到stdout */
                        s = write (1, buf, count);
                        if (s == -1) {
                            perror ("write");
                            abort ();
                        }
                        event.events = EPOLLOUT | EPOLLET;
                        epoll_ctl(efd, EPOLL_CTL_MOD, events[i].data.fd, &event);
                    }
                }
            } else if((events[i].events & EPOLLOUT) && (events[i].data.fd != sfd)) {
                /* 接入的socket有数据可写 */
                write(events[i].data.fd, "it's echo man\n", 14);
                event.events = EPOLLET | EPOLLIN;
                epoll_ctl(efd, EPOLL_CTL_MOD, events[i].data.fd, &event);
            }
        }
    }
    free (events);
    close (sfd);
    return EXIT_SUCCESS;
}

我们可以通过ncat命令和它聊天：

[codesun@lucode ~]$ ncat 127.0.0.1 8000
hello
it's echo man

    使用epoll一定要加定时器，否则后患无穷
    如果多个线程观察的fd相同(通常是server socket fd)，据说epoll_wait会有惊群问题(accept那个问题早就解决了)，但我暂时没有发现
    联合体data中的那个ptr是很有用的，只不过这就意味着你将该对象的生命周期交给了epoll，不排除会有潜在bug的影响，需要辅以timeout
    多线程环境下使用epoll，多考虑EPOLLONESHOT
    EPOLLLT也是一个不错的选择，除非你的框架能够确保每次事件触发后，都读/写至EAGAIN

Tutorial

the Makefile
```
    all: epoll_example
    epoll_example: epoll_example.c
      gcc -Wall -Werror -o $@ epoll_example.c
    clean:
      @rm -v epoll_example
```

```
    #include <stdio.h>     // for fprintf()
    #include <unistd.h>    // for close(), read()
    #include <sys/epoll.h> // for epoll_create1(), epoll_ctl(), struct epoll_event
    #include <string.h>    // for strncmp
```

Step 1: Create epoll file descriptor
First I’ll go through the process of just creating and closing an epoll instance.
```
#include <stdio.h>     // for fprintf()
#include <unistd.h>    // for close()
#include <sys/epoll.h> // for epoll_create1()

int main()
{
  int epoll_fd = epoll_create1(0);
  if(epoll_fd == -1)
  {
    fprintf(stderr, "Failed to create epoll file descriptor\n");
    return 1;
  }

  if(close(epoll_fd))
  {
    fprintf(stderr, "Failed to close epoll file descriptor\n");
    return 1;
  }
  return 0;
}
```
Level triggered and edge triggered event notifications

 In edge triggered mode we will only receive events when the state of the watched file descriptors change; whereas in level triggered mode we will continue to receive events until the underlying file descriptor is no longer in a ready state. 

Step 2: Add file descriptors for epoll to watch

The next thing to do is tell epoll what file descriptors to watch and what kinds of events to watch for.
```
#include <stdio.h>     // for fprintf()
#include <unistd.h>    // for close()
#include <sys/epoll.h> // for epoll_create1(), epoll_ctl(), struct epoll_event

int main()
{
  struct epoll_event event;
  int epoll_fd = epoll_create1(0);
  if(epoll_fd == -1)
  {
    fprintf(stderr, "Failed to create epoll file descriptor\n");
    return 1;
  }

  event.events = EPOLLIN;
  event.data.fd = 0;

  if(epoll_ctl(epoll_fd, EPOLL_CTL_ADD, 0, &event))
  {
    fprintf(stderr, "Failed to add file descriptor to epoll\n");
    close(epoll_fd);
    return 1;
  }

  if(close(epoll_fd))
  {
    fprintf(stderr, "Failed to close epoll file descriptor\n");
    return 1;
  }
  return 0;
}
```

 The event structure we pass in for the last argument lets epoll know we’re looking to watch only input events, EPOLLIN, and lets us provide some user-defined data that will be returned for events.

Step 3: Profit
```
#define MAX_EVENTS 5
#define READ_SIZE 10
#include <stdio.h>     // for fprintf()
#include <unistd.h>    // for close(), read()
#include <sys/epoll.h> // for epoll_create1(), epoll_ctl(), struct epoll_event
#include <string.h>    // for strncmp

int main()
{
  int running = 1, event_count, i;
  size_t bytes_read;
  char read_buffer[READ_SIZE + 1];
  struct epoll_event event, events[MAX_EVENTS];
  int epoll_fd = epoll_create1(0);

  if(epoll_fd == -1)
  {
    fprintf(stderr, "Failed to create epoll file descriptor\n");
    return 1;
  }

  event.events = EPOLLIN;
  event.data.fd = 0;

  if(epoll_ctl(epoll_fd, EPOLL_CTL_ADD, 0, &event))
  {
    fprintf(stderr, "Failed to add file descriptor to epoll\n");
    close(epoll_fd);
    return 1;
  }

  while(running)
  {
    printf("\nPolling for input...\n");
    event_count = epoll_wait(epoll_fd, events, MAX_EVENTS, 30000);
    printf("%d ready events\n", event_count);
    for(i = 0; i < event_count; i++)
    {
      printf("Reading file descriptor '%d' -- ", events[i].data.fd);
      bytes_read = read(events[i].data.fd, read_buffer, READ_SIZE);
      printf("%zd bytes read.\n", bytes_read);
      read_buffer[bytes_read] = '\0';
      printf("Read '%s'\n", read_buffer);

      if(!strncmp(read_buffer, "stop\n", 5))
        running = 0;
    }
  }

  if(close(epoll_fd))
  {
    fprintf(stderr, "Failed to close epoll file descriptor\n");
    return 1;
  }
  return 0;
}
```
 I used epoll_wait() to wait for events to occur from the epoll instance, the results will be stored in the events array up to MAX_EVENTS with a timeout of 30 second. The return value of epoll_wait() indicates how many members of the events array were filled with event data.

First I gave it a small string that fits in the buffer and it works fine and continues iterating over the loop. The second input was too long for the read buffer, and is where level triggering helped us out; events continued to populate until it read all of what was left in the buffer, in edge triggering mode we would have only received 1 notification and the application as-is would not progress until more was written to the file descriptor being watching.