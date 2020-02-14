```c
///// client
int sockfd = socket(AF_INET, SOCK_STREAM, 0); // create a Internet stream socket.
if (sockfd < 0) {
    err_sys("socket error");
}

struct sockaddr_in servaddr;    // Internet socket address structure, 此结构的 IP 地址和端品号有特定的格式。
bzero(&servaddr, sizeof(sockaddr_in));

servaddr.sin_family = AF_INET;
servaddr.sin_port = htons(13);  // port number( htons: host to network short )
if (inet_pton(AF_INET, "168.192.3.7", &servaddr.sin_port) <= 0) {   // IP address( inet_pton: presentation to numeric )
    err_quit("error");
}

if (connect(socked, (sockaddr_in*)&servaddr, sizeof(servaddr)) < 0) {
    err_sys("socket error");
}

char recvline[MAXLINE - 1];
// TCP 是面向字节流的协议，没有记录边界，所以要循环读入
while (1) {
    // read 返回0表示对方关闭连接，< 0, 出错
    int n = read(socked, recvline, MAXLINE);
    if (n == 0) {
        break;
    }
    else if (n < 0) {
        err_sys("error");
    }

    recvline[n] = 0;
    if (fputs(receline, stdout) == EOF) {
        err_sys("error");
    }

    exit(0);
}

///// server
int listened = Socket(AF_INET, SOCK_STREAM, 0);

struct sockaddr_in servaddr;
bzero(&servaddr, sizeof(sockaddr_in));

servaddr.sin_family = AF_INET;
servaddr.sin_addr.s_addr = htonl(INADDR_ANY);   // 接受所有的IP地址
servaddr.sin_port = htons(13);                  // 服务的端口号为 13

Bind(listened, (SA*)&servaddr, sozeof(servaddr));   // 服务的端口号绑定到 socket

Listen(listened, 500);  // 将socket 转换为 listening socket, 内核为此socket建立的连接队列大小500

char buffer[MAXLINE];
while (1) {
    int connfd = Accept(listened, (SA*)NULL， NULL);    // sleep，如果有连接到来并接受(完成三次握手)，返回

    int ticks = time(NULL);
    snprintf(buffer, sizeof(buffer), "%.24s\r\n", ctime(&ticks));

    Write(connfd, buff, strlen(buff));

    Close(connfd);  // send FIN to client
}
```
TCP 没有记录边界，应用程序自己解决边界的问题

如果多个连接过来，内核 queue 它们，逐个返回

raw socket 可以直接使用 ip 协议

三次握手：
 client: 调用 connect 发送SYN，然后阻塞。收到ACK后返回
 server: 调用 accept 阻塞。收到SYN，发送ACK和自己的SYN，收到client 的ACK后返回。

当服务器有多个网卡（IP地址），listen socket 要么只帧听一个地址，要么帧听所有地址，不能指定一个帧听地址列表。
帧听所有地址: servaddr.sin_addr.s_addr = htonl(INADDR_ANY);

由于firewall会丢掉ICMP, 通过 ip4 df 标志位找到 path MTU 不在有效。

设置 socket 的 send buffer: SO_SNDBUF
调用 write, 阻塞，当程序的 buffer 完全 copy 到 socket send buffer 后返回。此时，无法得知数据是否发送到对方，也无法得知对方是否收到数据。

udp 当程序数据大于 send buffer, 返回EMSGSIZE

提供的服务 /etc/services

ipv4 地址数据结构：

struct in_addr {
    in_addr_t s_addr;   /* 32-bit IPv4 address, network byte ordered */
};

struct sockaddr_in {
    uint8_t sin_len;            // length of structure (16)
    sa_family_t sin_family;     // AF_INET
    in_port_t sin_port;         // 16-bit TCP or UDP port number, network byte ordered
    struct in_addr sin_addr;    // 32-bit IPv4 address, network byte ordered
    char sin_zero[8];           // unused
};

下面函数把此结构从用户层 copy 到内核：
    bind , connect , sendto , sendmsg

下面函数把此结构从内核 copy 到用户层：
    accept , recvfrom , recvmsg , getpeername , getsockname

机器是 big-endian or little-endian
```rust
fn main() {
    union T {
        s: u16,
        b: [u8; 2]
    }

    let t = T{s: 0x0102};

    unsafe {
        if t.b[0] == 2 && t.b[1] == 1 {
            println!("little-endian, {},{}", t.b[0], t.b[1]);
        }
        else if t.b[0] == 1 && t.b[1] == 2 {
            println!("big-endian, {},{}", t.b[0], t.b[1]);
        }
        else {
            println!("unknown");
        }
    }
}
```

connect 三次握手完成或出错时返回
    出错：
        没收到 SYN 的 ACK, 返回 ETIMEDOUT. 重试： 6 秒后， 24 秒后， 最终 75 秒后返回错误，timeou 可以设置
        收到的是 RST, 返回 ECONNREFUSED。对方没有服务
            发送 RST:
                SYN 到达后，发现 port 没有服务在帧听
                TCP 要 abort 一个连接
                收到一个不属于此连接的数据报
        发送 SYN 收到的是从中间路由器返回的 ICMP "destination unreachable"。 内核保存此消息，并重试 75 秒后返回 EHOSTUNREACH or ENETUNREACH。如果是因为本地路由表没有目的地，直接返回。

connect 出错后，socket 失效，必须 closed

TODO(p135): daytimetcpcli ip port
    查看错误：
        远程 port 错误
        本地 port 错误
        远程 ip 错误
    
                
bind: 绑定一个本地地址给 socket
    如果服务没有绑定IP地址，内核使用 client 的目标地址作为其源地址(此时称 server 的源地址为 wildcard: htonl (INADDR_ANY))

listen(a, backlog): listening socket 有两个队列，两个队列的长度不能超过backlog
    没有完成三次握手的队列，socket 状态为 SYN_RCVD
    完成三次握手的队列，socket 状态为 ESTABLISHED
    当队列为空，服务进程 sleep
    正常情况下 socket 在incomplete队列中的时间为一个ＲＴＴ（１87ms）
    数据在完成三次握手后到达，如果服务端还没有调用accept，数据放在接收队列中

accetp: 从 complete 队列返回下一个socket，如果队列为空， sleep
    参数为 listening socket，返回为 connected socket
    一个服务一般只存在一个 listening socket，生命周期与服务的生命周期相同
    为每一个client建立一个 connected socket, 当完成此次操作，此socket 被关闭

TODO（Ｐ１４７）: print connected socket's address

fork: 进程创建自己的一个副本，分别执行两个任务．一个进程执行另外的程序，先创建自己的一个副本，然后运行 exec ，用另外的程序替换此副本．
    如果子进程结束，会产生zombies，父进程要 wait 子进程
    子进程结束会产生ＳＩＧＣＨＬＤ信号，父进程响应此信号，在信号处理函数中调用　wait/waitpid
由于linux对信号不会队列，如果同时发５个相同的信号，当第一个信号在处理时，后面的会合并成一个信号．这样，在处理多个 client连接时（产生多个SIGCHLD），可能会发生遗漏.　循环调用　waitpid(.., WNOHANG) WNOHANG表示当有子进程存在时，不阻塞．
    当响应信号时，父进程如果在 block 状态（调用accept），可能返回错误　ＥＩＮＴＲ, 这时，要对此进行处理，使其继续调用accept

当对 connected socket 在新的进程中执行时，在父进程中要关闭此 connected socket．在子进程中先关闭 listening socket，然后对 connected socket 处理，处理完毕后，关闭此 connected socket.

close: socket 也是一个文件，也有引用计数．close 只是将计数减　１，当计数为０　时，才关闭连接．如果要强行发送ＦＩＮ,调用　shutdown

wait/waitpid: 当有子进程时，阻塞，如果有一个子进程结束，返回．

在连接建立，accept 返回前，连接断开．
    此时accept 返回　ＥＣＯＮＮＡＢＯＲＴＥＤ 
    client socket option SO_LINGER　发送ＲＳＴ

服务进程崩溃: 如果此时client正在等待用户输入，将检测不到此情况，只有在下次发送数据时才会发现此情况
p184

当一个socket接收到了RST，当向此socket写入数据时，返回信号：　SIGPIPE，　默认行为会退出进程．如果进程对此信号进行了处理，或者对此信号ignore，　写入返回 EPIPE

当socket接收到ＦＩＮ,　说明对方关闭，此时还是可以写入数据的．

socket option SO_KEEPALIVE 检测服务器是否崩溃

I/O multiplexing 当有多个输入时，如 socket 和 stdin，同时只能阻塞在一个输入上，此时，如果另外一个发生错误，将无法知道．此时，使用select 或 pool

blocking I/O
nonblocking I/O
I/O multiplexing (select, poll)
signal I/O (SIGIO)  : 当ＩＯ操作可以被　initiated，收到通知
asyn I/O (aio_xxx) :当ＩＯ操作完成后，收到通知

socket option: SO_RCVLOWAT, socket buffer 的 low-water，当 select 一个socket，并检测其是否可读时，当 buffer中的数据达到 low-water，就返回，默认为 1
                SO_SENDLOWAT 发送buffer的 low-water,　默认2048


close 双方向关闭
shutdown:
    SHUT_RD 关闭读
    SHUT_WR 关闭写
    SHUT_RDWR 关闭读写

connect 在阻塞过程中，如果有信号产生（软中断），会返回EINTR, 使用此技术可以设置socket的超时，通过SIGALRM消息

nonblocking I/O

options: O_NONBLOCK

接收数据，发送数据，accept, 返回：　EWOULDBLOCK
connet 返回： EINPROGRESS
