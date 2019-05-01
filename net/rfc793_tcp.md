# TCP要解决的问题
* 基本的数据传输 Basic Data Transfer
* 可靠性 Reliability
* 流量控制 Flow Control
* 多路复用 Multiplexing
* 连接 Connections
* 优先级和安全 Precedence and Security

#Basic Data Transfer:
TCP可以在两个主机之间传输连续的字节流，并且可以同时双向传输。一般TCP可自行决定何时阻塞，何时传输数据。

如果用户要确保提交的数据都被传输，可调用 push 功能。 A push causes the TCPs to promptly forward and deliver data up to that point to the receiver. The exact push point might not be visible to the receiving user and the push function does not supply a record boundary marker.

#Reliability:
数据包有可能损坏，丢失，重复，乱序。TCP必须能够从这些情况中恢复。

数据包中的每一个字节都被编号 sequence number，发送的每一个字节都要得到对方的确认acknowledgment (ACK)。如果在一定时间内没有得到确认，重传数据。

数据接收方通过sequence numbers确定数据包在字节流中的位置，并丢弃重复的数据 。数据包通过 checksum 来判断 数据的正确性。

#Flow Control:
接收方在发送ACK的同时，会发送一个 "window"，通知发送方最多还能接收多少数据。

#Multiplexing:
    A pair of sockets uniquely identifies each connection.

#Connections:
要实现可靠性和流量控制，数据流要初始化一些状态，如 sockets, sequence numbers, window sizes。所有这些状态的集合称为连接。

两个进程进行通信，必须建立连接（初始化这些状态）。当通信结束，必须关闭连接来释放这些资源。

#Precedence and Security:
    The users of TCP may indicate the security and precedence of their
    communication.  Provision is made for default values to be used when
    these features are not needed.

#Reliable Communication
一个数据包的第一个字节的 sequence number 称为此数据包的 segment sequence number

一个数据包被发送时，复制一份到重传队列，并开启计时器。 when the acknowledgment for that data is received, the segment is deleted from the queue.  If the acknowledgment is not received before the timer runs out, the segment is retransmitted.

发送方接收到一个ACK，不能保证数据被提交到了对方用户。只表示接收方做出了一个响应。

#Connection Establishment and Clearing
调用 OPEN，建立一个连接。连接的信息保存在 Transmission Control Block (TCB)。建立连接时同时决定此连接 actively pursued, or to be passively waited for.

passive OPEN 说明此进程是服务端。

当一个 TCB entry 处于等待状态时，一个包含 SYN 的数据包到达，则开始建立连接。当双方的 sequence numbers 同步完成后，此连接建立完成。

关闭连接也要发生数据包的交换，此时数据包包含 FIN

#Data Communication
数据放在用户的 buffer 中，如果此数据包含 PUSH，此 buffer 立即返回。 If data arrives that fills the user's buffer before a PUSH is seen, the data is passed to the user in buffer size units.

  TCP also provides a means to communicate to the receiver of data that
  at some point further along in the data stream than the receiver is
  currently reading there is urgent data.  TCP does not attempt to
  define what the user specifically does upon being notified of pending
  urgent data, but the general notion is that the receiving process will
  take action to process the urgent data quickly.

#Precedence and Security
  The TCP makes use of the internet protocol type of service field and
  security option to provide precedence and security on a per connection
  basis to TCP users.  

  TCP modules which operate in a multilevel secure environment must
  properly mark outgoing segments with the security, compartment, and
  precedence.  Such TCP modules must also provide to their users or
  higher level protocols such as Telnet or THP an interface to allow
  them to specify the desired security level, compartment, and
  precedence of connections.

#Header Format
  Source Port:  16 bits
  Destination Port:  16 bits

  Sequence Number:  32 bits

    The sequence number of the first data octet in this segment (except
    when SYN is present). If SYN is present the sequence number is the
    initial sequence number (ISN) and the first data octet is ISN+1.

  Acknowledgment Number:  32 bits

    If the ACK control bit is set this field contains the value of the
    next sequence number the sender of the segment is expecting to
    receive.  Once a connection is established this is always sent.

  Data Offset:  4 bits

    The number of 32 bit words in the TCP Header.  This indicates where
    the data begins.  The TCP header (even one including options) is an
    integral number of 32 bits long.

  Reserved:  6 bits

  Control Bits:  6 bits (from left to right):

    URG:  Urgent Pointer field significant
    ACK:  Acknowledgment field significant
    PSH:  Push Function
    RST:  Reset the connection
    SYN:  Synchronize sequence numbers
    FIN:  No more data from sender

  Window:  16 bits

    The number of data octets beginning with the one indicated in the
    acknowledgment field which the sender of this segment is willing to
    accept.

  Checksum:  16 bits

    The checksum also covers a 96 bit pseudo header conceptually
    prefixed to the TCP header.  This pseudo header contains the Source
    Address, the Destination Address, the Protocol, and TCP length.
    This gives the TCP protection against misrouted segments.  This
    information is carried in the Internet Protocol and is transferred
    across the TCP/Network interface in the arguments or results of
    calls by the TCP on the IP.

                     +--------+--------+--------+--------+
                     |           Source Address          |
                     +--------+--------+--------+--------+
                     |         Destination Address       |
                     +--------+--------+--------+--------+
                     |  zero  |  PTCL  |    TCP Length   |
                     +--------+--------+--------+--------+

      The TCP Length is the TCP header length plus the data length in
      octets, and it does not count the 12 octets of the pseudo
      header.

  Urgent Pointer:  16 bits

    This field communicates the current value of the urgent pointer as a
    positive offset from the sequence number in this segment.  The
    urgent pointer points to the sequence number of the octet following
    the urgent data.  This field is only be interpreted in segments with
    the URG control bit set.

  Options:  variable

    Options may occupy space at the end of the TCP header and are a
    multiple of 8 bits in length.  All options are included in the
    checksum.  An option may begin on any octet boundary.  There are two
    cases for the format of an option:

      Case 1:  A single octet of option-kind.

      Case 2:  An octet of option-kind, an octet of option-length, and
               the actual option-data octets.

    The option-length counts the two octets of option-kind and
    option-length as well as the option-data octets.

    Note that the list of options may be shorter than the data offset
    field might imply.  The content of the header beyond the
    End-of-Option option must be header padding (i.e., zero).

    A TCP must implement all options.
    Currently defined options include (kind indicated in octal):

      Kind     Length    Meaning
      ----     ------    -------
       0         -       End of option list.
       1         -       No-Operation.
       2         4       Maximum Segment Size.

    Specific Option Definitions

      End of Option List

        +--------+
        |00000000|
        +--------+
         Kind=0

        This option code indicates the end of the option list.  This
        might not coincide with the end of the TCP header according to
        the Data Offset field.  This is used at the end of all options,
        not the end of each option, and need only be used if the end of
        the options would not otherwise coincide with the end of the TCP
        header.

      No-Operation

        +--------+
        |00000001|
        +--------+
         Kind=1

        This option code may be used between options, for example, to
        align the beginning of a subsequent option on a word boundary.
        There is no guarantee that senders will use this option, so
        receivers must be prepared to process options even if they do
        not begin on a word boundary.

      Maximum Segment Size

        +--------+--------+---------+--------+
        |00000010|00000100|   max seg size   |
        +--------+--------+---------+--------+
         Kind=2   Length=4

        Maximum Segment Size Option Data:  16 bits

          If this option is present, then it communicates the maximum
          receive segment size at the TCP which sends this segment.
          This field must only be sent in the initial connection request
          (i.e., in segments with the SYN control bit set).  If this
          option is not used, any segment size is allowed.

  Padding:  variable
    确保 TCP header ends and data begins on a 32 bit boundary.  

#Terminology
TCB 中存储着如下信息：
  the local and remote socket numbers
  the security and precedence of the connection
  pointers to the user's send and receive buffers
  pointers to the retransmit queue and to the current segment.
  和 send and receive sequence numbers 相关的一些变量：

    Send Sequence Variables

      SND.UNA - 已发送，还没确认
      SND.NXT - send next
      SND.WND - send window
      SND.UP  - send urgent pointer
      SND.WL1 - segment sequence number used for last window update
      SND.WL2 - segment acknowledgment number used for last window update
      ISS     - initial send sequence number

    Receive Sequence Variables

      RCV.NXT - receive next
      RCV.WND - receive window
      RCV.UP  - receive urgent pointer
      IRS     - initial receive sequence number

  Send Sequence Space

                   1         2          3          4
              ----------|----------|----------|----------
                     SND.UNA    SND.NXT    SND.UNA
                                          +SND.WND

        1 - 已确认的 sequence numbers
        2 - 已发送，还没确认
        3 - sequence numbers allowed for new data transmission
        4 - future sequence numbers which are not yet allowed

  The send window is the portion of the sequence space labeled 3

  Receive Sequence Space

                       1          2          3
                   ----------|----------|----------
                          RCV.NXT    RCV.NXT
                                    +RCV.WND

        1 - old sequence numbers which have been acknowledged
        2 - sequence numbers allowed for new reception
        3 - future sequence numbers which are not yet allowed

  The receive window is the portion of the sequence space labeled 2 

    Current Segment Variables

      SEG.SEQ - segment sequence number
      SEG.ACK - segment acknowledgment number
      SEG.LEN - segment length
      SEG.WND - segment window
      SEG.UP  - segment urgent pointer
      SEG.PRC - segment precedence value

  一个连接在整个生命周期内有不同的状态：

    LISTEN - 等待连接
    SYN-SENT - 发送了连接请求，等待对方的连接请求
    SYN-RECEIVED - 接收到了连接请求，向对方发送连接请求并等待ACK
    ESTABLISHED - 连接正式建立
    FIN-WAIT-1 - 发送关闭请求，等待ACK。或等待关闭请求
    FIN-WAIT-2 - 等待关闭请求
    CLOSE-WAIT - 等待本地应用程序的关闭请求
    CLOSING - 等待关闭请求ACK
    LAST-ACK - 发送关闭请求，等待ACK
    TIME-WAIT - 等待足够时间，确保对方接收到了关闭请求
    CLOSED - 无连接，此时没有 TCB，当然就不存在连接。

  The events are the user calls, OPEN, SEND, RECEIVE, CLOSE, ABORT, and STATUS;
  the incoming segments, containing the SYN, ACK, RST and FIN flags; and timeouts.

#Sequence Numbers
一个 ACK 为 X 说明 all octets up to but not including X have been received.
    (a)  Determining that an acknowledgment refers to some sequence
         number sent but not yet acknowledged.

    (b)  Determining that all sequence numbers occupied by a segment
         have been acknowledged (e.g., to remove the segment from a
         retransmission queue).

    (c)  Determining that an incoming segment contains sequence numbers
         which are expected (i.e., that the segment "overlaps" the
         receive window).

  In response to sending data the TCP will receive acknowledgments.  The
  following comparisons are needed to process the acknowledgments.

    SND.UNA = oldest unacknowledged sequence number

    SND.NXT = next sequence number to be sent

    SEG.ACK = acknowledgment from the receiving TCP (next sequence
              number expected by the receiving TCP)

    SEG.SEQ = first sequence number of a segment

    SEG.LEN = the number of octets occupied by the data in the segment
              (counting SYN and FIN)

    SEG.SEQ+SEG.LEN-1 = last sequence number of a segment

  A new acknowledgment (called an "acceptable ack"), is one for which
  the inequality below holds:

    SND.UNA < SEG.ACK =< SND.NXT

  When data is received the following comparisons are needed:

    RCV.NXT = next sequence number expected on an incoming segments, and
        is the left or lower edge of the receive window

    RCV.NXT+RCV.WND-1 = last sequence number expected on an incoming
        segment, and is the right or upper edge of the receive window

    SEG.SEQ = first sequence number occupied by the incoming segment

    SEG.SEQ+SEG.LEN-1 = last sequence number occupied by the incoming
        segment

  A segment is judged to occupy a portion of valid receive sequence
  space if

    RCV.NXT =< SEG.SEQ < RCV.NXT+RCV.WND

  or

    RCV.NXT =< SEG.SEQ+SEG.LEN-1 < RCV.NXT+RCV.WND

  The first part of this test checks to see if the beginning of the
  segment falls in the window, the second part of the test checks to see
  if the end of the segment falls in the window; if the segment passes
  either part of the test it contains data in the window.

  Actually, it is a little more complicated than this.  Due to zero
  windows and zero length segments, we have four cases for the
  acceptability of an incoming segment:

    Segment Receive  Test
    Length  Window
    ------- -------  -------------------------------------------

       0       0     SEG.SEQ = RCV.NXT

       0      >0     RCV.NXT =< SEG.SEQ < RCV.NXT+RCV.WND

      >0       0     not acceptable

      >0      >0     RCV.NXT =< SEG.SEQ < RCV.NXT+RCV.WND
                  or RCV.NXT =< SEG.SEQ+SEG.LEN-1 < RCV.NXT+RCV.WND

  Note that when the receive window is zero no segments should be
  acceptable except ACK segments.  Thus, it is be possible for a TCP to
  maintain a zero receive window while transmitting data and receiving
  ACKs.  However, even when the receive window is zero, a TCP must
  process the RST and URG fields of all incoming segments.

#The SYN and FIN
只用在建立连接和关闭连接中。SYN is considered to occur before the first actual data
  octet of the segment in which it occurs, while the FIN is considered
  to occur after the last actual data octet in a segment in which it
  occurs.  The segment length (SEG.LEN) includes both data and sequence
  space occupying controls.  When a SYN is present then SEG.SEQ is the
  sequence number of the SYN.

#Initial Sequence Number Selection
一个连接建立时，双方必须同步ISN。 通过交换数据包来达到此目的，此时的数据包设定了 SYN 标志，数据包包含有ISN。  As a shorthand, segments carrying the SYN bit are also called "SYNs".

同步过程为双方都要发送自己的 ISN 给对方并收到ACK。

    1) A --> B  SYN my sequence number is X
    2) A <-- B  ACK your sequence number is X
    3) A <-- B  SYN my sequence number is Y
    4) A --> B  ACK your sequence number is Y

2 和 3 可以合并，所以称为3次握手。

#The TCP Quiet Time Concept
发送的所有字节与 sequence number 有一一对应关系。sequence number 每隔 2**32 循环一次。在源主机，一个数据包建立并进入发送队列时，TCP 都要消耗相应的 sequence number space。

为防止重复，sequence space 非常大。数据包在网络的最大生存时间不超过几十秒。100 megabits/sec, 循环一次时间为 5.4 minutes

如果主机崩溃，同时发送的数据还在路上，如果重新恢复，发送的sequence number可能重复，为了让在路上的数据包失效，主机要延迟 MSL 秒后才能发送数据。此时的 MSL 为 2 分钟。

#Establishing a connection
      TCP A                                                TCP B

  1.  CLOSED                                                LISTEN

  2.  SYN-SENT    --> <SEQ=100><CTL=SYN>               --> SYN-RECEIVED

  3.  ESTABLISHED <-- <SEQ=300><ACK=101><CTL=SYN,ACK>  <-- SYN-RECEIVED

  4.  ESTABLISHED --> <SEQ=101><ACK=301><CTL=ACK>       --> ESTABLISHED

  5.  ESTABLISHED --> <SEQ=101><ACK=301><CTL=ACK><DATA> --> ESTABLISHED

  At line 4, TCP A 发送一个包含ACK的空数据包; in line 5, TCP A sends some data. 两者的 sequence number 是一样的。因为 ACK 不占用 sequence number 空间.

      TCP A                                               TCP B

  1.  CLOSED                                              CLOSED

  2.  SYN-SENT     --> <SEQ=100><CTL=SYN>              ...

  3.  SYN-RECEIVED <-- <SEQ=300><CTL=SYN>              <-- SYN-SENT

  4.               ... <SEQ=100><CTL=SYN>              --> SYN-RECEIVED

  5.  SYN-RECEIVED --> <SEQ=100><ACK=301><CTL=SYN,ACK> ...

  6.  ESTABLISHED  <-- <SEQ=300><ACK=101><CTL=SYN,ACK> <-- SYN-RECEIVED

  7.               ... <SEQ=101><ACK=301><CTL=ACK>     --> ESTABLISHED

  The principle reason for the three-way handshake is to prevent old
  duplicate connection initiations from causing confusion.  To deal with
  this, a special control message, reset, has been devised.  If the
  receiving TCP is in a  non-synchronized state (i.e., SYN-SENT,
  SYN-RECEIVED), it returns to LISTEN on receiving an acceptable reset.
  If the TCP is in one of the synchronized states (ESTABLISHED,
  FIN-WAIT-1, FIN-WAIT-2, CLOSE-WAIT, CLOSING, LAST-ACK, TIME-WAIT), it
  aborts the connection and informs its user. 

      TCP A                                                TCP B

  1.  CLOSED                                               LISTEN

  2.  SYN-SENT    --> <SEQ=100><CTL=SYN>               ...

  3.  (duplicate) ... <SEQ=90><CTL=SYN>               --> SYN-RECEIVED

  4.  SYN-SENT    <-- <SEQ=300><ACK=91><CTL=SYN,ACK>  <-- SYN-RECEIVED

  5.  SYN-SENT    --> <SEQ=91><CTL=RST>               --> LISTEN


  6.              ... <SEQ=100><CTL=SYN>               --> SYN-RECEIVED

  7.  SYN-SENT    <-- <SEQ=400><ACK=101><CTL=SYN,ACK>  <-- SYN-RECEIVED

  8.  ESTABLISHED --> <SEQ=101><ACK=401><CTL=ACK>      --> ESTABLISHED

 TCP A detects that the ACK field is incorrect and returns a
  RST (reset) with its SEQ field selected to make the segment
  believable.  TCP B, on receiving the RST, returns to the LISTEN state.
  When the original SYN (pun intended) finally arrives at line 6, the
  synchronization proceeds normally.  If the SYN at line 6 had arrived
  before the RST, a more complex exchange might have occurred with RST's
  sent in both directions.

#Half-Open Connections and Other Anomalies
对方的连接关闭，向对方发送数据，得到 RST

      TCP A                                           TCP B

  1.  (CRASH)                               (send 300,receive 100)

  2.  CLOSED                                           ESTABLISHED

  3.  SYN-SENT --> <SEQ=400><CTL=SYN>              --> (??)

  4.  (!!)     <-- <SEQ=300><ACK=100><CTL=ACK>     <-- ESTABLISHED

  5.  SYN-SENT --> <SEQ=100><CTL=RST>              --> (Abort!!)

  6.  SYN-SENT                                         CLOSED

  7.  SYN-SENT --> <SEQ=400><CTL=SYN>              -->

  When the SYN arrives at line 3, TCP B, being in a synchronized state,
  and the incoming segment outside the window, responds with an
  acknowledgment indicating what sequence it next expects to hear (ACK
  100).  TCP A sees that this segment does not acknowledge anything it
  sent and, being unsynchronized, sends a reset (RST) because it has
  detected a half-open connection.  TCP B aborts at line 5.  TCP A will
  continue to try to establish the connection; the problem is now
  reduced to the basic 3-way handshake of figure 7.

        TCP A                                              TCP B

  1.  (CRASH)                                   (send 300,receive 100)

  2.  (??)    <-- <SEQ=300><ACK=100><DATA=10><CTL=ACK> <-- ESTABLISHED

  3.          --> <SEQ=100><CTL=RST>                   --> (ABORT!!)

  An old duplicate arriving at TCP B (line 2) stirs B
  into action.  A SYN-ACK is returned (line 3) and causes TCP A to
  generate a RST (the ACK in line 3 is not acceptable).  TCP B accepts
  the reset and returns to its passive LISTEN state.

      TCP A                                         TCP B

  1.  LISTEN                                        LISTEN

  2.       ... <SEQ=Z><CTL=SYN>                -->  SYN-RECEIVED

  3.  (??) <-- <SEQ=X><ACK=Z+1><CTL=SYN,ACK>   <--  SYN-RECEIVED

  4.       --> <SEQ=Z+1><CTL=RST>              -->  (return to LISTEN!)

  5.  LISTEN                                        LISTEN

#Reset Generation

  As a general rule, reset (RST) must be sent whenever a segment arrives
  which apparently is not intended for the current connection.  A reset
  must not be sent if it is not clear that this is the case.

  There are three groups of states:

    1.  If the connection does not exist (CLOSED) then a reset is sent
    in response to any incoming segment except another reset.  In
    particular, SYNs addressed to a non-existent connection are rejected
    by this means.

    If the incoming segment has an ACK field, the reset takes its
    sequence number from the ACK field of the segment, otherwise the
    reset has sequence number zero and the ACK field is set to the sum
    of the sequence number and segment length of the incoming segment.
    The connection remains in the CLOSED state.

    2.  If the connection is in any non-synchronized state (LISTEN,
    SYN-SENT, SYN-RECEIVED), and the incoming segment acknowledges
    something not yet sent (the segment carries an unacceptable ACK), or
    if an incoming segment has a security level or compartment which
    does not exactly match the level and compartment requested for the
    connection, a reset is sent.

    If our SYN has not been acknowledged and the precedence level of the
    incoming segment is higher than the precedence level requested then
    either raise the local precedence level (if allowed by the user and
    the system) or send a reset; or if the precedence level of the
    incoming segment is lower than the precedence level requested then
    continue as if the precedence matched exactly (if the remote TCP
    cannot raise the precedence level to match ours this will be
    detected in the next segment it sends, and the connection will be
    terminated then).  If our SYN has been acknowledged (perhaps in this
    incoming segment) the precedence level of the incoming segment must
    match the local precedence level exactly, if it does not a reset
    must be sent.

    If the incoming segment has an ACK field, the reset takes its
    sequence number from the ACK field of the segment, otherwise the
    reset has sequence number zero and the ACK field is set to the sum
    of the sequence number and segment length of the incoming segment.
    The connection remains in the same state.

    3.  If the connection is in a synchronized state (ESTABLISHED,
    FIN-WAIT-1, FIN-WAIT-2, CLOSE-WAIT, CLOSING, LAST-ACK, TIME-WAIT),
    any unacceptable segment (out of window sequence number or
    unacceptible acknowledgment number) must elicit only an empty
    acknowledgment segment containing the current send-sequence number
    and an acknowledgment indicating the next sequence number expected
    to be received, and the connection remains in the same state.

    If an incoming segment has a security level, or compartment, or
    precedence which does not exactly match the level, and compartment,
    and precedence requested for the connection,a reset is sent and
    connection goes to the CLOSED state.  The reset takes its sequence
    number from the ACK field of the incoming segment.

#Reset Processing
如果当前状态不是 SYN-SENT, 通过检查 SEQ-fields 验证 RST。 sequence number 必须在 window 范围。
如果当前状态是 SYN-SENT (a RST received in response to an initial SYN), the RST is acceptable if the ACK field acknowledges the SYN.

当接收到 RST ，首先进行验证，然后改变状态。 当前为 LISTEN 时，忽略它。如果当前是 SYN-RECEIVED 并且前一状态是 LISTEN ，回到 LISTEN 状态，否则终止连接，设为 CLOSED 状态. 其它情况都终止连接，通知应用程序，设为 CLOSED 状态

#Closing a Connection
发送方关闭连接后，还可能接收数据。发送方在关闭连接时，会将buffer中的所有数据都进行发送。

##Local user initiates the close
FIN 数据加入发送队列。数据被发送后，进入 FIN-WAIT-1 状态, 此时可以接收数据。在接收到ACK之前，所有的数据，包含FIN，都可以被重传。

对方回应FIN，并且发送自己的FIN过来

对对方的FIN进行ACK.

Note that a TCP receiving a FIN will ACK but not send its own FIN until its user has CLOSED the connection also.

##TCP receives a FIN from the network
接收到一个 FIN，发送一个ACK到对方，并通知应用程序连接要关闭了。应用程序回应一个 CLOSE，将buffer中数据发送完后，发送 FIN 到对方，然后等待对方的 ACK。当收到后，删除连接。如果没收到，超时后，终止连接并通知程序。

##both users close simultaneously
    A simultaneous CLOSE by users at both ends of a connection causes
    FIN segments to be exchanged.  When all segments preceding the FINs
    have been processed and acknowledged, each TCP can ACK the FIN it
    has received.  Both will, upon receiving these ACKs, delete the
    connection.

      TCP A                                                TCP B

  1.  ESTABLISHED                                          ESTABLISHED

  2.  (Close)
      FIN-WAIT-1  --> <SEQ=100><ACK=300><CTL=FIN,ACK>  --> CLOSE-WAIT

  3.  FIN-WAIT-2  <-- <SEQ=300><ACK=101><CTL=ACK>      <-- CLOSE-WAIT

  4.                                                       (Close)
      TIME-WAIT   <-- <SEQ=300><ACK=101><CTL=FIN,ACK>  <-- LAST-ACK

  5.  TIME-WAIT   --> <SEQ=101><ACK=301><CTL=ACK>      --> CLOSED

  6.  (2 MSL)
      CLOSED

                         Normal Close Sequence

      TCP A                                                TCP B

  1.  ESTABLISHED                                          ESTABLISHED

  2.  (Close)                                              (Close)
      FIN-WAIT-1  --> <SEQ=100><ACK=300><CTL=FIN,ACK>  ... FIN-WAIT-1
                  <-- <SEQ=300><ACK=100><CTL=FIN,ACK>  <--
                  ... <SEQ=100><ACK=300><CTL=FIN,ACK>  -->

  3.  CLOSING     --> <SEQ=101><ACK=301><CTL=ACK>      ... CLOSING
                  <-- <SEQ=301><ACK=101><CTL=ACK>      <--
                  ... <SEQ=101><ACK=301><CTL=ACK>      -->

  4.  TIME-WAIT                                            TIME-WAIT
      (2 MSL)                                              (2 MSL)
      CLOSED                                               CLOSED

                      Simultaneous Close Sequence

#Precedence and Security
连接的安全不低于通信的双方端口。

#Data Communication
The CLOSE user call implies a push function, as does the FIN control flag in an incoming segment.

#Retransmission Timeout
重传时间必须动态决定。

#Managing the Window
window 表示接收方接收数据 buffer 的大小

如果发送方的发送窗口为0,发送方也必须能从程序接收并发送一个字节的数据。

如果接收方的窗口为0,发送方也要进行数据重传。此时重传的时间间隔为2分钟。这是为了保证窗口大小不为0时可以获得通知。

如果接收方的窗口为0,有数据到达时，也要回应一个ACK 和当前窗口（大小为0）。

    Window Management Suggestions

      Allocating a very small window causes data to be transmitted in
      many small segments when better performance is achieved using
      fewer large segments.

      One suggestion for avoiding small windows is for the receiver to
      defer updating a window until the additional allocation is at
      least X percent of the maximum allocation possible for the
      connection (where X might be 20 to 40).

      Another suggestion is for the sender to avoid sending small
      segments by waiting until the window is large enough before
      sending data.  If the user signals a push function then the
      data must be sent even if it is a small segment.

      Note that the acknowledgments should not be delayed or unnecessary
      retransmissions will result.  One strategy would be to send an
      acknowledgment when a small segment arrives (with out updating the
      window information), and then to send another acknowledgment with
      new window information when the window is larger.

      The segment sent to probe a zero window may also begin a break up
      of transmitted data into smaller and smaller segments.  If a
      segment containing a single data octet sent to probe a zero window
      is accepted, it consumes one octet of the window now available.
      If the sending TCP simply sends as much as it can whenever the
      window is non zero, the transmitted data will be broken into
      alternating big and small segments.  As time goes on, occasional
      pauses in the receiver making window allocation available will
      result in breaking the big segments into a small and not quite so
      big pair. And after a while the data transmission will be in
      mostly small segments.

      The suggestion here is that the TCP implementations need to
      actively attempt to combine small window allocations into larger
      windows, since the mechanisms for managing the window tend to lead
      to many small windows in the simplest minded implementations.
