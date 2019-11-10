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
