    Principle 1: Exploit all cycles/bandwidth. Avoid blocking I/O and unnecessary evictions of cache, but prefetch into cache where appropriate (this applies to page caches just as much as processor caches or any other layer of the memory hierarchy). Be prepared to exploit multiple processing elements. Properly align data and avoid cache-aliasing effects. Use jumbo frames in appropriate scenarios and proactively warn on network degradation (e.g., half-duplex Ethernet due to failed link negotiation).
    Principle 2: Don't duplicate work. Avoid unnecessary copies, context switches, system calls and signals. Use double-buffering or ringbuffers, and calls like Linux's splice(2).
    Principle 3: Measure, measure, and measure again, preferably automatically. Hardware, software and networks will all surprise you. Become friends with your hardware's performance counters and tools like Oprofile, dtrace, ktrace, etc. Build explicit support for performance analysis into the application, especially domain-specific statistics.

#Event Queues and Threads

##Details of the Linux epoll(7) implementation
A struct eventpoll contains, among other things:

- a spinlock_t, "lock"
- a struct mutex, "mtx"
- two wait_queue_head_t's, "wq" and "poll_wait"

sys_epoll_wait() validates input and calls ep_poll() (fs/eventpoll.c). Its
core loop:

- acquires "lock"
- if there are no events ready or signals pending:
-- release "lock", sleep on "wq"
--- ep_poll_callback() is in charge of waking up "wq"
--- a timeout is also used (LONG_MAX if we passed timeout < 0)
-- reacquire "lock", loop to event/signal check
- release "lock"
- call ep_send_events()->ep_scan_ready_list() (see below)
- if we have events ready, pending signals, or the timeout has expired, return
- otherwise, loop

ep_send_events()->ep_scan_ready_list() is largely linear code:

 - acquires "mtx"
 - acquires "lock"
 - steals the ready list "rdllist" to "txlist"
 - releases "lock"
 - calls ep_send_events_proc() with "txlist"
 - acquires "lock"
 - appends "ovflist" to "rdllist", resets "ovflist"
 - appends "txlist" to "rdllist"
 - if "rdllist" is non-empty, wake up "wq"
 - release "lock"
 - release "mtx" 
 - if "rdllist" is non-empty, wake up "poll_wait"

ep_send_events_proc() iterates over a list of struct epitem objects:

 - if the revent mask matches that requested:
 -- copy the event
 -- if EPOLLONESHOT is set, events &= (EPOLLONESHOT|EPOLLET)
 -- else if EPOLLET is *not* set, add it back to the ready list

"mtx" is held across ep_scan_ready_list() and thus ep_send_events_proc(), even
if EPOLLET is in use (and thus we needn't add events back to the ready list).
See the comment in ep_send_events_proc() about locking out epoll_ctl() callers,
which are directed to "ovflist".

sys_epoll_ctl() validates input, and:

 - acquires "mtx"
 - searches for the fd using ep_find()
 -- EPOLL_CTL_ADD with a present fd generates EEXIST
 -- EPOLL_CTL_DEL and EPOLL_CTL_MOD with a non-present fd generates ENOENT
 - call the appropriate one of ep_insert(), ep_remove(), ep_modify()
 - releases "mtx"

The sys_epoll_pwait() extension is implemented in terms of sys_epoll_wait(),
wrapping the latter in appropriate sigprocmask() calls.

-----------------------------------------------------------------------------
3. Functional semantics
-----------------------------------------------------------------------------
Multiple threads can safely call into event retrieval -- the event queue will
not be corrupted.

Events are retrieved greedily. A thread will grab as many as it indicates
room for.

Multiple threads can safely modify an event queue, event if threads are
retrieving from it -- the event queue will not be corrupted, and threads
sleeping in retrieval will be woken up for new ready events.

Level-triggered events cannot be safely processed by multiple threads
without some kind of extra locking, except in circumstances so rare
(and likely trivial) that they're barely worth accounting for:

 - LT event 1 becomes ready
 - Thread A retrieves LT event 1 instance 1
 - Thread B retrieves LT event 1 instance 2
 - Thread A enters LT event 1 handler
 - Thread B enters LT event 1 handler
 - either contention or race follows

Edge-triggered, auto-renewed events cannot be safely processed by multiple
threads without some kind of extra locking, except when file descriptor
availability suffices to synchronize the event handling. An example of this
would be a callback which simply loops on accept(2) until EAGAIN/EWOULDBLOCK,
possibly performing some MT-safe processing on each new file descriptor (adding
the new fd to the event queue would be fine, since we've already shown event
queue operations to be MT-safe. Otherwise, for instance if reading data from
the fd into an associated buffer:

 - ET event 1 becomes ready
 - Thread A retrieves ET event 1 instance 1
 - Thread B sleeps
 - Thread A enters ET event 1 handler, read(2)ing all available data
 - ET event 1 is automatically rearmed
 - Thread B wakes up, retrieves ET event 1 instance 2
 - Thread B enters ET event 1 handler, read(2)ing all available data
 - ET event 1 is automatically rearmed
 - Thread A enters ET event 1 critical section
 - Thread B enters ET event 1 critical section
 - either contention or race follows

Edge-triggered, one-shot events can be safely processed by multiple threads
so long as they're rearmed only at the end of the handler. They require
more event queue modifications than other solutions (assuming that we always
handle as much data as is provided in any callback -- the edge-triggering
requirement. Callbacks which depend on level-triggered semantics might
perform explicit disabling calls from time to time).

The choice of queue-sharing schemes affects the mechanics of library cleanup
and termination. See the document doc/termination for more details.

-----------------------------------------------------------------------------
4. Performance semantics
-----------------------------------------------------------------------------
Edge-triggered semantics allow a more efficient internal implementation than
level-triggered semantics.

Modifications and (running) retrievals are serialized by the kernel for
each event queue. As the number of threads per event queue rises, the
expected volume of waiting also increases. A complex feedback system is
likely to emerge if modification wait time ever becomes non-negligible.
This does not apply to retrieval wait time; if events are being handled more
quickly than they take to retrieve, we are bounded by retrieval time (by
the pigeonhole principle: in the large, a thread will always be
actively retrieving though we will sometimes empty the work queue).

System calls are, as ever, to be avoided when prudent. Using larger event
receipt buffers minimizes system calls into the event queueing framework,
but can lead to a poor short-term distribution of work and poor parallelism.

Batching of changes cannot be accomplished with the current Linux epoll_ctl()
implementation, despite our internal kevent() emulation affecting such an API
(it actually iterates over the list, invoking epoll_ctl() in turn). Until this
is changed, batching of changes is a non-starter on Linux. FreeBSD does provide
the interface; it remains to be determined whether this is worth the latency
penalties. Multithreading makes this less of an issue, but also leads to more
possible contention on change registration.

-----------------------------------------------------------------------------
5. Deductions
-----------------------------------------------------------------------------
Single-threaded retrieval of events into a userspace queue shared among
multiple threads doesn't solve any correctness problems, or meaningfully
reduce locking (unless, possibly, when threads are retrieving less events at
one time than is optimal based on load). The actual work of retrieval (as
opposed to sleeping) is still serialized by the kernel, and must be
performed.

Exclusive event queues per thread are the simplest solution, eliminating
all contention and even synchronization concerns at the event engine level.
This might not scale (it's doubtful that anyone's seriously used thousands
of event queues in a single app), and more importantly it leads to a lack of
balance; threads will only be as balanced as can be predicted during event
source distribution.

It is of course feasible to serve many threads using a scheme where only
one actually receives events for a given event queue at any time (ie, we
don't rely on epoll_wait() being safe for multiple callers), dumping them
into a common userspace buffer managed via mutexes and condition variables.
We'd not be exploiting the kernel locking to provide our own synchronization
anymore, but only a contention-free mutex acquisition and release have been
"added". Furthermore, this removes the event queue system call from each
thread; they now only need perform userspace locking (with, of course, a
system call to waitqueues should the lock be contended, but we presumed
that this was rare. What we want to avoid is a queue *modification*,
generated during the event loop, blocking against a retrieval operation
(preferably without adding a batching step ala the kqueue() API, since this
can add latency. Remember, retrievals contending means either that load is
negligible, or that we're not retrieving nearly enough events at a time. Using
TCP-like windows would solve that problem for multiple event threads).

-----------------------------------------------------------------------------
6. Signals
-----------------------------------------------------------------------------
All other threads in the application must have libtorque-registered signals
masked, or else libtorque might not be able to receive relevant signals. Upon
creation, libtorque threads mask all signals. When a signal is registered with
libtorque, a common handler is installed for that signal, demultiplexing into
the signal event table. Note that since handlers are process-wide, failure to
mask appropriate signals in application threads might result in libtorque
signals being handled in their contexts.

Signals and POSIX threads have a historically complicated relationship (see
the document doc/termination for more details about threads). When
EVFILT_SIGNAL (FreeBSD) or signalfd's (Linux) are used, they can be cleanly
handled within our standard event semantics. On older Linux kernels, however,
signalfds are not available. There, we must handle signals via the standard
POSIX signal delivery mechanism, adding a new complexity to the event queues.
Note from {2} that delivery of a signal results in immediate exit from
epoll_wait() and release of associated locks, thus the multiple threads of an
event queue will automatically enjoy distribution of events (this is
independent of any sa_mask value or SA_NODEFER flag provided to sigaction(2),
since each thread has its own signal mask).

If libtorque threads unblocked its signals throughout their lifetimes, those
signals could be delivered during event handling callbacks. Synchronization
requires that only one thread be handling a given event source at a time, so
repeated delivery of a signal during such a callback would both block the
event source's progress in that thread (it's busy handling the signal) and
preclude (without major contortions) any other thread handling the event.

POSIX signal handlers' action is heavily restricted. pthread_mutex_lock() is
not safe to call within a signal handler, nor is the C library's dynamic
allocation interface. We certainly cannot directly invoke registered callbacks
from the signal handler. The standard signal-handling idioms are:

--event: post to a global semaphore having type sig_atomic_t
--exception: clean up transprocess resources (files, etc), re-raise signal

Note that progress under the event idiom requires some thread to examine the
semaphore, and handle the event in its context. GNUlibc specifies that handling
a signal interrupts all event multiplexing interfaces with EINTR, regardless of
the use of SA_RESTART, so the semaphore check must live no further away from
the epoll_wait() call than whatever scope is defined by the loop on EINTR.

A Garden of Interfaces

We all know doddering old read(2) and write(2) (which can't, by the way, be portably used with shared memory). But what about...

    readv(2), writev(2) (FreeBSD's sendfile(2) has a struct iov handily attached, perfect for eg the Chunked transfer-encoding)
    splice(2), vmsplice(2) and tee(2) on Linux since version 2.6.17
        (When the first page of results for your interface centers largely on exploits, might it be time to reconsider your design assumptions?)
    sendfile(2) (with charmingly different interfaces on FreeBSD and Linux)
        On Linux since 2.6.2x (FIXME get a link), sendfile(2) is implemented in terms of splice(2)
    aio_ and friends for aysnchronous i/o
    mmap(2) and an entire associated bag of tricks (FIXME detail)
        most uses of mincore(2) and madvise(2) are questionable at best and useless at likely. FIXME defend
        broad use of mlock(2) as a performance hack is not even really questionable, just a bad idea FIXME defend
        use of large pages is highly recommended for any large, non-sparse maps FIXME explain
        mremap(2) and remap_file_pages(2) on Linux can be used effectively at times
        There's nothing wrong with MAP_FIXED so long as you've already allocated the region before (see caveats...)

    "The linux mremap() is an idiotic system call. Just unmap the file and re-mmap it. There are a thousand ways to do it, which is why linux's mremap() syscall is stupid." - Matthew Dillon

    "I claim that Mach people (and apparently FreeBSD) are incompetent idiots. Playing games with VM is bad. memory copies are _also_ bad, but quite frankly, memory copies often have _less_ downside than VM games, and bigger caches will only continue to drive that point home." - Linus Torvalds

    User-space networking stacks: The Return of Mach!
        Linux has long had zero-copy PF_PACKET RX; get ready for zero-copy TX (using the same PACKET_MMAP interface)
        I'm actively using PACKET_TX_RING in Omphalos as of June 2011; it works magnificently.
    "Zero-copy" gets banded about a good bit; be sure you're aware of hardware limitations (see FreeBSD's zero_copy(9), for instance)

The Full Monty: A Theory of UNIX Servers

We must mix and match:

    Many event sources, of multiple types and possibly various triggering mechanisms (edge- vs level-triggered):
        Socket descriptors, pipes
        File descriptors referring to actual files (these usually have different blocking semantics)
        Signals, perhaps being used for asynchronous I/O with descriptors (signalfd(2) on Linux unifies these with socket descriptors; kqueue supports EVFILT_SIGNAL events)
        Timers (timerfd(2) on Linux unifies these with socket descriptors; kqueue supports EVFILT_TIMER events)
        Condition variables and/or mutexes becoming available
        Filesystem events (inotify(7) on Linux, EVFILT_VNODE with kqueue)
        Networking events (netlink(7) (PF_NETLINK) sockets on Linux, EVFILT_NETDEV with kqueue)
    One or more event notifiers (epoll or kqueue fd)
    One or more event vectors, into which notifiers dump events
        kqueue supports vectorized registration of event changes, generalizing the issue
    Threads -- one event notifier per? one shared event notifier with one event vector per? one shared event notifier feeding one shared event vector? work-stealing/handoff?
        It is doubtful (but not, AFAIK, proven impossible) that one scheduling/sharing solution is optimal for all workloads
        The Flash web server dynamically spawns and culls helper threads for high-latency I/O operations
        The contest is between the costs of demultiplexing asynchronous event notifications vs managing threads
            My opinion: if fast async notifications can be distributed across threads, one thread per processing element always suffices
            Other opinions exist, centered around communication bottlenecks

    "Thread scheduling provides a facility for juggling between clients without further programming; if it is too expensive, the application may benefit from doing the juggling itself. Effectively, the application must implement its own internal scheduler that juggles the state of each client." - George Varghese, Network Algorithmics

DoS Prevention or, Maximizing Useful Service

    TCP SYN -- to Syncookie or nay? The "half-open session" isn't nearly as meaningful or important a concept on modern networking stacks as it was in 2000.
        Long-fat-pipe options, fewer MSS values, etc...but recent work (in Linux, at least) has improved them (my gut feeling: nay)
    Various attacks like slowloris, TCPPersist as written up in Phrack 0x0d-0x42-0x09, Outpost24 etc...
    What are the winning feedbacks? fractals and queueing theory, oh my! fixme detail

The Little Things
Hardware Esoterica

    Direct Cache Access must be supported by NICs, northbridge chipset, OS and microarchitecture
    IOMMU / I/OAT
    Checksum offloading / TSO / LRO / Frame descriptors
        Use ethtool on Linux to configure NICs (try ethtool -g, -k and -c)
    PCI shared bus/bus-mastering, PCIe slots/lanes (channel grouping), PCI-X, MSI

    "Many times, but not every time, a network frontend processor is likely to be an overly complex solution to the wrong part of the problem. It is possibly an expedient short-term measure (and there's certainly a place in the world for those), but as a long-term architectural approach, the commoditization of processor cores makes specialized hardware very difficult to justify." - Mike O'Dell, "Network Front-end Processors, Yet Again"

    What about robustness in the face of hardware failure? Actual hardware interfaces (MCE, IPMI, CPU and memory blacklisting) ought mainly be the domain of the operating system, but the effects will be felt by the application. If a processing unit is removed, are compute-bound threads pruned?

Operating System Esoterica

    The Linux networking stack is a boss hawg and a half. Check out the Linux Advanced Routing and Traffic Control (LARTC) HOWTO for details ad nauseam
    See my TCP page -- auto-tuning is pretty much to be assumed (and best not subverted) in recent Linux/FreeBSD
    When extending MAP_NOSYNC maps on FreeBSD, be sure to write(2) in zeros, rather than merely ftruncating (see the man page's warning)
    Linux enforces a systemwide limit on LVMAs (maps): /proc/sys/vm/max_map_count

Tuning for the Network

    All hosts ought employ the RFC 1323 options (see Syncookies regarding contraindications there)
    Avoid fragmentation: datagram services (UDP, DCCP) ought ensure they're not exceeding PMTUs
    LAN services ought consider jumbo frames.
    There is little point in setting IPv4 TOS bits (RFC 791, RFC 1349); they've been superseded as DiffServ/ECN (RFC 3168)
        IPv6 does away with this entire concept, using flow labels and letting the router decide

Power Consumption

Less power consumed means reduced operating cost and less waste heat, prolonging component life.

    Using on-demand CPU throttling (ACPI P-states, voltage reduction) is a no-brainer, but requires dynamic control to be effective.
        Be sure it's enabled in your OS and your BIOS; more info here
    Sleep states (architectural changes) are useful outside environments pairing low-latency requirements with sporadic traffic
        Even aggressive power-saving ACPI C-states wake up in usec
    Don't wake up disks when it's not necessary; try using tmpfs or async for transient logging, and don't log MARK entries
        If your app doesn't use disk directly, consider PXE booting and network-based logging
    Avoid periodic taskmastering and timers where available, using event-driven notification (more effective anyway!)
    Use as few processing elements as completely as possible, so that CPUs and caches can be powered down
        This also applies, of course, to machines in a cluster