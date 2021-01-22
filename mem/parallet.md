并发程序的目标：
性能
生产力
通用性

并行程序只是性能优化的一种方式。不一定是最好的方式。

生产力主要是因为硬件性能提高并且价格下降。

通用性与生产力和性能之间不能同时满足

从底层到应用层，通用性和性能越往下越重要。越往上生产力越重要。

目前硬件的升级只是提供多个cpu core，而对单个core的运算能力提高不是很大。

并行计算对性能的提升最大幅度受限于cpu的数量。

并行化是否能提升性能与程序设计有关，如：单线程如果是要等IO，并行化后可能每个cup都要等io。单线程如果读一个大文件，并行化后如果都要读同一个大文件，可能更加慢。

work partitioning:
    partitioning 造成性能提升，也会提高复杂性，如：会产生全局的事件和错误处理，各进程进行通信。如果一个cpu有太多的线程，会提升 cache missing 的几率

resource partitioning 并且复制:
    程序不同，策略也不同。如把经常写的数据和只读数据进行分离。数据分割可以是跨机器，跨存储设备，NUMA 节点，CPU，pages, cache line ...

Parallel access control
    同步机制。对资源的访问分为本地资源和远程资源，远程资源一般通过消息(或协议)进行访问。

与硬件交互:

partitioning 主要是为了减少各部分的通信。各部分能单独进行处理，而不用互相依赖。

cpu pipline

atomic operations 可能会造成 cpu pipline 的延迟或 flush. 此时 cache line 只能被当前cpu占有，其它cpu只能等。

memory barriers: 在其间的指令严格按顺序执行。会降低性能。

cache missing : 会降低性能。一个cpu要访问的内存，刚刚被另一个cup访问，被其cache。此时要进行cpu这间的通信。

compare-and-swap(CAS): 发生 cache missing 后，性能慢10倍左右

CAS 性能比 lock-unlock 快2倍，因为后者有两个 automic 操作

硬件的优化：
 大的 cacheline , 大的 cache，经常读不常写的数据复制到每个cpu的cache里

 减少任何形式的通信
 所有的共享数据都为 read-mostly

 lock:
 普通的lock:
    pthread_mutex_lock()
    pthread_mutex_unlock()

 reader-writer locking:
    pthread_rwlock_rdlock()
    pthread_rwlock_wrlock()
    pthread_rwlock_unlock()

    每次读都要访问锁，所以不一定快。

 atomic (gcc)
    compare-and-swap

 Per-Thread Variables
    pthread_key_create()
    pthread_key_delete()
    pthread_setspecific()
    pthread_getspecific()

编译器优化对并行造成的影响:

    load tearing: 多条指令读取一个变量，变量可能中间发生变化。
    store tearing: 多条指令保存一个变量，变量可能中间发生变化。
    load fusing: 如多次读取同一变量，如果没有写入，编译器有可能只读取一次，把后面的读取都优化掉了。
    store fusing: 多次写入同一变量，如果没有读取，编译器有可能只写最后一次，把前面的写入都优化掉了。
    code reordering: 
    invented loads: 将一些临时变量优化掉了。可能会造成 cache missing
    invented stores: 将一些临时变量优化掉了。
    
进程间通信或消息传递优于共享内存。

count:
    访问一个全局变量：加1, 读取
    使用 automic ，随着cpu和线程的增加，性能会下降。因为多个cpu这间要进行同步。

    Statistical Counters
        使用 per-Thread 变量，每个线程独立加1, 然后额外的线程将每个线程的计数相加。这样减少了cpu这间的同步，但如果线程数比较多，额外的相加线程也会有性能问题。
        可以弱化一致性，减少对每个线程计数相加的频率，来提高性能。
        也可使用gcd的线程存储(__thread)，这样在对每个线程的计数相加时，就不用调用per_thread(counter, t)。但是整个过程要对数据加锁。

Limit Counter
    把一个globecount 拆分到每个线程。每个线程有一个计数器和一个最大值。对globecount操作时要加锁。

    精确计算：
        使用 automic , 合并变量（一个16位的高8位为计数器，低8位为最大值）

        使用信号可避免使用 automic，提高性能。通过向一个额外线程发信号，来计算总数，但这需要设计状态。

1. 分割提升性能
2. 对部分进行特定处理
3. 弱一致性
4. 代码级优化

