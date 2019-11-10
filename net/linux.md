网络 packet 的所有信息都保存在 sk_buff 中
sk_buff 中包含指针 head data tail end 分别指向实际的数据块
skb_shared_info 在 sk_buff 的最后，包含一些扩展信息。

发送数据

inode
i_sock = 1
u.socket_i  --  socket
                ops         ----    proto_ops
                                    connect
                                    ...
                                    accept
                                    listen
                                    sendmsg
                                    recvmsg

                sk          --- sock
                inode           prot ----   proto
                                            connect
                                            ...
                                            accept
                                            listen
                                            sendmsg
                                            recvmsg

-------------------------------------------------------
        sock->ops->sendmsg = inet_sendmsg()
        socket specific processing
        socket 类型为 INET, 所以调用 inet_sendmsg
        在 INET socket 上有可能注册多种协议，这里调用 tcp
-------------------------------------------------------
        sk->prot->sendmsg = tcp_sendmsg()
        protocol specific processing
        sk 指向具体协议，此协议相关的操作由 prot 指向
        用户数据在这里被分成块，拷到小的 sk_buff 中。这些 sk_buff 被拷到 socket 发送 buffer
-------------------------------------------------------
        sk->tp->af_specific->queue_xmit = ip_queue_xmit()
        network layer processing
        新建 IP 头，并应用 firewall 原则。如果 firewall 允许通过，进入下一层
-------------------------------------------------------
        dev_queue_xmit()
        QOS & link layer processing
        网上由 net_device 对象表示
-------------------------------------------------------
        Tx soft IRQ
-------------------------------------------------------
            Packet transmitted


            packet received
            packet 通过 DMA 拷贝到设备的 Rx ring buffer 后，中断产生
-------------------------------------------------------
            interrupt handler
            removes packet from DMA ring buffer
            从 DMA ring buffer 中把数据移动到 CPU 输入队列。每个 CPU 都有自己的输入队列。
            这一动作完成 Rx NET softIRQ 产生, 调用 netif_rx()
-------------------------------------------------------
            netif_rx(), Rx soft IRQ
            数据从 CPU 输入队列中移出，进行处理, 最后将放入到 socket 接收队列，过程如下：
-------------------------------------------------------
            netif_receive_skb(),  protocol switch
            决定是什么协议，发现是 IP, 调用 ip_rcv
-------------------------------------------------------
            ip_rcv()   IP layer processing
            此时会进行路由选择. 如果是提交到本地，应用 firewall 规则， 如果一切 ok ，调用ip_local_deliver_finish
-------------------------------------------------------
            ip_local_deliver_finish()
            INET protocol switcher
-------------------------------------------------------
            tcp_v4_rcv   TCP entry point
            找到对应的 socket
            可能是新的连接，也可是已有连接的数据，所以要查找不同的 hash 表
            如果是已有连接的数据，进入 socket 的接收队列，如果此时有数据要发送，和 ACK 一起发送。
            此时，应用程序可以从 socket 中读入数据了
-------------------------------------------------------
            <--sock<--->sock<--->sock
-------------------------------------------------------
            protocol specific processing
-------------------------------------------------------
        sock
        receive_queue----sk_buff<--->sk_buff
-------------------------------------------------------
        application reads from receive queue



        application                                     application
        browser                                         browser
        |                                               |
        presentation                                    presentation
        http                                            http
-------------------------------------------------------
        |                                               |
        session = socket layer                       session = socket layer
        inet_sendmsg()                               socket receive buffer
        |                                               |
        transport = tcp                             transport = tcp
        tcp_sendmsg()                               tcp_v4_rcv()
        |                                               |
        network = IP                                network = IP
        ip_quene_xmit()                             ip_rcv()
        |                                               |
        link = hard transmit                    link = driver
        dev_quene_xmit()                        interrupt processing
------------------------------------------------------------
        |
        physical layer     ---------------------> physical layer


数据两个阶段：
    从设备通过 DMA 到 CPU 输入队列
    从CPU 输入队列到 socket 接收队列
