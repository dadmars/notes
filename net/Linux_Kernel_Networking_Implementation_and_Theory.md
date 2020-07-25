The Network Device
The lower layer, Layer 2 (L2)
struct net_device

- The IRQ number of the device.
- The MTU of the device.
- The MAC address of the device.
- The name of the device (like eth0 or eth1).
- The flags of the device (for example, whether it is up or down).
- A list of multicast addresses associated with the device.
- The promiscuity counter
- The features that the device supports (like GSO or GRO offloading).
- An object of network device callbacks ( net_device_ops object), which consists of function
pointers, such as for opening and stopping a device, starting to transmit, changing the MTU of
the network device, and more.
- An object of ethtool callbacks, which supports getting information about the device by
running the command-line ethtool utility.
- The number of Tx and Rx queues, when the device supports multiqueues.
- The timestamp of the last transmit of a packet on this device.
- The timestamp of the last reception of a packet on this device.

When the promiscuity counter is larger than 0, the network stack does not discard packets that are not destined to the local host. This is used, for example, by packet analyzers (“sniffers”) like tcpdump and wireshark, which open raw sockets in userspace and want to receive also this type of traffic. It is a counter and not a Boolean in order to enable opening several sniffers concurrently: opening each such sniffer increments the counter by 1. When a sniffer is closed, the promiscuity counter is decremented by 1; and if it reaches 0, there are no more sniffers running, and the device exits the promiscuous mode.

 New API (NAPI) in Network Devices
The old network device drivers worked in interrupt-driven mode, which means that for every received packet, there was an interrupt. This proved to be inefficient in terms of performance under high load traffic. A new software technique was developed, called New API (NAPI). With NAPI, under high load, the network device driver works in polling mode and not in interrupt-driven mode. This means that each received packet does not trigger an interrupt. Instead the packets are buffered in the driver, and the kernel polls the driver from time to time to fetch the packets. Using NAPI improves performance under high load. For sockets applications that need the lowest possible latency and are willing to pay a cost of higher CPU utilization, Linux has added a capability for Busy Polling on Sockets

The lookup in the routing subsystem is not the only factor that determines the traversal of a packet in the network stack. For example, there are five points in the network stack where callbacks of the netfilter subsystem can be registered. The first netfilter hook point of a received packet is NF_INET_PRE_ROUTING, before a routing lookup was performed. When a packet is handled by such a callback, which is invoked by a macro named NF_HOOK(), it will continue its traversal in the networking stack according to the result of this callback (also called verdict). For example, if the verdict is NF_DROP, the packet will be discarded, and if the verdict is NF_ACCEPT, the packet will continue its traversal as usual.  The kernel netfilter subsystem is the infrastructure for the well-known iptables userspace package. 

Besides the netfilter hooks, the packet traversal can be influenced by the IPsec subsystem—for example, when it matches a configured IPsec policy. IPsec provides a network layer security solution, and it uses the ESP and the AH protocols.  IPsec has two modes of operation: transport mode and tunnel mode. It is used as a basis for many virtual private network (VPN) solutions, though there are also non-IPsec VPN solutions. 

When a packet is received on the wire, an SKB is allocated by the network device driver.  the skb->data should point to the header of the layer in which it currently resides. When the packet was in L2 skb->data pointed to the L2 (Ethernet) header; now that the packet is going to be moved to Layer 3, skb->data should point to the network (L3) header,

Each SKB has a dev member, which is an instance of the net_device structure. 

I should mention here that theoretically netlink sockets can be used to communicate between two userspace
processes, or more (including sending multicast messages), though this is usually not used, and was not the
original goal of netlink sockets. The UNIX domain sockets provide an API for IPC, and they are widely used for
communication between two userspace processes.

Netlink has some advantages over other ways of communication between userspace and the kernel. For example,
there is no need for polling when working with netlink sockets. A userspace application opens a socket and then calls
recvmsg(), and enters a blocking state if no messages are sent from the kernel; see, for example, the rtnl_listen()
method of the iproute2 package (lib/libnetlink.c). Another advantage is that the kernel can be the initiator of
sending asynchronous messages to userspace, without any need for the userspace to trigger any action (for example,
by calling some IOCTL or by writing to some sysfs entry). Yet another advantage is that netlink sockets support
multicast transmission.

You create netlink sockets from userspace with the socket() system call. The netlink sockets can be SOCK_RAW
sockets or SOCK_DGRAM sockets.

socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE)

The iproute2 package is based upon
netlink sockets, and you’ll see an example of using netlink sockets in iproute2 in the section “Adding and deleting a
routing entry in a routing table”, later in this chapter. I mention the net-tools package, which is older and might be
deprecated in the future, to emphasize that as an alternative to iproute2, it has less power and less abilities.

The iproute2 package includes commands like the following:
•	 ip: For management of network tables and network interfaces
•	 tc: For traffic control management
•	 ss: For dumping socket statistics
•	 lnstat: For dumping linux network statistics
•	 bridge: For management of bridge addresses and devices

The iproute2 package is based mostly on sending requests to the kernel from userspace and getting replies back
over netlink sockets. There are a few exceptions where IOCTLs are used in iproute2. For example, the ip tuntap
command uses IOCTLs to add/remove a TUN/TAP device. If you look at the TUN/TAP software driver code, you’ll
find that it defines some IOCTL handlers, but it does not use the rtnetlink sockets. The net-tools package is based on
IOCTLs and includes known commands like these:
•	 ifconifg
•	 arp
•	 route
•	 netstat
•	 hostname
•	 rarp
Some of the advanced functionalities of the iproute2 package are not available in the net-tools package.


