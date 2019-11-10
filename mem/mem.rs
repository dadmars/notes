cpu cache

cache 使用的电路结果会直接得到

dram

dram 使用的电路各含有电容，在达到稳定状态时有一个充电和放电的过程。这要消耗掉一些时间。
地址线为了节约引脚资源，cell 按行，列进行排列。寻址时分为两个阶段，先选择行，后选择列。这之间又要花费时间
电容每隔一段时间要重新充电（30ms），在此期间是不能进行访问的。

cache 与操作系统和用户无关，管理成本太大。cache 由 cpu 管理。只是内存一部分地址的临时 copy 。cache 也可分为两部分，代码cache，数据cache

cache 使用提前载入的方式来提高性能。

每一个 cache entry 都被其所缓冲的内存地址，进行了 tag。当访问内存地址时，首先在cache中查找 tag。

访问一行数据要比一个字节快的多（在寻址时不用重新选择行）。所以 cache entry 的单位是行（几个连续的字节）

当访问一个地址时，整个 cache line 被载入一级cache。The memory address for each cache line is computed by masking the address value according to the cache line size.

cache line 被修改但还没有写回内存，设为 dirty。如果写回，cleared

cache 的实现方式：把 cache line 分成组(set)，计算 tag，得到分组，然后计算偏移得到组内的 cache line

tlb cache: 虚拟地址转化为物理地址的 cache

cache 优化: 
    write-back
    write-combining
    uncacheable

多个cup，一个cpu访问另一个cpu的cache line，要进行通信，使用协议: MESI（modify, exclusive, shared invalid）
modify: 本地cpu修改一个cache line，这也表明，此cache line只存在于本地cpu中
exclusive:cache line 没有被修改，但是不能被其它cup 访问
shared:cache line 没有被修改，并且正在被其它 cpu 共享
invalid:cache line 没有被使用

执行代码也会被 cache
