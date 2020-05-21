# RTO (retransmission timeout)

## The Basic Algorithm

* SRTT (smoothed round-trip time)
* RTTVAR (round-trip time variation)
* clock granularity of G seconds.

   (2.1) 开始计算 RTT 时， 发送方发送一个 segment 后，设置
         RTO <- 3 seconds
         考虑到 "backing off" on repeated retransmission 
   (2.2) 第一个 RTT 时间 R 得到后, 设置
            SRTT <- R
            RTTVAR <- R/2
            RTO <- SRTT + max (G, 4*RTTVAR) = R + max(G, 2R)
   (2.3) 后面的 RTT 时间 R' 得到后，设置
            RTTVAR <- (1 - 1/8) * RTTVAR + 1/4 * |SRTT - R'| = 7/16R + 1/4|R - R'|
            SRTT <- (1 - 1/8) * SRTT + 1/8 * R' = R - 1/8(R' - R)

            RTO <- SRTT + max (G, K*RTTVAR) = 5R - 1/8(R' - R) - 1/2(R' - R)

   (2.4) RTO 小于 1 second, 设置为 1 second.
   (2.5) RTO 的最大值最少 60 seconds.

# Taking RTT Samples

RTT 样本不能用重传的包。

# Clock Granularity

时钟频率 <= 100 msec 比较好

# Managing the RTO Timer

   (5.1) 每次发送一个数据，如果定时器没启动，启动之
   (5.2) 所有发送的数据都收到确认后，关闭 retransmission timer.
   (5.3) 当收到一个 ACK 后，重启 retransmission timer, 这样重传定时器在 RTO seconds 后到期

当定时器到期

   (5.4) 重传最早的没有被确认的 segment 
   (5.5) RTO <- RTO * 2 ("back off the timer").  
   (5.6) 启动定时器

在重传之后，当收到一个新的确认后，开始重新计算 RTO