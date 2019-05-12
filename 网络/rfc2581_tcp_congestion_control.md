# TCP Congestion Control

防拥塞算法

* slow start
* congestion avoidance
* fast retransmit
* fast recovery

# Definitions

* SENDER MAXIMUM SEGMENT SIZE (SMSS):  发送方能发送的最大数据包的大小，这个大小不包含TCP/IP头部和选项。这个值可能由下面的值决定： maximum transmission unit of the network, the path MTU discovery algorithm, RMSS, or other factors.  
* RECEIVER MAXIMUM SEGMENT SIZE (RMSS): 接收方能接收的最大数据包的大小，这个大小不包含TCP/IP头部和选项。这个值在建立连接时由接收方回应中的 MSS 决定。如果不指定 MSS, 536 bytes.
* FULL-SIZED SEGMENT: 数据包，包含最大长度的数据(如： 数据包包含 SMSS bytes 的数据).
* RECEIVER WINDOW (rwnd) 最近的 advertised receiver window.
* CONGESTION WINDOW (cwnd):  限制 TCP 能发送的数据总量。在任何时候，如果数据包的 sequence number 大于已确认过的最大 sequence number 加上 min( cwnd, rwnd) 之和，则不能被发送。
* INITIAL WINDOW (IW):  三次握手后，发送方 congestion window 的初始值。
* LOSS WINDOW (LW):  发送方通过 retransmission timer 探测到数据丢失后 congestion window 的大小
* RESTART WINDOW (RW):  TCP 在空闲阶段结束后重新开始传输时 congestion window 的大小
* FLIGHT SIZE:  已发送，未确认的所有数据

# Slow Start and Congestion Avoidance

发送方使用 slow start 和 congestion avoidance 算法控制网络中数据的总量。congestion window (cwnd) 为发送方在接收到 ACK 前能发送到网络上的最大数据量。advertised window (rwnd) 为接收方能接收的数据总量。 cwnd 和 rwnd 的最小值控制着数据的传输

slow start threshold (ssthresh) 决定使用 slow start 或 congestion avoidance 算法。

如果一开始就发送大量数据到网络，很容易造成拥塞。所以要求 TCP 逐步探测网络情况。slow start 算法用于在开始传输时解决这个问题。或通过 retransmission timer 修复数据丢失后

IW, the initial value of cwnd, MUST be less than or equal to 2*SMSS bytes and MUST NOT be more than 2 segments.

We note that a non-standard, experimental TCP extension allows that a TCP MAY use a larger initial window (IW)

      IW = min (4*SMSS, max (2*SMSS, 4380 bytes))           (1)

With this extension, a TCP sender MAY use a 3 or 4 segment initial window, provided the combined size of the segments does not exceed 4380 bytes. 

The initial value of ssthresh MAY be arbitrarily high (for example, some implementations use the size of the advertised window), but it may be reduced in response to congestion. cwnd < ssthresh 时使用 slow start 算法, cwnd > ssthresh 时使用 congestion avoidance 算法。cwnd == ssthresh 两种算法都可能使用。

在 slow start 过程中, 在每个 ACK 到达后，cwnd 最多增加 SMSS 字节。当 cwnd 超过 ssthresh 或当检测到拥塞后， slow start 结束

在 congestion avoidance 过程中, 在每个 RTT 周期，cwnd 增加一个 full-sized segment，一直到检测到拥塞后. 下面是一个公式：

      cwnd += SMSS*SMSS/cwnd                     (2)

在每个非重复 ACK 到来后进行计算。(2) 提供了一个近似的计算。

Note: 如果(2)中 cwnd 太大，计算结果为0, 应设为1.

发送方使用 retransmission timer 检测到数据包丢失时，ssthresh 的值不能大于下面公式计算出的值：

      ssthresh = max (FlightSize / 2, 2*SMSS)            (3)

FlightSize 为在网络中还没有被确认的数据总量

Furthermore, upon a timeout cwnd 必须设为小于 loss window, LW, 它的值为一个 full-sized segment (与IW 的值无关).  在重传丢失的数据包后，发送方开始 slow start 算法增加 window，使其从一个 full-sized segment 到 ssthresh, 然后在此时，congestion avoidance 算法开始运行

# Fast Retransmit/Fast Recovery

当 out-of-order 数据到达时，接收方必须马上发送 immediate duplicate ACK. 对发送方，重复 ACK 的产生有多种原因：

1. 丢失数据。此时，在丢失数据之后的所有数据，都会触发 duplicate ACKs.  
2. 数据 re-ordering (not a rare event along some network paths).
3. 数据重复

发送方使用 fast retransmit 算法探测和修复数据丢失。此算法基于重复 ACK。如果收到3个重复 ACK，则表明此数据段丢失。在收到 3 个重复 ACK 后，开始重传，而不必等待 retransmission timer 超时

在使用 fast retransmit 算法发送丢失数据后，使用 fast recovery 算法控制传输，直到非重复 ACK 到达。这里不使用 slow start 的原因是，重复 ACK 可能是数据丢失，也可能是数据离开网络(although a massive segment duplication by the network can invalidate this conclusion)，而保存在接收方的buffer中。  而且，since the ACK "clock" is preserved, the TCP sender can continue to transmit new segments (although transmission must continue using a reduced cwnd).

1. When the third duplicate ACK is received, set ssthresh to no more than the value given in equation (3).
2. 重传丢失的数据包，cwnd = ssthresh + 3*SMSS 增加 congestion window
3. For each additional duplicate ACK received, increment cwnd by SMSS.  增加 congestion window
4. 被 cwnd 的新值和接收方的 advertised window允许，则传输数据包
5. When the next ACK arrives that acknowledges new data, set cwnd to ssthresh (the value set in step 1).  

# Additional Considerations

## Re-starting Idle Connections

在TCP经过长时间空闲后开始传输，congestion control algorithms 可能会发送大量数据造成网络阻塞。在空闲过后，TCP 不能使用 ACK clock 控制发送新数据，因为所有的 ACK 都被 drained，所以 TCP 可能发送 cwnd-size line-rate burst into the network

在TCP经过长时间空闲后开始传输，可以使用 slow start。Slow start 重新开始 ACK clock。TCP 超过1个retransmission timeout 后没接收到数据, cwnd is reduced to the value of the restart window (RW) before transmission begins.

we define RW = IW. non-standard extension defines RW = min(IW, cwnd)

HTTP 长连接使用接收到数据的最后时间来决定是否减少 cwnd 的值。

## Loss Recovery Mechanisms

当 window 内的第一个数据丢失被探测到，ssthresh 的值不能超过(3)计算出的值。当数据丢失被探测到，在每个 RTT 内发送的数据包总数，不能超过 outstanding segments(已发送，未确认) 数量的一半.  在所有的丢失数据成功重传之后，cwnd 的值必须小于 ssthresh，而且必须使用 congestion avoidance 来增加 cwnd. 在两个连续的 window 内都发生数据丢失，或在重传时发生丢失，会发生两次 congestion ，这种情况 cwnd (and ssthresh) 会两次变小。

# Security Considerations

当 retransmission timeouts 或接收到重复 ACK 时，TCP 会降低发送频率. 攻击者要降低连接的性能，只要使数据包或 ACK 丢失，或伪造过高的重复 ACK。 Causing two congestion control events back-to-back will often cut ssthresh to its minimum value of 2*SMSS, 导致连接马上进入到低性能的 congestion avoidance 阶段.