# tcp

## Basic Data Transfer

如果用户要确保提交的数据都被传输，可调用 push 功能。 push 操作使所有数据被发送，并提交给应用程序。push 操作对接收方是不可见的，所以不能被用作数据边界。

## Connections

要实现可靠性和流量控制，数据流要初始化一些状态，如 sockets, sequence numbers, window sizes。所有这些状态的集合称为连接。

两个进程进行通信，必须建立连接（初始化这些状态）。当通信结束，必须关闭连接来释放这些资源。

## Precedence and Security

The users of TCP may indicate the security and precedence of their communication.  Provision is made for default values to be used when these features are not needed.

连接的安全不低于通信的双方端口。

## Reliable Communication

数据包第一个字节的 sequence number 称为此数据包的 segment sequence number

数据包被发送时，复制一份到重传队列，并开启计时器。 接收到 ACK 后，从重传队列中删除。如果计时器超时 ACK 还没收到，重传数据。

发送方接收到一个ACK，不能保证数据被提交到了对方用户。只表示接收方做出了一个响应。

## Connection Establishment and Clearing

调用 OPEN，建立一个连接。连接的信息保存在 Transmission Control Block (TCB)。建立连接时同时决定是主动打开还是被动打开。

被动 OPEN 说明此进程是服务端。

当一个 TCB entry 处于等待状态时，一个包含 SYN 的数据包到达，则开始建立连接。当双方的 sequence numbers 同步完成后，此连接建立完成。

关闭连接也要发生数据包的交换，此时数据包包含 FIN

## Data Communication

数据放在用户的 buffer 中，如果数据包含 PUSH，buffer 立即返回。 如果 buffer 中的数据不包含 PUSH，buffer 満了之后返回给应用程序。

## Header Format

* Source Port:  16 bits
* Destination Port:  16 bits
* Sequence Number:  32 bits

  If SYN is present the sequence number is the initial sequence number (ISN) and the first data octet is ISN+1.

* Acknowledgment Number:  32 bits

  If the ACK control bit is set this field contains the value of the next sequence number the sender of the segment is expecting to receive.  Once a connection is established this is always sent.

* Data Offset:  4 bits

  The number of 32 bit words in the TCP Header.  This indicates where the data begins.  The TCP header (even one including options) is an integral number of 32 bits long.

* Reserved:  6 bits
* Control Bits:  6 bits (from left to right):

    URG:  Urgent Pointer field significant
    ACK:  Acknowledgment field significant
    PSH:  Push Function
    RST:  Reset the connection
    SYN:  Synchronize sequence numbers
    FIN:  No more data from sender

* Window:  16 bits

  接收方可以接收的数据大小。

* Checksum:  16 bits

  checksum 包含在 TCP 头部之前的 96 bit pseudo header。 pseudo header 包含 Source Address, the Destination Address, the Protocol, and TCP length。This gives the TCP protection against misrouted segments.

      +--------+--------+--------+--------+
      |           Source Address          |
      +--------+--------+--------+--------+
      |         Destination Address       |
      +--------+--------+--------+--------+
      |  zero  |  PTCL  |    TCP Length   |
      +--------+--------+--------+--------+

  TCP Length is the TCP header length plus the data length in octets, and it does not count the 12 octets of the pseudo header.

* Urgent Pointer:  16 bits

  This field communicates the current value of the urgent pointer as a positive offset from the sequence number in this segment.  The urgent pointer points to the sequence number of the octet following the urgent data.  This field is only be interpreted in segments with the URG control bit set.

* Options:  variable

  Options may occupy space at the end of the TCP header and are a multiple of 8 bits in length.  All options are included in the checksum.  An option may begin on any octet boundary.  There are two cases for the format of an option:

  1. A single octet of option-kind.
  2. An octet of option-kind, an octet of option-length, and the actual option-data octets.

  The option-length counts the two octets of option-kind and option-length as well as the option-data octets.

  Kind | Length | Meaning
  -----|:------:|--------:
   0   |   -    | End of option list.
   1   |   -    | No-Operation.
   2   |   4    | Maximum Segment Size.

  * End of Option List

    +--------+
    |00000000|
    +--------+

    Kind=0

    This might not coincide with the end of the TCP header according to the Data Offset field.  This is used at the end of all options, not the end of each option, and need only be used if the end of the options would not otherwise coincide with the end of the TCP header.

  * No-Operation

    +--------+
    |00000001|
    +--------+

    Kind=1

    This option code may be used between options, for example, to align the beginning of a subsequent option on a word boundary. There is no guarantee that senders will use this option, so receivers must be prepared to process options even if they do not begin on a word boundary.

  * Maximum Segment Size

    +--------+--------+---------+--------+
    |00000010|00000100|   max seg size   |
    +--------+--------+---------+--------+

    Kind=2   Length=4

    Maximum Segment Size Option Data:  16 bits

    If this option is present, then it communicates the maximum receive segment size at the TCP which sends this segment. 此字段只能在建立连接时发送(i.e., in segments with the SYN control bit set).  If this option is not used, any segment size is allowed.

  * Padding:  variable
    确保 TCP header ends and data begins on a 32 bit boundary.  

## TCB

TCB 中存储着如下信息：

* 双方的 socket numbers
* the security and precedence of the connection
* 应用程序的发送，接收 buffer pointers
* 重传队列的指针，当前数据包的指针

send and receive sequence numbers 相关的变量：

* Send Sequence Variables

  * SND.UNA - 已发送，还没确认
  * SND.NXT - send next
  * SND.WND - send window
  * SND.UP  - send urgent pointer
  * SND.WL1 - segment sequence number used for last window update
  * SND.WL2 - segment acknowledgment number used for last window update
  * ISS     - initial send sequence number

* Receive Sequence Variables

  * RCV.NXT - receive next
  * RCV.WND - receive window
  * RCV.UP  - receive urgent pointer
  * IRS     - initial receive sequence number

* Send Sequence Space

           1         2          3          4
      ----------|----------|----------|----------
             SND.UNA    SND.NXT    SND.UNA
                                   +SND.WND

        1 - 已确认的 sequence numbers
        2 - 已发送，还没确认
        3 - sequence numbers allowed for new data transmission
        4 - future sequence numbers which are not yet allowed

    The send window is the portion of the sequence space labeled 3

* Receive Sequence Space

          1          2          3
      ----------|----------|----------
             RCV.NXT    RCV.NXT
                       +RCV.WND

        1 - old sequence numbers which have been acknowledged
        2 - sequence numbers allowed for new reception
        3 - future sequence numbers which are not yet allowed

  The receive window is the portion of the sequence space labeled 2

* Current Segment Variables

  * SEG.SEQ - segment sequence number
  * SEG.ACK - segment acknowledgment number
  * SEG.LEN - segment length
  * SEG.WND - segment window
  * SEG.UP  - segment urgent pointer
  * SEG.PRC - segment 优先级

## 连接在生命周期内的状态

* LISTEN        - 等待连接
* SYN-SENT      - 发送了连接请求，等待对方的连接请求
* SYN-RECEIVED  - 接收到了连接请求，向对方发送连接请求并等待ACK
* ESTABLISHED   - 连接正式建立
* FIN-WAIT-1    - 发送关闭请求，等待ACK。或等待关闭请求
* FIN-WAIT-2    - 等待关闭请求
* CLOSE-WAIT    - 等待本地应用程序的关闭请求
* CLOSING       - 等待关闭请求ACK
* LAST-ACK      - 发送关闭请求，等待ACK
* TIME-WAIT     - 等待足够时间，确保对方接收到了关闭请求
* CLOSED        - 无连接，此时没有 TCB，当然就不存在连接。

The events are the user calls, OPEN, SEND, RECEIVE, CLOSE, ABORT, and STATUS; the incoming segments, containing the SYN, ACK, RST and FIN flags; and timeouts.

## Sequence Numbers

一个 ACK 为 X 说明 all octets up to but not including X have been received. ACK 用于如下方面：

1. 确认已发送，未确认的 sequence number
2. 确认所有已确认的 sequence numbers (如： 从重传队列中删除)
3. 确认接收的数据包的 sequence numbers 是正确的(如： 防止重复).

TCP 接收 ACK 时，用到下面变量：

* SND.UNA = 最后的未确认 sequence number
* SND.NXT = 下一个被发送的 sequence number
* SEG.ACK = 从对方接收的 ACK (对方希望接收的下一个 sequence number)
* SEG.SEQ = 数据包的第一个 sequence number
* SEG.LEN = 数据包的长度 (counting SYN and FIN) SEG.SEQ + SEG.LEN-1 = 数据包最后的 sequence number

SND.UNA < SEG.ACK =< SND.NXT

TCP 接收数据包时，用到下面变量：

* RCV.NXT = 希望的 sequence number, 同时也是接收 window 的下边界
* RCV.NXT + RCV.WND - 1 = 接收 window 的上边界
* SEG.SEQ = 数据包的第一个 sequence number
* SEG.SEQ + SEG.LEN - 1 = 数据包的最后一个 sequence number

接收的数据包必须要在接收 window 之内：

     RCV.NXT =< SEG.SEQ < RCV.NXT+RCV.WND
  || RCV.NXT =< SEG.SEQ+SEG.LEN-1 < RCV.NXT+RCV.WND

还要考虑 0 window 和 0 长度的数据包：

数据包长度 | 接收 Window |  判断条件
----------|:----------:|-----
   0      |     0      | SEG.SEQ = RCV.NXT
   0      |    >0      | RCV.NXT =< SEG.SEQ < RCV.NXT+RCV.WND
  >0      |     0      | not acceptable

当接收 window 为 0, ACK，RST，URG 还是能接收的。

### The SYN and FIN

只用在建立连接和关闭连接中。SYN is considered to occur before the first actual data
  octet of the segment in which it occurs, while the FIN is considered
  to occur after the last actual data octet in a segment in which it
  occurs.  The segment length (SEG.LEN) includes both data and sequence
  space occupying controls.  When a SYN is present then SEG.SEQ is the
  sequence number of the SYN.

### Initial Sequence Number Selection

一个连接建立时，双方必须同步ISN。 通过交换数据包来达到此目的，此时的数据包设定了 SYN 标志，数据包包含有ISN。  As a shorthand, segments carrying the SYN bit are also called "SYNs".

同步过程为双方都要发送自己的 ISN 给对方并收到ACK。

    1) A --> B  SYN my sequence number is X
    2) A <-- B  ACK your sequence number is X
    3) A <-- B  SYN my sequence number is Y
    4) A --> B  ACK your sequence number is Y

2 和 3 可以合并，所以称为3次握手。

## The TCP Quiet Time Concept

发送的所有字节与 sequence number 有一一对应关系。sequence number 每隔 2**32 循环一次。在源主机，一个数据包建立并进入发送队列时，TCP 都要消耗相应的 sequence number space。

为防止重复，sequence space 非常大。数据包在网络的最大生存时间不超过几十秒。100 megabits/sec, 循环一次时间为 5.4 minutes

如果主机崩溃，同时发送的数据还在路上，如果重新恢复，发送的sequence number可能重复，为了让在路上的数据包失效，主机要延迟 MSL 秒后才能发送数据。此时的 MSL 为 2 分钟。

## 建立连接

### 情况1

 . | TCP A       |                                           | TCP B
---|:------------|:-----------------------------------------:|-----
1  | CLOSED      |                                           | LISTEN
2  | SYN-SENT    | --> <SEQ=100><CTL=SYN>                --> | SYN-RECEIVED
3  | ESTABLISHED | <-- <SEQ=300><ACK=101><CTL=SYN,ACK>   <-- | SYN-RECEIVED
4  | ESTABLISHED | --> <SEQ=101><ACK=301><CTL=ACK>       --> | ESTABLISHED
5  | ESTABLISHED | --> <SEQ=101><ACK=301><CTL=ACK><DATA> --> | ESTABLISHED

第 4 行，TCP A 发送一个包含ACK的空数据包; 第 5 行, TCP A 发送了一些数据。两者的 sequence number 是一样的。因为 ACK 不占用 sequence number 空间.

### 情况2

 . | TCP A        |                                         | TCP B
---|:-------------|:---------------------------------------:|-----
1  | CLOSED       |                                         | CLOSED
2  | SYN-SENT     | --> <SEQ=100><CTL=SYN>                  | ...
3  | SYN-RECEIVED | <-- <SEQ=300><CTL=SYN>              <-- | SYN-SENT
4  | ...          |     <SEQ=100><CTL=SYN>              --> | SYN-RECEIVED
5  | SYN-RECEIVED | --> <SEQ=100><ACK=301><CTL=SYN,ACK>     | ...
6  | ESTABLISHED  | <-- <SEQ=300><ACK=101><CTL=SYN,ACK> <-- | SYN-RECEIVED
7  | ...          |     <SEQ=101><ACK=301><CTL=ACK>     --> | ESTABLISHED

### 情况3

三次握手的好处在于可以避免重复初始化连接。为解决重复初始化的问题，定义了一个控件信息: reset。当接收到 reset 后，接收方在 non-synchronized 状态 (SYN-SENT, SYN-RECEIVED), 返回到 LISTEN 状态。接收方在 synchronized 状态 (ESTABLISHED, FIN-WAIT-1, FIN-WAIT-2, CLOSE-WAIT, CLOSING, LAST-ACK, TIME-WAIT), 终止连接并通知应用程序。

 . | TCP A        |                                         | TCP B
---|:-------------|:---------------------------------------:|-----
1  | CLOSED       |                                         | LISTEN
2  | SYN-SENT     | --> <SEQ=100><CTL=SYN>                  | ...
3  | (duplicate)  |     <SEQ=90><CTL=SYN>               --> | SYN-RECEIVED
4  | SYN-SENT     | <-- <SEQ=300><ACK=91><CTL=SYN,ACK>  <-- | SYN-RECEIVED
5  | SYN-SENT     | --> <SEQ=91><CTL=RST>               --> | LISTEN
6  | ...          |     <SEQ=100><CTL=SYN>              --> | SYN-RECEIVED
7  | SYN-SENT     | <-- <SEQ=400><ACK=101><CTL=SYN,ACK> <-- | SYN-RECEIVED
8  | ESTABLISHED  | --> <SEQ=101><ACK=401><CTL=ACK>     --> | ESTABLISHED

TCP A 发现 ACK 不正确，发送 RST ，其中的 SEQ 值表明此包是可信赖的包。TCP B, 接收到 RST, 返回 LISTEN 状态。行 6，如果 SYN 在 RST 之前到达，RST 会在双方向传递，这会导致更复杂的数据交换。

## Half-Open 连接和其它异常

### 情况1

对方的连接关闭，向对方发送数据，得到 RST

 . | TCP A    |                                     | TCP B
---|:---------|:-----------------------------------:|-----
1  | (CRASH)  |                                     | (send 300,receive 100)
2  | CLOSED   |                                     | ESTABLISHED
3  | SYN-SENT | --> <SEQ=400><CTL=SYN>          --> | (??)
4  | (!!)     | <-- <SEQ=300><ACK=100><CTL=ACK> <-- | ESTABLISHED
5  | SYN-SENT | --> <SEQ=100><CTL=RST>          --> | (Abort!!)
6  | SYN-SENT |                                     | CLOSED
7  | SYN-SENT | --> <SEQ=400><CTL=SYN>          --> |

When the SYN arrives at line 3, TCP B, being in a synchronized state, and the incoming segment outside the window, responds with an acknowledgment indicating what sequence it next expects to hear (ACK 100).  TCP A sees that this segment does not acknowledge anything it sent and, being unsynchronized, sends a reset (RST) because it has detected a half-open connection.  TCP B aborts at line 5.  TCP A will continue to try to establish the connection; the problem is now reduced to the basic 3-way handshake of figure 7.

### 情况2

        TCP A                                              TCP B

  1  (CRASH)                                   (send 300,receive 100)

  2  (??)    <-- <SEQ=300><ACK=100><DATA=10><CTL=ACK> <-- ESTABLISHED

  3          --> <SEQ=100><CTL=RST>                   --> (ABORT!!)

  An old duplicate arriving at TCP B (line 2) stirs B
  into action.  A SYN-ACK is returned (line 3) and causes TCP A to
  generate a RST (the ACK in line 3 is not acceptable).  TCP B accepts
  the reset and returns to its passive LISTEN state.

      TCP A                                         TCP B

  1  LISTEN                                        LISTEN

  2       ... <SEQ=Z><CTL=SYN>                -->  SYN-RECEIVED

  3  (??) <-- <SEQ=X><ACK=Z+1><CTL=SYN,ACK>   <--  SYN-RECEIVED

  4       --> <SEQ=Z+1><CTL=RST>              -->  (return to LISTEN!)

  5  LISTEN                                        LISTEN

## Reset Generation

接收到的数据包明显不属于此连接时，才发送 RST。

1. 接收到一个数据包（非 RST ），此数据包对应的连接不存在(CLOSED)， 会向对方发送 reset. In particular, SYNs addressed to a non-existent connection are rejected by this means.

    如果接收到的数据包是 ACK , reset takes its sequence number from the ACK field of the segment, 否则，reset 的 sequence number 为 0 并且 the ACK field is set to the sum of the sequence number and segment length of the incoming segment. The connection remains in the CLOSED state.

2. 连接在 non-synchronized 状态 (LISTEN, SYN-SENT, SYN-RECEIVED), 接收的数据包ACK，但响应的数据还没有被发送(数据包包含了不能被接受的 ACK), 或数据包的安全等级不匹配，a reset is sent.

    如果接收到的数据包是 the reset takes its sequence number from the ACK field of the segment, otherwise the reset has sequence number zero and the ACK field is set to the sum of the sequence number and segment length of the incoming segment. The connection remains in the same state.

3. 连接在 synchronized 状态 (ESTABLISHED, FIN-WAIT-1, FIN-WAIT-2, CLOSE-WAIT, CLOSING, LAST-ACK, TIME-WAIT), 所有无效数据包 (out of window sequence number or unacceptible acknowledgment number) must elicit only an empty acknowledgment segment containing the current send-sequence number and an acknowledgment indicating the next sequence number expected to be received, and the connection remains in the same state.

    If an incoming segment has a security level, or compartment, or precedence which does not exactly match the level, and compartment, and precedence requested for the connection,a reset is sent and connection goes to the CLOSED state.  The reset takes its sequence number from the ACK field of the incoming segment.

## Reset Processing

如果当前状态不是 SYN-SENT, 通过检查 SEQ-fields 验证 RST。 sequence number 必须在 window 范围。
如果当前状态是 SYN-SENT (a RST received in response to an initial SYN), the RST is acceptable if the ACK field acknowledges the SYN.

当接收到 RST ，首先进行验证，然后改变状态。 当前为 LISTEN 时，忽略它。如果当前是 SYN-RECEIVED 并且前一状态是 LISTEN ，回到 LISTEN 状态，否则终止连接，设为 CLOSED 状态. 其它情况都终止连接，通知应用程序，设为 CLOSED 状态

## Closing a Connection

发送方关闭连接后，还可能接收数据。发送方在关闭连接时，会将buffer中的所有数据都进行发送。

### Local user initiates the close

FIN 数据加入发送队列。数据被发送后，进入 FIN-WAIT-1 状态, 此时可以接收数据。在接收到ACK之前，所有的数据，包含FIN，都可以被重传。

对方回应FIN，并且发送自己的FIN过来

对对方的FIN进行ACK.

Note that a TCP receiving a FIN will ACK but not send its own FIN until its user has CLOSED the connection also.

### TCP receives a FIN from the network

接收到一个 FIN，发送一个ACK到对方，并通知应用程序连接要关闭了。应用程序回应一个 CLOSE，将buffer中数据发送完后，发送 FIN 到对方，然后等待对方的 ACK。当收到后，删除连接。如果没收到，超时后，终止连接并通知程序。

### both users close simultaneously

    A simultaneous CLOSE by users at both ends of a connection causes
    FIN segments to be exchanged.  When all segments preceding the FINs
    have been processed and acknowledged, each TCP can ACK the FIN it
    has received.  Both will, upon receiving these ACKs, delete the
    connection.

      TCP A                                                TCP B

  1  ESTABLISHED                                          ESTABLISHED

  2  (Close)
      FIN-WAIT-1  --> <SEQ=100><ACK=300><CTL=FIN,ACK>  --> CLOSE-WAIT

  3  FIN-WAIT-2  <-- <SEQ=300><ACK=101><CTL=ACK>      <-- CLOSE-WAIT

  4                                                       (Close)
      TIME-WAIT   <-- <SEQ=300><ACK=101><CTL=FIN,ACK>  <-- LAST-ACK

  5  TIME-WAIT   --> <SEQ=101><ACK=301><CTL=ACK>      --> CLOSED

  6  (2 MSL)
      CLOSED

                         Normal Close Sequence

      TCP A                                                TCP B

  1  ESTABLISHED                                          ESTABLISHED

  2  (Close)                                              (Close)
      FIN-WAIT-1  --> <SEQ=100><ACK=300><CTL=FIN,ACK>  ... FIN-WAIT-1
                  <-- <SEQ=300><ACK=100><CTL=FIN,ACK>  <--
                  ... <SEQ=100><ACK=300><CTL=FIN,ACK>  -->

  3  CLOSING     --> <SEQ=101><ACK=301><CTL=ACK>      ... CLOSING
                  <-- <SEQ=301><ACK=101><CTL=ACK>      <--
                  ... <SEQ=101><ACK=301><CTL=ACK>      -->

  4  TIME-WAIT                                            TIME-WAIT
      (2 MSL)                                              (2 MSL)
      CLOSED                                               CLOSED

The CLOSE user call implies a push function, as does the FIN control flag in an incoming segment.

## Retransmission Timeout

重传时间必须动态决定。

## Managing the Window

window 表示接收方接收数据 buffer 的大小

如果发送方的发送窗口为0,发送方也必须能从程序接收并发送一个字节的数据。

如果接收方的窗口为0,发送方也要进行数据重传。此时重传的时间间隔为2分钟。这是为了保证窗口大小不为0时可以获得通知。

如果接收方的窗口为0,有数据到达时，也要回应一个ACK 和当前窗口（大小为0）。

### Window Management Suggestions

window 太小，数据会被拆分到大量的小数据包内发送。如果要提升性能，选择稍大一点的数据包。

接收方可以在剩余空间达到连接最大空间的20%到40%之间时，再更新 window 大小，这样可以避免小 window 的出现。

发送方可以等 window 足够大时再发送数据。除非遇到 push。

ACK 不能延迟发送，不然会重传数据。One strategy would be to send an acknowledgment when a small segment arrives (with out updating the window information), and then to send another acknowledgment with new window information when the window is larger.

发送数据包去探测 zero window 也会使数据包越来越小。If a segment containing a single data octet sent to probe a zero window is accepted, it consumes one octet of the window now available. If the sending TCP simply sends as much as it can whenever the window is non zero, the transmitted data will be broken into alternating big and small segments.  As time goes on, occasional pauses in the receiver making window allocation available will result in breaking the big segments into a small and not quite so big pair. And after a while the data transmission will be in mostly small segments.

The suggestion here is that the TCP implementations need to actively attempt to combine small window allocations into larger windows, since the mechanisms for managing the window tend to lead to many small windows in the simplest minded implementations.
