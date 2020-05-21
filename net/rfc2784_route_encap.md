# Generic Routing Encapsulation (GRE)

数据首先在一个 GRE 包里被封装. 然后 GRE packet 被其它协议封装并转发(delivery protocol) 

# Structure of a GRE Encapsulated Packet

    ---------------------------------
    |                               |
    |       Delivery Header         |
    |                               |
    ---------------------------------
    |                               |
    |       GRE Header              |
    |                               |
    ---------------------------------
    |                               |
    |       Payload packet          |
    |                               |
    ---------------------------------

## GRE Header

    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    |C|       Reserved0       | Ver |         Protocol Type         |
    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    |      Checksum (optional)      |       Reserved1 (Optional)    |
    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

## Checksum Present (bit 0)

If the Checksum Present bit is set to one, then the Checksum and the Reserved1 fields are present and the Checksum field contains valid information. Note that a compliant implementation MUST accept and process this field.

## Reserved0 (bits 1-12)

A receiver MUST discard a packet where any of bits 1-5 are non-zero, unless that receiver implements RFC 1701. Bits 6-12 are reserved for future use. These bits MUST be sent as zero and MUST be ignored on receipt.

## Version Number (bits 13-15)

The Version Number field MUST contain the value zero.

## Protocol Type (2 octets)

The Protocol Type field contains the protocol type of the payload packet. These Protocol Types are defined in [RFC1700] as "ETHER TYPES" and in [ETYPES]. An implementation receiving a packet containing a Protocol Type which is not listed in [RFC1700] or [ETYPES] SHOULD discard the packet.

## Checksum (2 octets)

The Checksum field contains the IP (one's complement) checksum sum of the all the 16 bit words in the GRE header and the payload packet. For purposes of computing the checksum, the value of the checksum field is zero. This field is present only if the Checksum Present bit is set to one.

## Reserved1 (2 octets)

The Reserved1 field is reserved for future use, and if present, MUST be transmitted as zero. The Reserved1 field is present only when the Checksum field is present (that is, Checksum Present bit is set to one).

# IPv4 as a Payload

When IPv4 is being carried as the GRE payload, the Protocol Type field MUST be set to 0x800.

## Forwarding Decapsulated IPv4 Payload Packets

When a tunnel endpoint decapsulates a GRE packet which has an IPv4 packet as the payload, the destination address in the IPv4 payload packet header MUST be used to forward the packet and the TTL of the payload packet MUST be decremented. Care should be taken when forwarding such a packet, since if the destination address of the payload packet is the encapsulator of the packet (i.e., the other end of the tunnel), looping can occur. In this case, the packet MUST be discarded.

# IPv4 as a Delivery Protocol

 The IPv4 protocol 47 [RFC1700] is used when GRE packets are enapsulated in IPv4. See [RFC1122] for requirements relating to the delivery of packets over IPv4 networks.

# Interoperation with RFC 1701 Compliant Implementations

 In RFC 1701, the field described here as Reserved0 contained a number of flag bits which this specification deprecates. In particular, the Routing Present, Key Present, Sequence Number Present, and Strict Source Route bits have been deprecated, along with the Recursion Control field. As a result, the GRE header will never contain the Key, Sequence Number or Routing fields specified in RFC 1701.

There are, however, existing implementations of RFC 1701. The following sections describe correct interoperation with such implementations.

## RFC 1701 Compliant Receiver

An implementation complying to this specification will transmit the Reserved0 field set to zero. An RFC 1701 compliant receiver will interpret this as having the Routing Present, Key Present, Sequence Number Present, and Strict Source Route bits set to zero, and will not expect the RFC 1701 Key, Sequence Number or Routing fields to be present.

## RFC 1701 Compliant Transmitter

An RFC 1701 transmitter may set any of the Routing Present, Key Present, Sequence Number Present, and Strict Source Route bits set to one, and thus may transmit the RFC 1701 Key, Sequence Number or Routing fields in the GRE header. As stated in Section 5.3, a packet with non-zero bits in any of bits 1-5 MUST be discarded unless the receiver implements RFC 1701.

# Security Considerations

Security in a network using GRE should be relatively similar to security in a normal IPv4 network, as routing using GRE follows the same routing that IPv4 uses natively. Route filtering will remain unchanged. However packet filtering requires either that a firewall look inside the GRE packet or that the filtering is done on the GRE tunnel endpoints. In those environments in which this is considered to be a security issue it may be desirable to terminate the tunnel at the firewall.

# IANA Considerations

 This section considers the assignment of additional GRE Version Numbers and Protocol Types.

## GRE Version Numbers

This document specifies GRE version number 0. GRE version number 1 is used by PPTP [RFC2637]. Additional GRE version numbers are assigned by IETF Consensus as defined in RFC 2434 [RFC2434].

## Protocol Types

GRE uses an ETHER Type for the Protocol Type. New ETHER TYPES are assigned by Xerox Systems Institute [RFC1700].