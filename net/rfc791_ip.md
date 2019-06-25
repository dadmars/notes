连接在一起的计算机之间进行通信，信息被拆分成一个个数据包。IP协议把数据包从一台机器发送到另一台机器。每台机器都有一个固定长度的地址。不同网络，数据包的大小也不同，IP协议要对数据包进行分片和重组。

IP协议只负责尽力发送数据包，但不提供可靠性。对IP协议来说，每一个数据包都是独立的，相互之间没有任何关系。

IP协议被上层协议调用，如TCP。IP协议调用local network协议，将数据包发送到下一个路由器或是目的机器。

IP协议实现两个基本功能：地址和分片。

地址放在数据包的头部，选择数据包到达目的机器经过的路线，称为路由
分片和重组的信息也放在数据包头部

IP模块存在于每个机器中，包含主机和路由器。对地址的处理和对分片重组的处理都使用相同的算法。IP模块还要执行路由选择，尤其是在路由器。

IP协议实现了4个主要机制： Type of Service, Time to Live, Options, and Header Checksum.

* Time to Live 决定数据包的生存时间。它被发送方设定，每经过一个路由器，减1。如果在到达目的机器前变为0，此数据包被丢弃。
* Options 一般不用。包含： provisions for timestamps, security, and special routing.
* Header Checksum 用来效验数据正确性，验证不通过，数据包被丢弃。 
* 错误信息通过ICMP协议发送

#IP模块
   Application                                      Application
   Program                                            Program
         \                                              /
       Internet Module      Internet Module      Internet Module
             \               /       \                /
             LNI-1        LNI-1      LNI-2         LNI-2
                \          /             \          /
              Local Network 1          Local Network 2

##地址
区分三个概念： names, addresses, and routes. A name indicates what we seek.  An address indicates where it is.  A route indicates how to get there.

IP层主要处理 addresses. name到 addresses 的转化由上层协议完成。IP层完成IP地址到 local net addresses的转化。  It is the task of lower level (i.e., local net or gateways) procedures to make the mapping from local net addresses to routes.

IP地址长度为4字节(32 bits). 前面部分为network number，后面部分为local address。一台物理机器可以有多个网卡，每个网卡可以有多个IP地址

#分片
一个数据包可以被设置为"don't fragment."，如果此数据包由于大小原因不能到达目的机器，被丢弃

#Gateways
  Gateways用于在不同的网络之间转发数据包。Gateways also implement the Gateway to Gateway Protocol (GGP) to coordinate routing and other internet control information.

  In a gateway the higher level protocols need not be implemented and
  the GGP functions are added to the IP module.


                   +-------------------------------+
                   | Internet Protocol & ICMP & GGP|
                   +-------------------------------+
                           |                 |
                 +---------------+   +---------------+
                 |   Local Net   |   |   Local Net   |
                 +---------------+   +---------------+

#Internet Header Format

    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |Version|  IHL  |Type of Service|          Total Length         |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |         Identification        |Flags|      Fragment Offset    |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |  Time to Live |    Protocol   |         Header Checksum       |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                       Source Address                          |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                    Destination Address                        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                    Options                    |    Padding    |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

  Version:  4 bits

    The Version field indicates the format of the internet header.  This document describes version 4.

  IHL:  4 bits

    IP头部长度，单位：32 bit words., and thus points to the beginning of the data.  Note that the minimum value for a correct header is 5.

  Type of Service:  8 bits

  Total Length:  16 bits
    总长度，包含头部和数据，单位：字节。 最大长度为：65,535字节. 所有主机必须接受 576 字节的数据包。

    The number 576 is selected to allow a reasonable sized data block to
    be transmitted in addition to the required header information.  For
    example, this size allows a data block of 512 octets plus 64 header
    octets to fit in a datagram.  The maximal internet header is 60
    octets, and a typical internet header is 20 octets, allowing a
    margin for headers of higher level protocols.

  Identification:  16 bits

    An identifying value assigned by the sender to aid in assembling the
    fragments of a datagram.

  Flags:  3 bits

    Various Control Flags.

      Bit 0: reserved, must be zero
      Bit 1: (DF) 0 = May Fragment,  1 = Don't Fragment.
      Bit 2: (MF) 0 = Last Fragment, 1 = More Fragments.

          0   1   2
        +---+---+---+
        |   | D | M |
        | 0 | F | F |
        +---+---+---+

  Fragment Offset:  13 bits

    This field indicates where in the datagram this fragment belongs.
    The fragment offset is measured in units of 8 octets (64 bits).  The
    first fragment has offset zero.

  Time to Live:  8 bits

    This field is modified in internet header processing.  

  Protocol:  8 bits

    This field indicates the next level protocol used in the data portion of the internet datagram.  

  Header Checksum:  16 bits

    A checksum on the header only.  Since some header fields change
    (e.g., time to live), this is recomputed and verified at each point
    that the internet header is processed.

    The checksum algorithm is:

      The checksum field is the 16 bit one's complement of the one's
      complement sum of all 16 bit words in the header.  For purposes of
      computing the checksum, the value of the checksum field is zero.

  Source Address:  32 bits
  Destination Address:  32 bits
  Options:  variable

    The options may appear or not in datagrams.  They must be
    implemented by all IP modules (host and gateways).  What is optional
    is their transmission in any particular datagram, not their
    implementation.

    In some environments the security option may be required in all
    datagrams.

    The option field is variable in length.  There may be zero or more
    options.  There are two cases for the format of an option:

      Case 1:  A single octet of option-type.

      Case 2:  An option-type octet, an option-length octet, and the
               actual option-data octets.

    The option-length octet counts the option-type octet and the
    option-length octet as well as the option-data octets.

    The option-type octet is viewed as having 3 fields:

      1 bit   copied flag,
      2 bits  option class,
      5 bits  option number.

    The copied flag indicates that this option is copied into all
    fragments on fragmentation.

      0 = not copied
      1 = copied

    The option classes are:

      0 = control
      1 = reserved for future use
      2 = debugging and measurement
      3 = reserved for future use

    The following internet options are defined:

      CLASS NUMBER LENGTH DESCRIPTION
      ----- ------ ------ -----------
        0     0      -    End of Option list.  This option occupies only
                          1 octet; it has no length octet.
        0     1      -    No Operation.  This option occupies only 1
                          octet; it has no length octet.
        0     2     11    Security.  Used to carry Security,
                          Compartmentation, User Group (TCC), and
                          Handling Restriction Codes compatible with DOD
                          requirements.
        0     3     var.  Loose Source Routing.  Used to route the
                          internet datagram based on information
                          supplied by the source.
        0     9     var.  Strict Source Routing.  Used to route the
                          internet datagram based on information
                          supplied by the source.
        0     7     var.  Record Route.  Used to trace the route an
                          internet datagram takes.
        0     8      4    Stream ID.  Used to carry the stream
                          identifier.
        2     4     var.  Internet Timestamp.

    Specific Option Definitions

      End of Option List

        +--------+
        |00000000|
        +--------+
          Type=0

        This option indicates the end of the option list.  This might
        not coincide with the end of the internet header according to
        the internet header length.  This is used at the end of all
        options, not the end of each option, and need only be used if
        the end of the options would not otherwise coincide with the end
        of the internet header.

        May be copied, introduced, or deleted on fragmentation, or for
        any other reason.

      No Operation

        +--------+
        |00000001|
        +--------+
          Type=1

        This option may be used between options, for example, to align
        the beginning of a subsequent option on a 32 bit boundary.

        May be copied, introduced, or deleted on fragmentation, or for
        any other reason.

      Security

        This option provides a way for hosts to send security,
        compartmentation, handling restrictions, and TCC (closed user
        group) parameters.  The format for this option is as follows:

          +--------+--------+---//---+---//---+---//---+---//---+
          |10000010|00001011|SSS  SSS|CCC  CCC|HHH  HHH|  TCC   |
          +--------+--------+---//---+---//---+---//---+---//---+
           Type=130 Length=11

        Security (S field):  16 bits

          Specifies one of 16 levels of security (eight of which are
          reserved for future use).

            00000000 00000000 - Unclassified
            11110001 00110101 - Confidential
            01111000 10011010 - EFTO
            10111100 01001101 - MMMM
            01011110 00100110 - PROG
            10101111 00010011 - Restricted
            11010111 10001000 - Secret
            01101011 11000101 - Top Secret
            00110101 11100010 - (Reserved for future use)
            10011010 11110001 - (Reserved for future use)
            01001101 01111000 - (Reserved for future use)
            00100100 10111101 - (Reserved for future use)
            00010011 01011110 - (Reserved for future use)
            10001001 10101111 - (Reserved for future use)
            11000100 11010110 - (Reserved for future use)
            11100010 01101011 - (Reserved for future use)

        Compartments (C field):  16 bits

          An all zero value is used when the information transmitted is
          not compartmented.  Other values for the compartments field
          may be obtained from the Defense Intelligence Agency.

        Handling Restrictions (H field):  16 bits

          The values for the control and release markings are
          alphanumeric digraphs and are defined in the Defense
          Intelligence Agency Manual DIAM 65-19, "Standard Security
          Markings".

        Transmission Control Code (TCC field):  24 bits

          Provides a means to segregate traffic and define controlled
          communities of interest among subscribers. The TCC values are
          trigraphs, and are available from HQ DCA Code 530.

        Must be copied on fragmentation.  This option appears at most
        once in a datagram.

      Loose Source and Record Route

        +--------+--------+--------+---------//--------+
        |10000011| length | pointer|     route data    |
        +--------+--------+--------+---------//--------+
         Type=131

        The loose source and record route (LSRR) option provides a means
        for the source of an internet datagram to supply routing
        information to be used by the gateways in forwarding the
        datagram to the destination, and to record the route
        information.

        The option begins with the option type code.  The second octet
        is the option length which includes the option type code and the
        length octet, the pointer octet, and length-3 octets of route
        data.  The third octet is the pointer into the route data
        indicating the octet which begins the next source address to be
        processed.  The pointer is relative to this option, and the
        smallest legal value for the pointer is 4.

        A route data is composed of a series of internet addresses.
        Each internet address is 32 bits or 4 octets.  If the pointer is
        greater than the length, the source route is empty (and the
        recorded route full) and the routing is to be based on the
        destination address field.

        If the address in destination address field has been reached and
        the pointer is not greater than the length, the next address in
        the source route replaces the address in the destination address
        field, and the recorded route address replaces the source
        address just used, and pointer is increased by four.

        The recorded route address is the internet module's own internet
        address as known in the environment into which this datagram is
        being forwarded.

        This procedure of replacing the source route with the recorded
        route (though it is in the reverse of the order it must be in to
        be used as a source route) means the option (and the IP header
        as a whole) remains a constant length as the datagram progresses
        through the internet.

        This option is a loose source route because the gateway or host
        IP is allowed to use any route of any number of other
        intermediate gateways to reach the next address in the route.

        Must be copied on fragmentation.  Appears at most once in a
        datagram.

      Strict Source and Record Route

        +--------+--------+--------+---------//--------+
        |10001001| length | pointer|     route data    |
        +--------+--------+--------+---------//--------+
         Type=137

        The strict source and record route (SSRR) option provides a
        means for the source of an internet datagram to supply routing
        information to be used by the gateways in forwarding the
        datagram to the destination, and to record the route
        information.

        The option begins with the option type code.  The second octet
        is the option length which includes the option type code and the
        length octet, the pointer octet, and length-3 octets of route
        data.  The third octet is the pointer into the route data
        indicating the octet which begins the next source address to be
        processed.  The pointer is relative to this option, and the
        smallest legal value for the pointer is 4.

        A route data is composed of a series of internet addresses.
        Each internet address is 32 bits or 4 octets.  If the pointer is
        greater than the length, the source route is empty (and the
        recorded route full) and the routing is to be based on the
        destination address field.

        If the address in destination address field has been reached and
        the pointer is not greater than the length, the next address in
        the source route replaces the address in the destination address
        field, and the recorded route address replaces the source
        address just used, and pointer is increased by four.

        The recorded route address is the internet module's own internet
        address as known in the environment into which this datagram is
        being forwarded.

        This procedure of replacing the source route with the recorded
        route (though it is in the reverse of the order it must be in to
        be used as a source route) means the option (and the IP header
        as a whole) remains a constant length as the datagram progresses
        through the internet.

        This option is a strict source route because the gateway or host
        IP must send the datagram directly to the next address in the
        source route through only the directly connected network
        indicated in the next address to reach the next gateway or host
        specified in the route.

        Must be copied on fragmentation.  Appears at most once in a
        datagram.

      Record Route

        +--------+--------+--------+---------//--------+
        |00000111| length | pointer|     route data    |
        +--------+--------+--------+---------//--------+
          Type=7

        The record route option provides a means to record the route of
        an internet datagram.

        The option begins with the option type code.  The second octet
        is the option length which includes the option type code and the
        length octet, the pointer octet, and length-3 octets of route
        data.  The third octet is the pointer into the route data
        indicating the octet which begins the next area to store a route
        address.  The pointer is relative to this option, and the
        smallest legal value for the pointer is 4.

        A recorded route is composed of a series of internet addresses.
        Each internet address is 32 bits or 4 octets.  If the pointer is
        greater than the length, the recorded route data area is full.
        The originating host must compose this option with a large
        enough route data area to hold all the address expected.  The
        size of the option does not change due to adding addresses.  The
        intitial contents of the route data area must be zero.

        When an internet module routes a datagram it checks to see if
        the record route option is present.  If it is, it inserts its
        own internet address as known in the environment into which this
        datagram is being forwarded into the recorded route begining at
        the octet indicated by the pointer, and increments the pointer
        by four.

        If the route data area is already full (the pointer exceeds the
        length) the datagram is forwarded without inserting the address
        into the recorded route.  If there is some room but not enough
        room for a full address to be inserted, the original datagram is
        considered to be in error and is discarded.  In either case an
        ICMP parameter problem message may be sent to the source
        host [3].

        Not copied on fragmentation, goes in first fragment only.
        Appears at most once in a datagram.

      Stream Identifier

        +--------+--------+--------+--------+
        |10001000|00000010|    Stream ID    |
        +--------+--------+--------+--------+
         Type=136 Length=4

        This option provides a way for the 16-bit SATNET stream
        identifier to be carried through networks that do not support
        the stream concept.

        Must be copied on fragmentation.  Appears at most once in a
        datagram.

      Internet Timestamp

        +--------+--------+--------+--------+
        |01000100| length | pointer|oflw|flg|
        +--------+--------+--------+--------+
        |         internet address          |
        +--------+--------+--------+--------+
        |             timestamp             |
        +--------+--------+--------+--------+
        |                 .                 |
                          .
                          .
        Type = 68

        The Option Length is the number of octets in the option counting
        the type, length, pointer, and overflow/flag octets (maximum
        length 40).

        The Pointer is the number of octets from the beginning of this
        option to the end of timestamps plus one (i.e., it points to the
        octet beginning the space for next timestamp).  The smallest
        legal value is 5.  The timestamp area is full when the pointer
        is greater than the length.

        The Overflow (oflw) [4 bits] is the number of IP modules that
        cannot register timestamps due to lack of space.

        The Flag (flg) [4 bits] values are

          0 -- time stamps only, stored in consecutive 32-bit words,

          1 -- each timestamp is preceded with internet address of the
               registering entity,

          3 -- the internet address fields are prespecified.  An IP
               module only registers its timestamp if it matches its own
               address with the next specified internet address.

        The Timestamp is a right-justified, 32-bit timestamp in
        milliseconds since midnight UT.  If the time is not available in
        milliseconds or cannot be provided with respect to midnight UT
        then any time may be inserted as a timestamp provided the high
        order bit of the timestamp field is set to one to indicate the
        use of a non-standard value.

        The originating host must compose this option with a large
        enough timestamp data area to hold all the timestamp information
        expected.  The size of the option does not change due to adding
        timestamps.  The intitial contents of the timestamp data area
        must be zero or internet address/zero pairs.

        If the timestamp data area is already full (the pointer exceeds
        the length) the datagram is forwarded without inserting the
        timestamp, but the overflow count is incremented by one.

        If there is some room but not enough room for a full timestamp
        to be inserted, or the overflow count itself overflows, the
        original datagram is considered to be in error and is discarded.
        In either case an ICMP parameter problem message may be sent to
        the source host [3].

        The timestamp option is not copied upon fragmentation.  It is
        carried in the first fragment.  Appears at most once in a
        datagram.

  Padding:  variable

    The internet header padding is used to ensure that the internet
    header ends on a 32 bit boundary.  The padding is zero.

#Discussion
  Addressing

    To provide for flexibility in assigning address to networks and
    allow for the  large number of small to intermediate sized networks
    the interpretation of the address field is coded to specify a small
    number of networks with a large number of host, a moderate number of
    networks with a moderate number of hosts, and a large number of
    networks with a small number of hosts.  In addition there is an
    escape code for extended addressing mode.

    Address Formats:

      High Order Bits   Format                           Class
      ---------------   -------------------------------  -----
            0            7 bits of net, 24 bits of host    a
            10          14 bits of net, 16 bits of host    b
            110         21 bits of net,  8 bits of host    c
            111         escape to extended addressing mode

      A value of zero in the network field means this network.  This is
      only used in certain ICMP messages.  The extended addressing mode
      is undefined.  Both of these features are reserved for future use.

    The actual values assigned for network addresses is given in
    "Assigned Numbers" [9].

    The local address, assigned by the local network, must allow for a
    single physical host to act as several distinct internet hosts.
    That is, there must be a mapping between internet host addresses and
    network/host interfaces that allows several internet addresses to
    correspond to one interface.  It must also be allowed for a host to
    have several physical interfaces and to treat the datagrams from
    several of them as if they were all addressed to a single host.

    Address mappings between internet addresses and addresses for
    ARPANET, SATNET, PRNET, and other networks are described in "Address
    Mappings" [5].

  Fragmentation and Reassembly.

    The internet identification field (ID) is used together with the
    source and destination address, and the protocol fields, to identify
    datagram fragments for reassembly.

    The More Fragments flag bit (MF) is set if the datagram is not the
    last fragment.  The Fragment Offset field identifies the fragment
    location, relative to the beginning of the original unfragmented
    datagram.  Fragments are counted in units of 8 octets.  The
    fragmentation strategy is designed so than an unfragmented datagram
    has all zero fragmentation information (MF = 0, fragment offset =
    0).  If an internet datagram is fragmented, its data portion must be
    broken on 8 octet boundaries.

    This format allows 2**13 = 8192 fragments of 8 octets each for a
    total of 65,536 octets.  Note that this is consistent with the the
    datagram total length field (of course, the header is counted in the
    total length and not in the fragments).

    When fragmentation occurs, some options are copied, but others
    remain with the first fragment only.

    Every internet module must be able to forward a datagram of 68
    octets without further fragmentation.  This is because an internet
    header may be up to 60 octets, and the minimum fragment is 8 octets.

    Every internet destination must be able to receive a datagram of 576
    octets either in one piece or in fragments to be reassembled.

    The fields which may be affected by fragmentation include:

      (1) options field
      (2) more fragments flag
      (3) fragment offset
      (4) internet header length field
      (5) total length field
      (6) header checksum

    If the Don't Fragment flag (DF) bit is set, then internet
    fragmentation of this datagram is NOT permitted, although it may be
    discarded.  This can be used to prohibit fragmentation in cases
    where the receiving host does not have sufficient resources to
    reassemble internet fragments.

    One example of use of the Don't Fragment feature is to down line
    load a small host.  A small host could have a boot strap program
    that accepts a datagram stores it in memory and then executes it.

    The fragmentation and reassembly procedures are most easily
    described by examples.  The following procedures are example
    implementations.

    General notation in the following pseudo programs: "=<" means "less
    than or equal", "#" means "not equal", "=" means "equal", "<-" means
    "is set to".  Also, "x to y" includes x and excludes y; for example,
    "4 to 7" would include 4, 5, and 6 (but not 7).

    An Example Fragmentation Procedure

      The maximum sized datagram that can be transmitted through the
      next network is called the maximum transmission unit (MTU).

      If the total length is less than or equal the maximum transmission
      unit then submit this datagram to the next step in datagram
      processing; otherwise cut the datagram into two fragments, the
      first fragment being the maximum size, and the second fragment
      being the rest of the datagram.  The first fragment is submitted
      to the next step in datagram processing, while the second fragment
      is submitted to this procedure in case it is still too large.

      Notation:

        FO    -  Fragment Offset
        IHL   -  Internet Header Length
        DF    -  Don't Fragment flag
        MF    -  More Fragments flag
        TL    -  Total Length
        OFO   -  Old Fragment Offset
        OIHL  -  Old Internet Header Length
        OMF   -  Old More Fragments flag
        OTL   -  Old Total Length
        NFB   -  Number of Fragment Blocks
        MTU   -  Maximum Transmission Unit

      Procedure:

        IF TL =< MTU THEN Submit this datagram to the next step
             in datagram processing ELSE IF DF = 1 THEN discard the
        datagram ELSE
        To produce the first fragment:
        (1)  Copy the original internet header;
        (2)  OIHL <- IHL; OTL <- TL; OFO <- FO; OMF <- MF;
        (3)  NFB <- (MTU-IHL*4)/8;
        (4)  Attach the first NFB*8 data octets;
        (5)  Correct the header:
             MF <- 1;  TL <- (IHL*4)+(NFB*8);
             Recompute Checksum;
        (6)  Submit this fragment to the next step in
             datagram processing;
        To produce the second fragment:
        (7)  Selectively copy the internet header (some options
             are not copied, see option definitions);
        (8)  Append the remaining data;
        (9)  Correct the header:
             IHL <- (((OIHL*4)-(length of options not copied))+3)/4;
             TL <- OTL - NFB*8 - (OIHL-IHL)*4);
             FO <- OFO + NFB;  MF <- OMF;  Recompute Checksum;
        (10) Submit this fragment to the fragmentation test; DONE.

      In the above procedure each fragment (except the last) was made
      the maximum allowable size.  An alternative might produce less
      than the maximum size datagrams.  For example, one could implement
      a fragmentation procedure that repeatly divided large datagrams in
      half until the resulting fragments were less than the maximum
      transmission unit size.

    An Example Reassembly Procedure

      For each datagram the buffer identifier is computed as the
      concatenation of the source, destination, protocol, and
      identification fields.  If this is a whole datagram (that is both
      the fragment offset and the more fragments  fields are zero), then
      any reassembly resources associated with this buffer identifier
      are released and the datagram is forwarded to the next step in
      datagram processing.

      If no other fragment with this buffer identifier is on hand then
      reassembly resources are allocated.  The reassembly resources
      consist of a data buffer, a header buffer, a fragment block bit
      table, a total data length field, and a timer.  The data from the
      fragment is placed in the data buffer according to its fragment
      offset and length, and bits are set in the fragment block bit
      table corresponding to the fragment blocks received.

      If this is the first fragment (that is the fragment offset is
      zero)  this header is placed in the header buffer.  If this is the
      last fragment ( that is the more fragments field is zero) the
      total data length is computed.  If this fragment completes the
      datagram (tested by checking the bits set in the fragment block
      table), then the datagram is sent to the next step in datagram
      processing; otherwise the timer is set to the maximum of the
      current timer value and the value of the time to live field from
      this fragment; and the reassembly routine gives up control.

      If the timer runs out, the all reassembly resources for this
      buffer identifier are released.  The initial setting of the timer
      is a lower bound on the reassembly waiting time.  This is because
      the waiting time will be increased if the Time to Live in the
      arriving fragment is greater than the current timer value but will
      not be decreased if it is less.  The maximum this timer value
      could reach is the maximum time to live (approximately 4.25
      minutes).  The current recommendation for the initial timer
      setting is 15 seconds.  This may be changed as experience with
      this protocol accumulates.  Note that the choice of this parameter
      value is related to the buffer capacity available and the data
      rate of the transmission medium; that is, data rate times timer
      value equals buffer size (e.g., 10Kb/s X 15s = 150Kb).

      Notation:

        FO    -  Fragment Offset
        IHL   -  Internet Header Length
        MF    -  More Fragments flag
        TTL   -  Time To Live
        NFB   -  Number of Fragment Blocks
        TL    -  Total Length
        TDL   -  Total Data Length
        BUFID -  Buffer Identifier
        RCVBT -  Fragment Received Bit Table
        TLB   -  Timer Lower Bound

      Procedure:

        (1)  BUFID <- source|destination|protocol|identification;
        (2)  IF FO = 0 AND MF = 0
        (3)     THEN IF buffer with BUFID is allocated
        (4)             THEN flush all reassembly for this BUFID;
        (5)          Submit datagram to next step; DONE.
        (6)     ELSE IF no buffer with BUFID is allocated
        (7)             THEN allocate reassembly resources
                             with BUFID;
                             TIMER <- TLB; TDL <- 0;
        (8)          put data from fragment into data buffer with
                     BUFID from octet FO*8 to
                                         octet (TL-(IHL*4))+FO*8;
        (9)          set RCVBT bits from FO
                                        to FO+((TL-(IHL*4)+7)/8);
        (10)         IF MF = 0 THEN TDL <- TL-(IHL*4)+(FO*8)
        (11)         IF FO = 0 THEN put header in header buffer
        (12)         IF TDL # 0
        (13)          AND all RCVBT bits from 0
                                             to (TDL+7)/8 are set
        (14)            THEN TL <- TDL+(IHL*4)
        (15)                 Submit datagram to next step;
        (16)                 free all reassembly resources
                             for this BUFID; DONE.
        (17)         TIMER <- MAX(TIMER,TTL);
        (18)         give up until next fragment or timer expires;
        (19) timer expires: flush all reassembly with this BUFID; DONE.

      In the case that two or more fragments contain the same data
      either identically or through a partial overlap, this procedure
      will use the more recently arrived copy in the data buffer and
      datagram delivered.

  Identification

    The choice of the Identifier for a datagram is based on the need to
    provide a way to uniquely identify the fragments of a particular
    datagram.  The protocol module assembling fragments judges fragments
    to belong to the same datagram if they have the same source,
    destination, protocol, and Identifier.  Thus, the sender must choose
    the Identifier to be unique for this source, destination pair and
    protocol for the time the datagram (or any fragment of it) could be
    alive in the internet.

    It seems then that a sending protocol module needs to keep a table
    of Identifiers, one entry for each destination it has communicated
    with in the last maximum packet lifetime for the internet.

    However, since the Identifier field allows 65,536 different values,
    some host may be able to simply use unique identifiers independent
    of destination.

    It is appropriate for some higher level protocols to choose the
    identifier. For example, TCP protocol modules may retransmit an
    identical TCP segment, and the probability for correct reception
    would be enhanced if the retransmission carried the same identifier
    as the original transmission since fragments of either datagram
    could be used to construct a correct TCP segment.

  Type of Service

    The type of service (TOS) is for internet service quality selection.
    The type of service is specified along the abstract parameters
    precedence, delay, throughput, and reliability.  These abstract
    parameters are to be mapped into the actual service parameters of
    the particular networks the datagram traverses.

    Precedence.  An independent measure of the importance of this
    datagram.

    Delay.  Prompt delivery is important for datagrams with this
    indication.

    Throughput.  High data rate is important for datagrams with this
    indication.

    Reliability.  A higher level of effort to ensure delivery is
    important for datagrams with this indication.

    For example, the ARPANET has a priority bit, and a choice between
    "standard" messages (type 0) and "uncontrolled" messages (type 3),
    (the choice between single packet and multipacket messages can also
    be considered a service parameter). The uncontrolled messages tend
    to be less reliably delivered and suffer less delay.  Suppose an
    internet datagram is to be sent through the ARPANET.  Let the
    internet type of service be given as:

      Precedence:    5
      Delay:         0
      Throughput:    1
      Reliability:   1

    In this example, the mapping of these parameters to those available
    for the ARPANET would be  to set the ARPANET priority bit on since
    the Internet precedence is in the upper half of its range, to select
    standard messages since the throughput and reliability requirements
    are indicated and delay is not.  More details are given on service
    mappings in "Service Mappings" [8].

  Time to Live

    The time to live is set by the sender to the maximum time the
    datagram is allowed to be in the internet system.  If the datagram
    is in the internet system longer than the time to live, then the
    datagram must be destroyed.

    This field must be decreased at each point that the internet header
    is processed to reflect the time spent processing the datagram.
    Even if no local information is available on the time actually
    spent, the field must be decremented by 1.  The time is measured in
    units of seconds (i.e. the value 1 means one second).  Thus, the
    maximum time to live is 255 seconds or 4.25 minutes.  Since every
    module that processes a datagram must decrease the TTL by at least
    one even if it process the datagram in less than a second, the TTL
    must be thought of only as an upper bound on the time a datagram may
    exist.  The intention is to cause undeliverable datagrams to be
    discarded, and to bound the maximum datagram lifetime.

    Some higher level reliable connection protocols are based on
    assumptions that old duplicate datagrams will not arrive after a
    certain time elapses.  The TTL is a way for such protocols to have
    an assurance that their assumption is met.

  Options

    The options are optional in each datagram, but required in
    implementations.  That is, the presence or absence of an option is
    the choice of the sender, but each internet module must be able to
    parse every option.  There can be several options present in the
    option field.

    The options might not end on a 32-bit boundary.  The internet header
    must be filled out with octets of zeros.  The first of these would
    be interpreted as the end-of-options option, and the remainder as
    internet header padding.

    Every internet module must be able to act on every option.  The
    Security Option is required if classified, restricted, or
    compartmented traffic is to be passed.

  Checksum

    The internet header checksum is recomputed if the internet header is
    changed.  For example, a reduction of the time to live, additions or
    changes to internet options, or due to fragmentation.  This checksum
    at the internet level is intended to protect the internet header
    fields from transmission errors.

    There are some applications where a few data bit errors are
    acceptable while retransmission delays are not.  If the internet
    protocol enforced data correctness such applications could not be
    supported.

  Errors

    Internet protocol errors may be reported via the ICMP messages [3].

3.3.  Interfaces

  The functional description of user interfaces to the IP is, at best,
  fictional, since every operating system will have different
  facilities.  Consequently, we must warn readers that different IP
  implementations may have different user interfaces.  However, all IPs
  must provide a certain minimum  set of services to guarantee that all
  IP implementations can support the same protocol hierarchy.  This
  section specifies the functional interfaces required of all IP
  implementations.

  Internet protocol interfaces on one side to the local network and on
  the other side to either a higher level protocol or an application
  program.  In the following, the higher level protocol or application
  program (or even a gateway program) will be called the "user" since it
  is using the internet module.  Since internet protocol is a datagram
  protocol, there is minimal memory or state maintained between datagram
  transmissions, and each call on the internet protocol module by the
  user supplies all information necessary for the IP to perform the
  service requested.

  An Example Upper Level Interface

  The following two example calls satisfy the requirements for the user
  to internet protocol module communication ("=>" means returns):

  SEND (src, dst, prot, TOS, TTL, BufPTR, len, Id, DF, opt => result)

    where:

      src = source address
      dst = destination address
      prot = protocol
      TOS = type of service
      TTL = time to live
      BufPTR = buffer pointer
      len = length of buffer
      Id  = Identifier
      DF = Don't Fragment
      opt = option data
      result = response
        OK = datagram sent ok
        Error = error in arguments or local network error

    Note that the precedence is included in the TOS and the
    security/compartment is passed as an option.

  RECV (BufPTR, prot, => result, src, dst, TOS, len, opt)

    where:

      BufPTR = buffer pointer
      prot = protocol
      result = response
        OK = datagram received ok
        Error = error in arguments
      len = length of buffer
      src = source address
      dst = destination address
      TOS = type of service
      opt = option data

  When the user sends a datagram, it executes the SEND call supplying
  all the arguments.  The internet protocol module, on receiving this
  call, checks the arguments and prepares and sends the message.  If the
  arguments are good and the datagram is accepted by the local network,
  the call returns successfully.  If either the arguments are bad, or
  the datagram is not accepted by the local network, the call returns
  unsuccessfully.  On unsuccessful returns, a reasonable report must be
  made as to the cause of the problem, but the details of such reports
  are up to individual implementations.

  When a datagram arrives at the internet protocol module from the local
  network, either there is a pending RECV call from the user addressed
  or there is not.  In the first case, the pending call is satisfied by
  passing the information from the datagram to the user.  In the second
  case, the user addressed is notified of a pending datagram.  If the
  user addressed does not exist, an ICMP error message is returned to
  the sender, and the data is discarded.

  The notification of a user may be via a pseudo interrupt or similar
  mechanism, as appropriate in the particular operating system
  environment of the implementation.

  A user's RECV call may then either be immediately satisfied by a
  pending datagram, or the call may be pending until a datagram arrives.

  The source address is included in the send call in case the sending
  host has several addresses (multiple physical connections or logical
  addresses).  The internet module must check to see that the source
  address is one of the legal address for this host.

  An implementation may also allow or require a call to the internet
  module to indicate interest in or reserve exclusive use of a class of
  datagrams (e.g., all those with a certain value in the protocol
  field).

  This section functionally characterizes a USER/IP interface.  The
  notation used is similar to most procedure of function calls in high
  level languages, but this usage is not meant to rule out trap type
  service calls (e.g., SVCs, UUOs, EMTs), or any other form of
  interprocess communication.

APPENDIX A:  Examples & Scenarios

Example 1:

  This is an example of the minimal data carrying internet datagram:


    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |Ver= 4 |IHL= 5 |Type of Service|        Total Length = 21      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |      Identification = 111     |Flg=0|   Fragment Offset = 0   |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |   Time = 123  |  Protocol = 1 |        header checksum        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                         source address                        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                      destination address                      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |     data      |
   +-+-+-+-+-+-+-+-+

                       Example Internet Datagram

                               Figure 5.

  Note that each tick mark represents one bit position.

  This is a internet datagram in version 4 of internet protocol; the
  internet header consists of five 32 bit words, and the total length of
  the datagram is 21 octets.  This datagram is a complete datagram (not
  a fragment).

Example 2:

  In this example, we show first a moderate size internet datagram (452
  data octets), then two internet fragments that might result from the
  fragmentation of this datagram if the maximum sized transmission
  allowed were 280 octets.


    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |Ver= 4 |IHL= 5 |Type of Service|       Total Length = 472      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |     Identification = 111      |Flg=0|     Fragment Offset = 0 |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |   Time = 123  | Protocol = 6  |        header checksum        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                         source address                        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                      destination address                      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                             data                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                             data                              |
   \                                                               \
   \                                                               \
   |                             data                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |             data              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                       Example Internet Datagram

                               Figure 6.

  Now the first fragment that results from splitting the datagram after
  256 data octets.


    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |Ver= 4 |IHL= 5 |Type of Service|       Total Length = 276      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |     Identification = 111      |Flg=1|     Fragment Offset = 0 |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |   Time = 119  | Protocol = 6  |        Header Checksum        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                         source address                        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                      destination address                      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                             data                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                             data                              |
   \                                                               \
   \                                                               \
   |                             data                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                             data                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                       Example Internet Fragment

                               Figure 7.

  And the second fragment.


    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |Ver= 4 |IHL= 5 |Type of Service|       Total Length = 216      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |     Identification = 111      |Flg=0|  Fragment Offset  =  32 |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |   Time = 119  | Protocol = 6  |        Header Checksum        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                         source address                        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                      destination address                      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                             data                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                             data                              |
   \                                                               \
   \                                                               \
   |                             data                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |            data               |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                       Example Internet Fragment

                               Figure 8.


Example 3:

  Here, we show an example of a datagram containing options:


    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |Ver= 4 |IHL= 8 |Type of Service|       Total Length = 576      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |       Identification = 111    |Flg=0|     Fragment Offset = 0 |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |   Time = 123  |  Protocol = 6 |       Header Checksum         |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                        source address                         |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                      destination address                      |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   | Opt. Code = x | Opt.  Len.= 3 | option value  | Opt. Code = x |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   | Opt. Len. = 4 |           option value        | Opt. Code = 1 |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   | Opt. Code = y | Opt. Len. = 3 |  option value | Opt. Code = 0 |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                             data                              |
   \                                                               \
   \                                                               \
   |                             data                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                             data                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                       Example Internet Datagram

                               Figure 9.

APPENDIX B:  Data Transmission Order

The order of transmission of the header and data described in this
document is resolved to the octet level.  Whenever a diagram shows a
group of octets, the order of transmission of those octets is the normal
order in which they are read in English.  For example, in the following
diagram the octets are transmitted in the order they are numbered.


    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |       1       |       2       |       3       |       4       |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |       5       |       6       |       7       |       8       |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |       9       |      10       |      11       |      12       |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                      Transmission Order of Bytes

                               Figure 10.

Whenever an octet represents a numeric quantity the left most bit in the
diagram is the high order or most significant bit.  That is, the bit
labeled 0 is the most significant bit.  For example, the following
diagram represents the value 170 (decimal).


                            0 1 2 3 4 5 6 7
                           +-+-+-+-+-+-+-+-+
                           |1 0 1 0 1 0 1 0|
                           +-+-+-+-+-+-+-+-+

                          Significance of Bits

                               Figure 11.

Similarly, whenever a multi-octet field represents a numeric quantity
the left most bit of the whole field is the most significant bit.  When
a multi-octet quantity is transmitted the most significant octet is
transmitted first.