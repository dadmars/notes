# Internet Header Format

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

  IHL:  4 bits

    IP头部长度，单位：32 bit. Note that the minimum value for a correct header is 5.

  Type of Service:  8 bits

  Total Length:  16 bits
    总长度，包含头部和数据，单位：字节。 最大长度为：65,535字节. 所有主机必须接受 576 字节的数据包。

    this size allows a data block of 512 octets plus 64 header
    octets to fit in a datagram.  The maximal internet header is 60
    octets, and a typical internet header is 20 octets, allowing a
    margin for headers of higher level protocols.

  Identification:  16 bits

    An identifying value assigned by the sender to aid in assembling the
    fragments of a datagram.

  Flags:  3 bits

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

  Protocol:  8 bits

    This field indicates the next level protocol used in the data portion of the internet datagram.  

  Header Checksum:  16 bits

    只计算头部，不包含用户数据.  Since some header fields change
    (e.g., time to live), this is recomputed and verified at each point
    that the internet header is processed.

  Source Address:  32 bits
  Destination Address:  32 bits
  Options:  variable

    There are two cases for the format of an option:

      Case 1:  A single octet of option-type.

      Case 2:  An option-type octet, an option-length octet, and the
               actual option-data octets.

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
        0     3     var.  Loose Source Routing. 
        0     9     var.  Strict Source Routing.
        0     7     var.  Record Route.
        0     8      4    Stream ID.  Used to carry the stream identifier.
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

# Addressing

      High Order Bits   Format                           Class
      ---------------   -------------------------------  -----
            0            7 bits of net, 24 bits of host    a
            10          14 bits of net, 16 bits of host    b
            110         21 bits of net,  8 bits of host    c
            111         escape to extended addressing mode

      A value of zero in the network field means this network.  This is
      only used in certain ICMP messages.  The extended addressing mode
      is undefined.

# Errors

    Internet protocol errors may be reported via the ICMP messages [3].
