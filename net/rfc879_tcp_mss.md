# The TCP Maximum Segment Size Related Topics

The default IP Maximum Datagram Size is 576.

The default TCP Maximum Segment Size is 536.

# The TCP Maximum Segment Size Option

MSS 指实际数据的大小，不包含 TCP header 和 IP header.

A footnote:  MSS does not count the TCP SYN and FIN control bits even though SYN and FIN do consume TCP sequence numbers.

# The Relationship between IP Datagram and TCP Segment Sizes

接收方通知发送方使用的 MSS 如下计算：

      MSS = MTU - sizeof(TCPHDR) - sizeof(IPHDR)

发送方收到 MSS 后，发送的最大数据如下计算：

      SndMaxSegSiz = MIN((MTU - sizeof(TCPHDR) - sizeof(IPHDR)), MSS)

where MSS is the value in the option, and MTU is the Maximum Transmission Unit (or the maximum packet size) allowed on the directly attached network.

关于 sizeof(TCPHDR) 和 sizeof(IPHDR)，有三种情况：

* conservative
* moderate
* liberal

The conservative or pessimistic position assumes the worst -- that both the IP header and the TCP header are maximum size, that is, 60 octets each.

      MSS = MTU - 60 - 60 = MTU - 120

      If MTU is 576 then MSS = 456

The moderate position assumes the that the IP is maximum size (60 octets) and the TCP header is minimum size (20 octets), because there are no TCP header options currently defined that would normally be sent at the same time as data segments.

      MSS = MTU - 60 - 20 = MTU - 80

      If MTU is 576 then MSS = 496

The liberal or optimistic position assumes the best -- that both the IP header and the TCP header are minimum size, that is, 20 octets each.

      MSS = MTU - 20 - 20 = MTU - 40

      If MTU is 576 then MSS = 536

If nothing is said about MSS, the data sender may cram as much as possible into a 576 octet datagram, and if the datagram has minimum headers (which is most likely), the result will be 536 data octets in the TCP segment.  The rule relating MSS to the maximum datagram size ought to be consistent with this.

A practical point is raised in favor of the liberal position too. Since the use of minimum IP and TCP headers is very likely in the very large percentage of cases, it seems wasteful to limit the TCP segment data to so much less than could be transmitted at once, especially since it is less that 512 octets.

      For comparison:  536/576 is 93% data, 496/576 is 86% data, 456/576
      is 79% data.

# Maximum Packet Size

Each network has some maximum packet size, or maximum transmission unit (MTU).  Ultimately there is some limit imposed by the technology, but often the limit is an engineering choice or even an administrative choice.  Different installations of the same network product do not have to use the same maximum packet size.  Even within one installation not all host must use the same packet size (this way lies madness, though).

Some IP implementers have assumed that all hosts on the directly attached network will be the same or at least run the same implementation.  This is a dangerous assumption.  It has often developed that after a small homogeneous set of host have become operational additional hosts of different types are introduced into the environment.  And it has often developed that it is desired to use a copy of the implementation in a different inhomogeneous environment.

Designers of gateways should be prepared for the fact that successful gateways will be copied and used in other situation and installations.  Gateways must be prepared to accept datagrams as large as can be sent in the maximum packets of the directly attached networks.  Gateway implementations should be easily configured for installation in different circumstances.

A footnote:  The MTUs of some popular networks (note that the actual limit in some installations may be set lower by administrative policy):

      ARPANET, MILNET = 1007
      Ethernet (10Mb) = 1500
      Proteon PRONET  = 2046

# Source Fragmentation

A source host would not normally create datagram fragments.  Under normal circumstances datagram fragments only arise when a gateway must send a datagram into a network with a smaller maximum packet size than the datagram.  In this case the gateway must fragment the datagram (unless it is marked "don't fragment" in which case it is discarded, with the option of sending an ICMP message to the source reporting the problem).

It might be desirable for the source host to send datagram fragments if the maximum segment size (default or negotiated) allowed by the data receiver were larger than the maximum packet size allowed by the directly attached network.  However, such datagram fragments must not combine to a size larger than allowed by the destination host.

For example, if the receiving TCP announced that it would accept segments up to 5000 octets (in cooperation with the receiving IP) then the sending TCP could give such a large segment to the sending IP provided the sending IP would send it in datagram fragments that fit in the packets of the directly attached network.

There are some conditions where source host fragmentation would be necessary.

* If the host is attached to a network with a small packet size (for example 256 octets), and it supports an application defined to send fixed sized messages larger than that packet size (for example TFTP [5]).

* If the host receives ICMP Echo messages with data it is required to send an ICMP Echo-Reply message with the same data.  If the amount of data in the Echo were larger than the packet size of the directly attached network the following steps might be required: (1) receive the fragments, (2) reassemble the datagram, (3) interpret the Echo, (4) create an Echo-Reply, (5) fragment it, and (6) send the fragments.

# Gateway Fragmentation

Gateways must be prepared to do fragmentation.  It is not an optional feature for a gateway.

Gateways have no information about the size of datagrams destination hosts are prepared to accept.  It would be inappropriate for gateways to attempt to keep such information.

Gateways must be prepared to accept the largest datagrams that are allowed on each of the directly attached networks, even if it is larger than 576 octets.

Gateways must be prepared to fragment datagrams to fit into the packets of the next network, even if it smaller than 576 octets.

If a source host thought to take advantage of the local network's ability to carry larger datagrams but doesn't have the slightest idea if the destination host can accept larger than default datagrams and expects the gateway to fragment the datagram into default size fragments, then the source host is misguided.  If indeed, the destination host can't accept larger than default datagrams, it probably can't reassemble them either. If the gateway either passes on the large datagram whole or fragments into default size fragments the destination will not accept it.  Thus, this mode of behavior by source hosts must be outlawed.

A larger than default datagram can only arrive at a gateway because the source host knows that the destination host can handle such large datagrams (probably because the destination host announced it to the source host in an TCP MSS option).  Thus, the gateway should pass on this large datagram in one piece or in the largest fragments that fit into the next network.

An interesting footnote is that even though the gateways may know about know the 576 rule, it is irrelevant to them.

# Inter-Layer Communication

The Network Driver (ND) or interface should know the Maximum Transmission Unit (MTU) of the directly attached network.

The IP should ask the Network Driver for the Maximum Transmission Unit.

The TCP should ask the IP for the Maximum Datagram Data Size (MDDS). This is the MTU minus the IP header length (MDDS = MTU - IPHdrLen).

When opening a connection TCP can send an MSS option with the value equal MDDS - TCPHdrLen.

TCP should determine the Maximum Segment Data Size (MSDS) from either the default or the received value of the MSS option.

TCP should determine if source fragmentation is possible (by asking the IP) and desirable.

If so TCP may hand to IP segments (including the TCP header) up to MSDS + TCPHdrLen.

If not TCP may hand to IP segments (including the TCP header) up to the lesser of (MSDS + TCPHdrLen) and MDDS.

IP checks the length of data passed to it by TCP.  If the length is less than or equal MDDS, IP attached the IP header and hands it to the ND.  Otherwise the IP must do source fragmentation.

# What is the Default MSS

Another way of asking this question is "What transmitted value for MSS has exactly the same effect of not transmitting the option at all?".

In terms of the previous section:

The default assumption is that the Maximum Transmission Unit is 576 octets.

         MTU = 576

The Maximum Datagram Data Size (MDDS) is the MTU minus the IP header length.

         MDDS = MTU - IPHdrLen = 576 - 20 = 556

When opening a connection TCP can send an MSS option with the value equal MDDS - TCPHdrLen.

         MSS = MDDS - TCPHdrLen = 556 - 20 = 536

TCP should determine the Maximum Segment Data Size (MSDS) from either the default or the received value of the MSS option.

         Default MSS = 536, then MSDS = 536

TCP should determine if source fragmentation is possible and desirable.

If so TCP may hand to IP segments (including the TCP header) up to MSDS + TCPHdrLen (536 + 20 = 556).

If not TCP may hand to IP segments (including the TCP header) up to the lesser of (MSDS + TCPHdrLen (536 + 20 = 556)) and MDDS (556).

# The Truth

IP MSS 和 TCP MSS 之间的关系：

      TCP MSS = IP MDS - 40

If the TCP Maximum Segment Size option is not transmitted then the data sender is allowed to send IP datagrams of maximum size (576) with a minimum IP header (20) and a minimum TCP header (20) and thereby be able to stuff 536 octets of data into each TCP segment.

The definition of the MSS option can be stated:

The maximum number of data octets that may be received by the sender of this TCP option in TCP segments with no TCP header options transmitted in IP datagrams with no IP header options.

# The Consequences

When TCP is used in a situation when either the IP or TCP headers are not minimum and yet the maximum IP datagram that can be received remains 576 octets then the TCP Maximum Segment Size option must be used to reduce the limit on data octets allowed in a TCP segment.

For example, if the IP Security option (11 octets) were in use and the IP maximum datagram size remained at 576 octets, then the TCP should send the MSS with a value of 525 (536-11).