* 避免　blocking I/O 和 unnecessary evictions of cache, prefetch into cache
* 避免不必要的 copies, context switches, system calls and signals. 使用 double-buffering or ringbuffers, and calls like Linux's splice
* 依赖测试

# epoll

A struct eventpoll contains, among other things:

* a spinlock_t, "lock"
* a struct mutex, "mtx"
* two wait_queue_head_t's, "wq" and "poll_wait"

sys_epoll_wait() validates input and calls ep_poll() Its core loop:

* acquires "lock"
* 没有 events ready or signals pending:
  * release "lock", sleep on "wq"
  * ep_poll_callback() is in charge of waking up "wq"
  * a timeout is also used (LONG_MAX if we passed timeout < 0)
  * reacquire "lock", loop to event/signal check
* release "lock"
* call ep_send_events()->ep_scan_ready_list() (see below)
* 存在 events ready, pending signals, or the timeout has expired, return
* otherwise, loop

ep_send_events()->ep_scan_ready_list() is

* acquires "mtx"
* acquires "lock"
* 移动 ready list "rdllist" to "txlist"
* releases "lock"
* calls ep_send_events_proc() with "txlist"
* acquires "lock"
* appends "ovflist" to "rdllist", resets "ovflist"
* appends "txlist" to "rdllist"
* if "rdllist" is non-empty, wake up "wq"
* release "lock"
* release "mtx" 
* if "rdllist" is non-empty, wake up "poll_wait"

ep_send_events_proc() iterates over a list of struct epitem objects:

* if the revent mask matches that requested:
  * copy the event
  * if EPOLLONESHOT is set, events &= (EPOLLONESHOT|EPOLLET)
  * else if EPOLLET is *not* set, add it back to the ready list

"mtx" is held across ep_scan_ready_list() and thus ep_send_events_proc(), even
if EPOLLET is in use (and thus we needn't add events back to the ready list).
See the comment in ep_send_events_proc() about locking out epoll_ctl() callers,
which are directed to "ovflist".

sys_epoll_ctl() validates input, and:

* acquires "mtx"
* searches for the fd using ep_find()
  * EPOLL_CTL_ADD with a present fd generates EEXIST
  * EPOLL_CTL_DEL and EPOLL_CTL_MOD with a non-present fd generates ENOENT
* call the appropriate one of ep_insert(), ep_remove(), ep_modify()
* releases "mtx"

The sys_epoll_pwait() extension is implemented in terms of sys_epoll_wait(),
wrapping the latter in appropriate sigprocmask() calls.

* 避免过多的　System calls 