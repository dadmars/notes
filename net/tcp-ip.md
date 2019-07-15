# 网络

主机上装有网卡，每个网卡都有唯一的硬件地址（MAC地址），不同的主机通过网线连接在一起，组成局域网。跟据网络硬件和连接方式的不同，有以太网等不同类型的网络。

不同的局域网，通过路由器连接在一起。路由器有多个网卡，每个网卡都有不同的MAC地址。每个网卡连接一个不同的局域网。数据包从一个网卡进入，从另一个网卡发送出去，从而实现两个网络的连通。

因为数据是通过网卡进行传输的，所以所有的网卡都要有一个唯一的地址，这样数据才能正确交互。这个地址叫做IP地址。为了保证IP地址全球唯一，只能通过专门的机构进行分配。IP地址是结构化的地址，分成网络号（专门机构分配），子网号，主机号。子网号和主机号由子网掩码决定。这一机制，使得路由器外部无法直接访问内部网络。

由于硬件和传输介质的限制，传输的数据包都有长度的限制，即网络最大传输单元，称为MTU。当IP包大于MTU，要进行分片。当IP路由选择一条传输线路后，这条路径的最小MTU，称为 path MTU。

# 链路层 ISO中为L2

为方便，处理不同的硬件和介质的问题，都放到同一层，这就是链路层。如各种网卡驱动。链路层只能处理直接相连的主机的数据传输问题。

数据在介质中传输，要有一定的格式，这个格式称为封装。以太网的数据格式中包含发送方和接收方的地址。这是因为，数据发送到线路上后，所有局域网中的网卡都能收到，从数据中得到目的地址后，与自己的地址比较，才能决定这些数据是不是发给自己的。

本机支持环回接口，IP为127.0.0.1，用来支持客户端和服务端在同一台主机上，并且两者可通过TCP/IP进行通信。可以把它当作虚拟网卡。

到现在为止，一共有两种地址，IP地址和MAC地址。要想数据能够传输，从IP地址转化为MAC地址，使用ARP协议。这是一个广播协议，一台主机向局域网广播要查找的目的IP地址，目标主机收到此广播后，向发送方回应自己的IP地址和MAC地址。为提高效率，每台主机包含ARP cache。一个对应项存在的时间为20分钟。

代理ARP，路由器连接两个网络，一个网络的主机向另一个网络发送数据时，路由器对ARP请求进行回应，这样发向另一个网络的数据都会发送到路由器，然后此路由器查找路由表进行转发。

如果主机通过ARP查找自己的IP，如果有收到回应，说明有IP地址冲突。也可用于主机更换网卡，向全网公布自己新的MAC地址。

# 网络层 ISO中为L3

网络层把数据从一个IP地址传输到另一个IP地址。这两个IP地址可能在同一台机器，可能在同一个局域网，也可能在不同的网络，中间跨过多个路由器。IP协议负责找到两个IP地址之间的传输线路，也叫作路由。

IP不提供可靠性，这样可以简化设计，使其专注于网络传输。

网络字节序采用 big endian。4个字节32位进行发送时，按0-7位，8-15位，16-23位，24-32位的顺序进行传输。

由于IP包可能分片，所以要有唯一ID号标识每一个IP包。在IP头部有一字段标识此数据包不能分片(DF)，如果要分片，返回一个ICMP不能分片错误。

为防IP包在网络中无限的存在下去，存在TTL，每经过一个路由器，此值减1,为0后，被丢弃，并发送一个ICMP到发送方。

# 最小字节

主机必须保证可以接收576字节的IP包。UDP一般将用户数据限制在512字节。

# checksum

IP的checksum 只校验头，不校验其它数据。因为其它协议会对自己数据进行较验。算法为：把所有数据分成16位，分别相加，取补数，然后保存在checksum字段。校验时，同样，结果全为1,校验成功。此时不发送ICMP，而是等待重传。

UPD的checksum包含头部和数据。包含前面12字节的pseudo-header

TCP的checksum包含头部和数据。包含前面12字节的pseudo-header

# IP路由

路由daemo决定在路由表中放入何种路由。IP路由决定如何在路由表中查找。

路由表大约30秒更新一次，也会被 ICMP "redict" 更新。

每个路由表项包含下面信息：

* 目标IP地址
* 下一站路由IP地址或直接相连的网络的IP地址
* flag
  * G 此路由为间接路由，不是直接路由。没有此标记，说明目标地址和此路由器是直接相连的。
  * H 设置此标记，说明是一个主机地址。没有此标记，说明只包含网络地址和子网地址，不包含主机地址。
  * D M 当路由器更改路由，发送重定向的ICMP到发送方，发送方更新路由表, D新建， M为修改
* 网卡
* 一些信息，在拥塞避免等算法中使用

路由表：
* 不联网，只有一个环回接口
* 局域网，环回接口和局域网
* 连接到路由器，加一个default，指向此路由器
* 有多个路由器

接收到一个数据包时，如果目标地址与本机相同，或是一个广播地址，提交到上层。否则，如果本机不是路由器，丢弃之。

如果本机是路由器，则查找路由表：

* 先匹配目标地址，按完全匹配
* 匹配网络号（子网掩码）。
* default。

如果以上都失败，发送“host unreachable”到发送方。

主机的路由表可以手工配置，当设置网卡时，会自动加入一个路由表项。

当主机启动后，广播或多播一个ICMP路由评述请求信息，一个或多个路由器响应一个ICMP应答。主机收到后更新自己的路由表。路由器每隔450或600秒，广播或多播一个ICMP路由信息。收到此信息的主机更新其路由表。此信息包含生存期字段（30分钟），当路由器的一个接口关闭，生存期字段为0.如果有多个路由器，可以设置每个路由器的优先级。

路由器之间交换路由信息通过RIP等动态路由协议来进行。RIP使用UDP。当路由器启动，广播一条信息，向相连的路由器请求路由表。收到后更新自己的路由表。后面定时发送自己的路由表（30秒）。

# 传输层UDP

当接收方认为发送方速度太快，有可能发送 ICMP Source Quench 错误到发送方。

广播和多播只能用于UDP。因为TCP是面向连接的。

# 传输层TCP

可靠性：
* 将字节流分成一块块的数据，通过IP层向目标传输。
* 对每一个字节进行编号。并且对每一个接收的字节进行确认
* 建立连接时，交换MSS。MSS是为了防止分片，但是如果两个主机要经过多个中间路由器，起作用的是path MTU
* window size(rwnd)来防止发送方太快，来不及接收。在发送方收到ACK时，同时会收到对方的window size。称为滑动窗口
* 超时没收到ACK，重传数据包
* server进行确认时，会等一下，将确认和数据一同发送，减少流量，一般等待时间为最大200MS
* 对头部和数据计算checksum
* 处理乱序和重复的问题
* 拥塞控制和慢启动

建立连接
* client发送SYN，通知server自己的初始序列号（ISN），MSS，窗口大小
* server对client的SYN进行确认。server发送SYN，包含自己的初始序列号（ISN），MSS，窗口大小。如果不包含MSS，默认为536字节。
* client对server的SYN进行确认

关闭连接
* client发送包含FIN标记的数据
* server收到FIN后，通知用户程序end-of-file，向client发送确认
* server用户程序处理完后，发送FIN到client
* client对FIN进行确认。此时client处在2MSL等待状态(TIME_WAIT)，有可能server没收到确认，进行超时重发.当处在TIME_WAIT状态时，所有因为延时到达的数据都被丢弃。在server端，如果终止服务然后立即重启，由于处于TIME_WAIT状态，不能重用端口号，无法建立连接，所以无法重启。

通过双方的IP地址和端口号来标识一个连接。一个IP地址加端口号称为socket。TCP头部包含端口号，标识两边的用户程序；包含checksum，进行数据校验；包含标志位，如SYN建立连接，FIN终止连接，ACK确认；包含序列号（数据的偏移）和长度；包含ACK序列号；还有一些选项，如MSS

quiet time: 2MSL的等待可以防止延时数据造成的冲突，但如果在2MSL中主机崩溃并重启，延时数据将会被当作新的数据（IP地址和端口号和新连接相同）。所以，主机重启后在MSL时间内不能建立TCP连接。

FIN_WAIT_2: client发送FIN，并收到ACK，进行FIN_WAIN_2状态。如果收到server的FIN，进入TIME_WAIT状态。这意味着可能一直处在此状态。为防止此问题，引入超时管理。

RESET： 数据到来但是发现连接是错误的，发送RESET。如，建立连接，发现server对应端口号没有帧听。

Nagle 算法：
* 当有大量小数据传输
* 发送一个数据包，等待ACK，此时不继续发送。当收到ACK，将积累下来的数据包一起发送。
* 在server会发生delayed ACK
* socket API 使用TCP_NODELAY可以禁止此算法

delyed ACK:
* 接收的数据被放入IP的输入队列中
* delyed ACK的定时器被设置
* 定时器超时，发送ACK
* delyed ACK的定时器被重新设置

window flow control:
* 接收方有一个接收buffer，大小为window size
* 每一个ACK都会发送剩下buffer的大小（window size）
* 发送方只能发送小于window size大小的数据
* 当windows size为0时，发送方停止发送

slow start:
* 引入cwnd（拥塞窗口），初始化为一个数据包(现在一般为10个数据包)。发送方只能发送min( cwnd, window)的数据包
* 在开始传输时，只发送一个数据包（cwnd为1个数据段）
* 当收到ACK时，发送2个数据包（cwnd为2个数据段）
* 当2个数据包都收到ACK时，发送4个数据包（cwnd为4个数据段）
* 如此一直达到拥塞状态
* cwnd是发送方对流量的控制，window是接收方对流量的控制（接收方的buffer）

Slow-Start Restart：
* sysctl net.ipv4.tcp_slow_start_after_idle
* sysctl -w net.ipv4.tcp_slow_start_after_idle=0
* 当TCP连接空闲时，将cwdn设为默认值，从而避免拥塞
* 会有性能问题，服务器端建议关闭此选项

拥塞避免算法：
* 与slow start同时使用
* 引入一个阀值ssthresh
* 初始化, cwnd为一个数据包。ssthresh为65535
* 当拥塞发生(重传超时或连续收到相同ACK) ssthresh = max(min(cwnd, window size) / 2, 2 segments)。如果拥塞是因为重传超时引发的，将cwnd设为1个segment(开始slow start)
* 开始slow start，当cwn超过ssthresh时，使用拥塞避免，每收到一个ACK，cwnd增长最多1个segments
* 拥塞避免，cwnd += segsize * segsize / cwnd + segsize/8 (segsize由MSS决定，如果没有MSS，默认为536字节)

快速重传，和快速恢复算法
* client连续收到3个相同的ACK，开始重传而不等待超时。
* ssthresh = cwnd/2， cwnd = ssthresh + 3×segments (当发生重传时，流量会减少到一半)
* 开始重传
* 每多收到一个重复ACK，cwnd += 1*segments(快速恢复算法)
* 新的ACK到达（说明重传结束），cwnd=ssthresh
* 此时开始使用拥塞避免算法(cwnd >= ssthresh)。
* 当重传数据时，可以传输更大的数据（包含要重传的数据）

TCP的4个定时器：
* 重传定时器，时间长度与RTT相关，并且会变化。RTT有专门的公式进行计算。
* 当接收方window size为0时，定时探测window size大小
* keepalive 定时器
* 2MSL定时器

Karn's Algorithm
* 解决重传和老数据冲突问题

传输过程中对ICMP错误的处理
* source quench: cwnd=1，开始slow start
* host/network unreachable: 被忽略

window scale option
* 默认打开
* sysctl net.ipv4.tcp_window_scaling
* sysctl -w net.ipv4.tcp_window_scaling=1
* 将window size大小从16位增加到32位，增加可发送数据的上限
* 只能在SYN时设置，必须双方都支持

# 性能

* 延时: 从源发送一个包，到目的端收到此包经过的时间
* 带宽: 逻辑或物理路径的最大通过率

## 性能提高

主干网速度很高，主要延时来自网络边缘，相关因素：服务商的能力，中间路由器，费用等

* 减少RTT
* 数据靠近用户（CDN）
* 程序减少延时：cache；提前获取；等技术

## TCP性能

* 由于建立连接要花费很多时间（3次握手），所以要进行连接重用。重用连接可以避免3次握手和slow start,性能可能提高2倍

TCP性能的核心
* 3次握手建立连接产生的延迟
* 新的连接都要slow start
* flow control和congestion control在整个传输过程都起作用
* TCP流量受当前的cwnd控制

服务器最佳实践：
* Increasing TCP’s Initial Congestion Window
* Slow-Start Restart: 禁用此功能，提高长连接性能
* Window Scaling: 开启此功能，提高流量
* TCP Fast Open: 在发送SYN时发送用户数据。这是一个新的优化，要client和server同时支持

查看socker设置

```bash
ss --options --extended --memory --processes --info
```

性能Checklist
* Upgrade server kernel to latest version 
* Ensure that cwnd size is set to 10.
* Disable slow-start after idle.
* Ensure that window scaling is enabled.
* Eliminate redundant data transfers.
* Compress transferred data.
* Position servers closer to the user to reduce roundtrip times.
* Reuse established TCP connections whenever possible.
