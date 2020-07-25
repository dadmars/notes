# network command

- [ip](#ip)
- [ss](#ss)
- [nc](#nc)
- [curl](#curl)
- [ssh](#ssh)
- [tcpdump](#tcpdump)
- [hostname](#hostname)
- [nmap](#nmap)
- [iptables](#iptables)

tc, lnstat, bridge

## ip

显示 ip 地址

```bash
ip a
ip -4 a  # 只显示 ip4
ip -4 a show dev wlo1  # 只显示 wlo1
```

增加 ip 地址

```bash
sudo ip a add 192.168.1.9/24 dev wlo1
```

删除 ip 地址

```bash
sudo ip a del 192.168.1.9/24 dev wlo1
```

显示设备

```bash
ip link
ip link show wlo1
```

关闭设备

```bash
sudo ip link set wlo1 down
```

打开设备

```bash
sudo ip link set wlo1 up
```

设置设备参数

```bash
sudo ip link set wlo1 mtu 1000
sudo ip link set wlo1 name abc
```

显示路由

```bash
ip route
ip route list 192.168.1.0/24
```

添加路由

```bash
sudo ip route add 192.168.1.0/24 dev wlo1
```

删除路由

```bash
sudo ip route delete 192.168.1.0/24 dev wlo1
```

show / manipulate routing, devices, policy routing and tunnels

       ip [ OPTIONS ] OBJECT { COMMAND | help }

       ip [ -force ] -batch filename

       OBJECT := { link | address | addrlabel | route | rule | neigh | ntable | tunnel | tuntap | maddress | mroute | mrule | monitor | xfrm | netns | l2tp | tcp_metrics }

       OPTIONS := { -V[ersion] | -h[uman-readable] | -s[tatistics] | -r[esolve] | -f[amily] { inet | inet6 | ipx | dnet | link } | -o[neline] | -n[etns] name | -a[ll] | -c[olor] }

OPTIONS
       -V, -Version
              Print the version of the ip utility and exit.

       -h, -human, -human-readable
              output statistics with human readable values followed by suffix.

       -b, -batch <FILENAME>
              Read commands from provided file or standard input and invoke them.  First failure will cause termination of ip.

       -force Don't terminate ip on errors in batch mode.  If there were any errors during execution of the commands, the application return code will be non zero.

       -s, -stats, -statistics
              Output more information. If the option appears twice or more, the amount of information increases.  As a rule, the information is statistics or some time values.

       -d, -details
              Output more detailed information.

       -l, -loops <COUNT>
              Specify maximum number of loops the 'ip address flush' logic will attempt before giving up. The default is 10.  Zero (0) means loop until all addresses are removed.

       -f, -family <FAMILY>
              Specifies the protocol family to use. The protocol family identifier can be one of inet, inet6, bridge, ipx, dnet, mpls or link.  If this option is not present, the protocol family is guessed from other arguments. If the rest of the command line does not give enough information to guess the family, ip falls back to the default one, usually inet or any.  link is a special family identifier meaning that no networking protocol is involved.

       -4     shortcut for -family inet.

       -6     shortcut for -family inet6.

       -B     shortcut for -family bridge.

       -D     shortcut for -family decnet.

       -I     shortcut for -family ipx.

       -M     shortcut for -family mpls.

       -0     shortcut for -family link.

       -o, -oneline
              output each record on a single line, replacing line feeds with the '\' character. This is convenient when you want to count records with wc(1) or to grep(1) the output.

       -r, -resolve
              use the system's name resolver to print DNS names instead of host addresses.

       -n, -netns <NETNS>
              switches ip to the specified network namespace NETNS.  Actually it just simplifies executing of:

              ip netns exec NETNS ip [ OPTIONS ] OBJECT { COMMAND | help }

              to

              ip -n[etns] NETNS [ OPTIONS ] OBJECT { COMMAND | help }

       -a, -all
              executes specified command over all objects, it depends if command supports this option.

       -c, -color
              Use color output.

       -t, -timestamp
              display current time when using monitor option.

   OBJECT
       address
              - protocol (IP or IPv6) address on a device.

       addrlabel
              - label configuration for protocol address selection.

       l2tp   - tunnel ethernet over IP (L2TPv3).

       link   - network device.

       maddress
              - multicast address.

       monitor
              - watch for netlink messages.

       mroute - multicast routing cache entry.

       mrule  - rule in multicast routing policy database.

       neighbour
              - manage ARP or NDISC cache entries.

       netns  - manage network namespaces.

       ntable - manage the neighbor cache's operation.

       route  - routing table entry.

       rule   - rule in routing policy database.

       tcp_metrics/tcpmetrics
              - manage TCP Metrics

       tunnel - tunnel over IP.

       tuntap - manage TUN/TAP devices.

       xfrm   - manage IPSec policies.

       The names of all objects may be written in full or abbreviated form, for example address can be abbreviated as addr or just a.

   COMMAND
       As a rule, it is possible to add, delete and show (or list ) objects, but some objects do not allow all of these operations or have some additional commands.

## ss

       -n, --numeric
              Do not try to resolve service names.

       -r, --resolve
              Try to resolve numeric address/ports.

       -a, --all
              Display both listening and non-listening (for TCP this means established connections) sockets.

       -l, --listening
              Display only listening sockets (these are omitted by default).

       -o, --options
              Show timer information.

       -e, --extended
              Show detailed socket information

       -m, --memory
              Show socket memory usage.

       -p, --processes
              Show process using socket.

       -i, --info
              Show internal TCP information.

       -s, --summary
              Print  summary statistics. This option does not parse socket lists obtaining summary from various sources. It is useful when amount of sockets is so huge that parsing /proc/net/tcp is painful.

       -Z, --context
              As the -p option but also shows process security context.

              For netlink(7) sockets the initiating process context is displayed as follows:

                     1.  If valid pid show the process context.

                     2.  If destination is kernel (pid = 0) show kernel initial context.

                     3.  If a unique identifier has been allocated by the kernel or netlink user, show context as "unavailable". This will generally indicate that a process has more than one netlink socket active.

       -z, --contexts
              As  the -Z option but also shows the socket context. The socket context is taken from the associated inode and is not the actual socket context held by the kernel. Sockets are typically labeled with the context of the creating process, however the context shown will reflect any policy role,  type and/or range transition rules applied, and is therefore a useful reference.

       -N NSNAME, --net=NSNAME
              Switch to the specified network namespace name.

       -b, --bpf
              Show socket BPF filters (only administrators are allowed to get these information).

       -4, --ipv4
              Display only IP version 4 sockets (alias for -f inet).

       -6, --ipv6
              Display only IP version 6 sockets (alias for -f inet6).

       -0, --packet
              Display PACKET sockets (alias for -f link).

       -t, --tcp
              Display TCP sockets.

       -u, --udp
              Display UDP sockets.

       -d, --dccp
              Display DCCP sockets.

       -w, --raw
              Display RAW sockets.

       -x, --unix
              Display Unix domain sockets (alias for -f unix).

       -f FAMILY, --family=FAMILY
              Display sockets of type FAMILY.  Currently the following families are supported: unix, inet, inet6, link, netlink.

       -A QUERY, --query=QUERY, --socket=QUERY
              List  of  socket  tables  to  dump,  separated by commas. The following identifiers are understood: all, inet, tcp, udp, raw, unix, packet, netlink, unix_dgram, unix_stream, unix_seqpacket, packet_raw, packet_dgram.

       -D FILE, --diag=FILE
              Do not display anything, just dump raw information about TCP sockets to FILE after applying filters. If FILE is - stdout is used.

       -F FILE, --filter=FILE
              Read filter information from FILE.  Each line of FILE is interpreted like single command line option. If FILE is - stdin is used.

       FILTER := [ state STATE-FILTER ] [ EXPRESSION ]
              Please take a look at the official documentation (Debian package iproute-doc) for details regarding filters.

STATE-FILTER
       STATE-FILTER allows to construct arbitrary set of states to match. Its syntax is sequence of keywords state and exclude followed by identifier of state.

       Available identifiers are:

              All standard TCP states: established, syn-sent, syn-recv, fin-wait-1, fin-wait-2, time-wait, closed, close-wait, last-ack, listen and closing.

              all - for all the states

              connected - all the states except for listen and closed

              synchronized - all the connected states except for syn-sent

              bucket - states, which are maintained as minisockets, i.e.  time-wait and syn-recv

              big - opposite to bucket

USAGE EXAMPLES
       ss -t -a
              Display all TCP sockets.

       ss -t -a -Z
              Display all TCP sockets with process SELinux security contexts.

       ss -u -a
              Display all UDP sockets.

       ss -o state established '( dport = :ssh or sport = :ssh )'
              Display all established ssh connections.

       ss -x src /tmp/.X11-unix/*
              Find all local processes connected to X server.

       ss -o state fin-wait-1 '( sport = :http or sport = :https )' dst 193.233.7/24
              List all the tcp sockets in state FIN-WAIT-1 for our apache to network 193.233.7/24 and look at their timers.

## nmap

网络探测工具，安全/端口扫描

nmap [Scan Type...] [Options] {target specification}

可扫描网络上哪些主机是可见的，主机提供哪些服务(程序名称和版本)，操作系统名称和版本，使用什么防火墙

端口相关：端口号，协议，服务名称，状态，其中状态为： open, filtered, closed, or unfiltered.

* Filtered, 存在防火墙，filter, or other network obstacle 阻止端口通信，无法探测。

```bash
nmap -A -T4 scanme.nmap.org
```

* 192.168.10.0/24 扫描 192.168.10.0 到 192.168.10.255, 共 256 个主机
* 192.168.10.40/24 扫描 192.168.10.0 到 192.168.10.255, 共 256 个主机
* scanme.nmap.org( 64.13.134.52 ) scanme.nmap.org/16 扫描 65,536 个 IP ( 64.13.0.0 到 64.13.255.255 )
* /0 扫描整个 Internet
* /32 只扫描一个主机

* 192.168.0-255.1-254 跳过最后为 .0 或 .255 的地址
* 192.168.3-5,7.1 扫描 192.168.3.1, 192.168.4.1, 192.168.5.1, 192.168.7.1
* - 是 0-255 的简写, 最开始使用 0- 以避免和linux命令冲突
* 0-255.0-255.13.37 扫描所有结尾为 13.37 的地址

       -iL inputfilename (Input from list) .
           从文件读入 target specifications

       --exclude host1[,host2[,...]] (Exclude hosts/networks) .
           不包含主机列表

       --excludefile exclude_file (Exclude list from file) .
           不包含主机列表，从文件读入

       The -P* options (which select ping types) can be combined. Also note that ARP/Neighbor Discovery (-PR) is done by default 

host discovery

       -sL (List Scan) .
           只列出每个主机, 列出主机名， 确定目标有多少 ip 地址

       -sn (No port scan) .
           在发现主机后不进行端口扫描, 也不列出主机名。

       -Pn (No ping) .
           跳过 discovery 阶段

       -PS port list (TCP SYN Ping) .
           发送 empty TCP packet with the SYN flag set. 

       -PA port list (TCP ACK Ping) .
           与 -PS 相似，不同是 TCP ACK flag is set

           防火墙可能会阻止 syn 请求，此时可使用 ack。防火墙也可能会丢弃不期望的包(stateful rules), 此时使用 syn

           可同时使用 -PS -PA

       -PU port list (UDP Ping) .
           发关系 UDP packet 

           可以穿过只设置了 tcp 的防火墙

       -PY port list (SCTP INIT Ping) .
           发送 SCTP packet containing a minimal INIT chunk

       -PE; -PP; -PM (ICMP Ping Types) .
           与 ping 相同

       -PO protocol list (IP Protocol Ping) .
           IP protocol ping, which sends IP packets with the specified protocol number set in their IP header. The protocol list takes the same format as do port lists in the previously discussed TCP, UDP and SCTP host discovery options. If no protocols are specified, the default is to send multiple IP packets for ICMP (protocol 1), IGMP (protocol 2), and IP-in-IP (protocol 4). Note that for the ICMP, IGMP, TCP (protocol 6), UDP (protocol 17) and SCTP (protocol 132), the packets are sent with the proper protocol headers.  while other protocols are sent with no additional data beyond the IP header (unless any of --data, --data-string, or --data-length options are specified).

       -PR (ARP Ping) .
           扫描 ethernet LAN

       --traceroute (Trace path to host) .
           Traceroutes. It works with all scan types except connect scans (-sT) and idle scans (-sI)

       -n (No DNS resolution) .
           never do reverse DNS resolution on the active IP addresses it finds

       -R (DNS resolution for all targets) .
           Tells Nmap to always do reverse DNS resolution on the target IP addresses.

PORT SCANNING BASICS

           The unfiltered state means that a port is accessible, but Nmap is unable to determine whether it is open or closed. Only the ACK scan, which is used to map firewall rulesets, classifies ports into this state. Scanning unfiltered ports with other scan types such as Window scan, SYN scan, or FIN scan, may help resolve whether the port is open.

           Nmap places ports in this state when it is unable to determine whether a port is open or filtered. This occurs for scan types in which open ports give no response. The lack of response could also mean that a packet filter dropped the probe or any response it elicited. So Nmap does not know for sure whether the port is open or being filtered. The UDP, IP protocol, FIN, NULL, and Xmas scans classify ports this way.

PORT SCANNING TECHNIQUES

       Most of the scan types are only available to privileged users

       -sS (TCP SYN scan) .
           SYN scan is the default and most popular scan option for good reasons. 

           This technique is often referred to as half-open scanning, because you don't open a full TCP connection. You send a SYN packet, as if you are going to open a real connection and then wait for a response. A SYN/ACK indicates the port is listening (open), while a RST (reset) is indicative of a non-listener. If no response is received after several retransmissions, the port is marked as filtered. The port is also marked filtered if an ICMP unreachable error (type 3, code 0, 1, 2, 3, 9, 10, or 13) is received. The port is also considered open if a SYN packet (without the ACK flag) is received in response. This can be due to an extremely rare TCP feature known as a simultaneous open or split handshake connection (see https://nmap.org/misc/split-handshake.pdf).

       -sT (TCP connect scan) .
           TCP connect scan is the default TCP scan type when SYN scan is not an option. This is the case when a user does not have raw packet privileges. 

       -sU (UDP scans) .

       -sY (SCTP INIT scan) .

       -sN; -sF; -sX (TCP NULL, FIN, and Xmas scans) .

           Null scan (-sN)
               Does not set any bits (TCP flag header is 0)

           FIN scan (-sF)
               Sets just the TCP FIN bit.

           Xmas scan (-sX)
               Sets the FIN, PSH, and URG flags, lighting the packet up like a Christmas tree.

           有可能穿过防火墙

       -sA (TCP ACK scan) .
           It is used to map out firewall rulesets, determining whether they are stateful or not and which ports are filtered.

           When scanning unfiltered systems, open and closed ports will both return a RST packet. Nmap then labels them as unfiltered, meaning that they are reachable by the ACK packet, but whether they are open or closed is undetermined. Ports that don't respond, or send certain ICMP error messages back (type 3, code 0, 1, 2, 3, 9, 10, or 13), are labeled filtered.

       -sW (TCP Window scan) .
           It does this by examining the TCP Window field of the RST packets returned. On some systems, open ports use a positive window size (even for RST packets) while closed ones have a zero window. So instead of always listing a port as unfiltered when it receives a RST back, Window scan lists the port as open or closed if the TCP Window value in that reset is positive or zero, respectively.

           you can't always trust it. 

       -sM (TCP Maimon scan) .
           This technique is exactly the same as NULL, FIN, and Xmas scans, except that the probe is FIN/ACK. According to RFC 793[8] (TCP), a RST packet should be generated in response to such a probe whether the port is open or closed. However, Uriel noticed that many BSD-derived systems simply drop the packet if the port is open.

       --scanflags (Custom TCP scan) .

       -sZ (SCTP COOKIE ECHO scan) .

       -sI zombie host[:probeport] (idle scan) .

       -sO (IP protocol scan) .

       -b FTP relay host (FTP bounce scan) .

PORT SPECIFICATION AND SCAN ORDER

       -p port ranges (Only scan specified ports) .
           1-1023 

           T: for TCP, U: for UDP, S: for SCTP, or P: for IP
           
           -p U:53,111,137,T:21-25,80,139,8080 

       --exclude-ports port ranges (Exclude the specified ports from scanning) .

       -F (Fast (limited port) scan) .
           Normally Nmap scans the most common 1,000 ports for each scanned protocol. With -F, this is reduced to 100.

       -r (Don't randomize ports) .
           specify -r for sequential (sorted from lowest to highest) port scanning instead.

       --port-ratio ratio<decimal number between 0 and 1>
           Scans all ports in nmap-services file with a ratio greater than the one given.  ratio must be between 0.0 and 1.1.

       --top-ports n
           Scans the n highest-ratio ports found in nmap-services file after excluding all ports specified by --exclude-ports.  n must be 1 or greater.


EXAMPLES

       nmap -v scanme.nmap.org.

       This option scans all reserved TCP ports on the machine scanme.nmap.org . The -v option enables verbose mode.

       sudo nmap -sS -O scanme.nmap.org/24.

       SYN scan against each machine that is up out of the 256 IPs on the class C sized network where Scanme resides. It also tries to determine what operating system is running on each host that is up and running.

       nmap -sV -p 22,53,110,143,4564 198.116.0-255.1-127.

       host enumeration and a TCP scan at the first half of each of the 255 possible eight-bit subnets in the 198.116 class B address space. This tests whether the systems run SSH, DNS, POP3, or IMAP on their standard ports, or anything on port 4564. For any of these ports found open, version detection is used to determine what application is running.

       nmap -v -iR 100000 -Pn -p 80.

       Asks Nmap to choose 100,000 hosts at random and scan them for web servers (port 80). Host enumeration is disabled with -Pn since first sending a couple probes to determine whether a host is up is wasteful when you are only probing one port on each target host anyway.

       nmap -Pn -p80 -oX logs/pb-port80scan.xml -oG logs/pb-port80scan.gnmap 216.163.128.20/20.

       This scans 4096 IPs for any web servers (without pinging them) and saves the output in grepable and XML formats.


## nc

SYNOPSIS
     nc [-46bCDdhklnrStUuvZz] [-I length] [-i interval] [-O length] [-P proxy_username] [-p source_port] [-q seconds] [-s source] [-T toskeyword] [-V rtable]
        [-w timeout] [-X proxy_protocol] [-x proxy_address[:port]] [destination] [port]

DESCRIPTION
     The nc (or netcat) utility is used for just about anything under the sun involving TCP, UDP, or UNIX-domain sockets.  It can open TCP connections, send UDP packets, listen on arbitrary TCP and UDP ports, do port scanning, and deal with both IPv4 and IPv6.

     Common uses include:

           ·   simple TCP proxies
           ·   shell-script based HTTP clients and servers
           ·   network daemon testing
           ·   a SOCKS or HTTP ProxyCommand for ssh(1)
           ·   and much, much more

     The options are as follows:

     -4      Forces nc to use IPv4 addresses only.

     -6      Forces nc to use IPv6 addresses only.

     -b      Allow broadcast.

     -C      Send CRLF as line-ending.

     -D      Enable debugging on the socket.

     -d      Do not attempt to read from stdin.

     -h      Prints out nc help.

     -I length
             Specifies the size of the TCP receive buffer.

     -i interval
             Specifies a delay time interval between lines of text sent and received.  Also causes a delay time between connections to multiple ports.

     -k      Forces nc to stay listening for another connection after its current connection is completed.  It is an error to use this option without the -l option.

     -l      Used to specify that nc should listen for an incoming connection rather than initiate a connection to a remote host.  It is an error to use this option in conjunction with the -p, -s, or -z options.  Additionally, any timeouts specified with the -w option are ignored.

     -n      Do not do any DNS or service lookups on any specified addresses, hostnames or ports.

     -O length
             Specifies the size of the TCP send buffer.

     -P proxy_username
             Specifies a username to present to a proxy server that requires authentication.  If no username is specified then authentication will not be attempted.  Proxy authentication is only supported for HTTP CONNECT proxies at present.

     -p source_port
             Specifies the source port nc should use, subject to privilege restrictions and availability.

     -q seconds
             after EOF on stdin, wait the specified number of seconds and then quit. If seconds is negative, wait forever.

     -r      Specifies that source and/or destination ports should be chosen randomly instead of sequentially within a range or in the order that the system assigns them.

     -S      Enables the RFC 2385 TCP MD5 signature option.

     -s source
             Specifies the IP of the interface which is used to send the packets.  For UNIX-domain datagram sockets, specifies the local temporary socket file to create and use so that datagrams can be received.  It is an error to use this option in conjunction with the -l option.

     -T toskeyword
             Change IPv4 TOS value.  toskeyword may be one of critical, inetcontrol, lowcost, lowdelay, netcontrol, throughput, reliability, or one of the Diff‐ Serv Code Points: ef, af11 ... af43, cs0 ... cs7; or a number in either hex or decimal.

     -t      Causes nc to send RFC 854 DON'T and WON'T responses to RFC 854 DO and WILL requests.  This makes it possible to use nc to script telnet sessions.

     -U      Specifies to use UNIX-domain sockets.

     -u      Use UDP instead of the default option of TCP.  For UNIX-domain sockets, use a datagram socket instead of a stream socket.  If a UNIX-domain socket is used, a temporary receiving socket is created in /tmp unless the -s flag is given.

     -V rtable
             Set the routing table to be used.  The default is 0.

     -v      Have nc give more verbose output.

     -w timeout
             Connections which cannot be established or are idle timeout after timeout seconds.  The -w flag has no effect on the -l option, i.e. nc will listen forever for a connection, with or without the -w flag.  The default is no timeout.

     -X proxy_protocol
             Requests that nc should use the specified protocol when talking to the proxy server.  Supported protocols are “4” (SOCKS v.4), “5” (SOCKS v.5) and “connect” (HTTPS proxy).  If the protocol is not specified, SOCKS version 5 is used.

     -x proxy_address[:port]
             Requests that nc should connect to destination using a proxy at proxy_address and port.  If port is not specified, the well-known port for the proxy protocol is used (1080 for SOCKS, 3128 for HTTPS).

     -Z      DCCP mode.

     -z      Specifies that nc should just scan for listening daemons, without sending any data to them.  It is an error to use this option in conjunction with the -l option.

     destination can be a numerical IP address or a symbolic hostname (unless the -n option is given).  In general, a destination must be specified, unless the -l option is given (in which case the local host is used).  For UNIX-domain sockets, a destination is required and is the socket path to connect to (or listen on if the -l option is given).

     port can be a single integer or a range of ports.  Ranges are in the form nn-mm.  In general, a destination port must be specified, unless the -U option is given.

CLIENT/SERVER MODEL
     It is quite simple to build a very basic client/server model using nc.  On one console, start nc listening on a specific port for a connection.  For example:

           $ nc -l 1234

     nc is now listening on port 1234 for a connection.  On a second console (or a second machine), connect to the machine and port being listened on:

           $ nc 127.0.0.1 1234

     There should now be a connection between the ports.  Anything typed at the second console will be concatenated to the first, and vice-versa.  After the connection has been set up, nc does not really care which side is being used as a ‘server’ and which side is being used as a ‘client’.  The connection may be terminated using an EOF (‘^D’).

     There is no -c or -e option in this netcat, but you still can execute a command after connection being established by redirecting file descriptors. Be cautious here because opening a port and let anyone connected execute arbitrary command on your site is DANGEROUS. If you really need to do this, here is an example:

     On ‘server’ side:

           $ rm -f /tmp/f; mkfifo /tmp/f
           $ cat /tmp/f | /bin/sh -i 2>&1 | nc -l 127.0.0.1 1234 > /tmp/f

     On ‘client’ side:

           $ nc host.example.com 1234
           $ (shell prompt from host.example.com)

     By doing this, you create a fifo at /tmp/f and make nc listen at port 1234 of address 127.0.0.1 on ‘server’ side, when a ‘client’ establishes a connection successfully to that port, /bin/sh gets executed on ‘server’ side and the shell prompt is given to ‘client’ side.

     When connection is terminated, nc quits as well. Use -k if you want it keep listening, but if the command quits this option won't restart it or keep nc running. Also don't forget to remove the file descriptor once you don't need it anymore:

           $ rm -f /tmp/f

DATA TRANSFER
     The example in the previous section can be expanded to build a basic data transfer model.  Any information input into one end of the connection will be output to the other end, and input and output can be easily captured in order to emulate file transfer.

     Start by using nc to listen on a specific port, with output captured into a file:

           $ nc -l 1234 > filename.out

     Using a second machine, connect to the listening nc process, feeding it the file which is to be transferred:

           $ nc host.example.com 1234 < filename.in

     After the file has been transferred, the connection will close automatically.

TALKING TO SERVERS
     It is sometimes useful to talk to servers “by hand” rather than through a user interface.  It can aid in troubleshooting, when it might be necessary to verify what data a server is sending in response to commands issued by the client.  For example, to retrieve the home page of a web site:

           $ printf "GET / HTTP/1.0\r\n\r\n" | nc host.example.com 80

     Note that this also displays the headers sent by the web server.  They can be filtered, using a tool such as sed(1), if necessary.

     As another example, an email may be submitted to an SMTP server using:

           $ nc [-C] localhost 25 << EOF
           HELO host.example.com
           MAIL FROM:<user@host.example.com>
           RCPT TO:<user2@host.example.com>
           DATA
           Body of email.
           .
           QUIT
           EOF

PORT SCANNING
     It may be useful to know which ports are open and running services on a target machine.  The -z flag can be used to tell nc to report open ports, rather than initiate a connection. Usually it's useful to turn on verbose output to stderr by use this option in conjunction with -v option.

     For example:

           $ nc -zv host.example.com 20-30
           Connection to host.example.com 22 port [tcp/ssh] succeeded!
           Connection to host.example.com 25 port [tcp/smtp] succeeded!

     The port range was specified to limit the search to ports 20 - 30, and is scanned by increasing order.

     You can also specify a list of ports to scan, for example:

           $ nc -zv host.example.com 80 20 22
           nc: connect to host.example.com 80 (tcp) failed: Connection refused
           nc: connect to host.example.com 20 (tcp) failed: Connection refused
           Connection to host.example.com port [tcp/ssh] succeeded!

     The ports are scanned by the order you given.

     Alternatively, it might be useful to know which server software is running, and which versions.  This information is often contained within the greeting banners.  In order to retrieve these, it is necessary to first make a connection, and then break the connection when the banner has been retrieved.  This can be accomplished by specifying a small timeout with the -w flag, or perhaps by issuing a "QUIT" command to the server:

           $ echo "QUIT" | nc host.example.com 20-30
           SSH-1.99-OpenSSH_3.6.1p2
           Protocol mismatch.
           220 host.example.com IMS SMTP Receiver Version 0.84 Ready

EXAMPLES
     Open a TCP connection to port 42 of host.example.com, using port 31337 as the source port, with a timeout of 5 seconds:

           $ nc -p 31337 -w 5 host.example.com 42

     Open a UDP connection to port 53 of host.example.com:

           $ nc -u host.example.com 53

     Open a TCP connection to port 42 of host.example.com using 10.1.2.3 as the IP for the local end of the connection:

           $ nc -s 10.1.2.3 host.example.com 42

     Create and listen on a UNIX-domain stream socket:

           $ nc -lU /var/tmp/dsocket

     Connect to port 42 of host.example.com via an HTTP proxy at 10.2.3.4, port 8080.  This example could also be used by ssh(1); see the ProxyCommand directive in ssh_config(5) for more information.

           $ nc -x10.2.3.4:8080 -Xconnect host.example.com 42

     The same example again, this time enabling proxy authentication with username “ruser” if the proxy requires it:

           $ nc -x10.2.3.4:8080 -Xconnect -Pruser host.example.com 42

CAVEATS
     UDP port scans using the -uz combination of flags will always report success irrespective of the target machine's state.  However, in conjunction with a traffic sniffer either on the target machine or an intermediary device, the -uz combination could be useful for communications diagnostics.  Note that the amount of UDP traffic generated may be limited either due to hardware resources and/or configuration settings.

## curl

```bash
# 发送 post 数据
curl localhost -XPOST -d '{json format}'

#发送 get 数据
curl localhost

# 显示头部
curl localhost -i

# 文件下载指定文件名
curl -o hello.zip ftp://speedtest.tele2.net/1MB.zip
# 文件下载同名
curl -O ftp://speedtest.tele2.net/1MB.zip
# 文件显示进度条
curl -# -o hello.zip ftp://speedtest.tele2.net/1MB.zip
# 断点续传
curl -C - -O ftp://speedtest.tele2.net/1MB.zip
# ftp用户名密码
curl -u demo:password -O ftp://test.rebex.net/readme.txt
```

## ssh

     -i identity_file
             Selects a file from which the identity (private key) for public key authentication is read.  The default is ~/.ssh/identity for protocol version 1, and ~/.ssh/id_dsa, ~/.ssh/id_ecdsa, ~/.ssh/id_ed25519 and ~/.ssh/id_rsa for protocol version 2.  Identity files may also be specified on a per-host basis in the configuration file.  It is possible to have multiple -i options (and multiple identities specified in configuration files).  If no certificates have been explicitly specified by the CertificateFile directive, ssh will also try to load certificate information from the filename obtained by appending -cert.pub to identity filenames.

AUTHENTICATION
     Host-based authentication works as follows: If the machine the user logs in from is listed in /etc/hosts.equiv or /etc/ssh/shosts.equiv on the remote machine, and the user names are the same on both sides, or if the files ~/.rhosts or ~/.shosts exist in the user's home directory on the remote machine and contain a line containing the name of the client machine and the name of the user on that machine, the user is considered for login.  Additionally, the server must be able to verify the client's host key (see the description of /etc/ssh/ssh_known_hosts and ~/.ssh/known_hosts, below) for login to be permit‐ ted.  This authentication method closes security holes due to IP spoofing, DNS spoofing, and routing spoofing.  [Note to the administrator: /etc/hosts.equiv, ~/.rhosts, and the rlogin/rsh protocol in general, are inherently insecure and should be disabled if security is desired.]

     Public key authentication works as follows: The scheme is based on public-key cryptography, using cryptosystems where encryption and decryption are done using separate keys, and it is unfeasible to derive the decryption key from the encryption key.  The idea is that each user creates a public/private key pair for authentication purposes.  The server knows the public key, and only the user knows the private key.  ssh implements public key authentication protocol automatically, using one of the DSA, ECDSA, Ed25519 or RSA algorithms.  The HISTORY section of ssl(8) (on non-OpenBSD systems, see http://www.openbsd.org/cgi-bin/man.cgi?query=ssl&sektion=8#HISTORY) contains a brief discussion of the DSA and RSA algorithms.

     The file ~/.ssh/authorized_keys lists the public keys that are permitted for logging in.  When the user logs in, the ssh program tells the server which key pair it would like to use for authentication.  The client proves that it has access to the private key and the server checks that the corresponding public key is authorized to accept the account.

     The user creates his/her key pair by running ssh-keygen(1).  This stores the private key in ~/.ssh/identity (protocol 1), ~/.ssh/id_dsa (DSA), ~/.ssh/id_ecdsa (ECDSA), ~/.ssh/id_ed25519 (Ed25519), or ~/.ssh/id_rsa (RSA) and stores the public key in ~/.ssh/identity.pub (protocol 1), ~/.ssh/id_dsa.pub (DSA), ~/.ssh/id_ecdsa.pub (ECDSA), ~/.ssh/id_ed25519.pub (Ed25519), or ~/.ssh/id_rsa.pub (RSA) in the user's home directory.  The user should then copy the public key to ~/.ssh/authorized_keys in his/her home directory on the remote machine.  The authorized_keys file corresponds to the conventional ~/.rhosts file, and has one key per line, though the lines can be very long.  After this, the user can log in without giving the password.

     A variation on public key authentication is available in the form of certificate authentication: instead of a set of public/private keys, signed certificates are used.  This has the advantage that a single trusted certification authority can be used in place of many public/private keys.  See the CERTIFICATES sec‐ tion of ssh-keygen(1) for more information.

     The most convenient way to use public key or certificate authentication may be with an authentication agent.  See ssh-agent(1) and (optionally) the AddKeysToAgent directive in ssh_config(5) for more information.

     Challenge-response authentication works as follows: The server sends an arbitrary "challenge" text, and prompts for a response.  Examples of challenge-response authentication include BSD Authentication (see login.conf(5)) and PAM (some non-OpenBSD systems).

     Finally, if other authentication methods fail, ssh prompts the user for a password.  The password is sent to the remote host for checking; however, since all communications are encrypted, the password cannot be seen by someone listening on the network.

     ssh automatically maintains and checks a database containing identification for all hosts it has ever been used with.  Host keys are stored in ~/.ssh/known_hosts in the user's home directory.  Additionally, the file /etc/ssh/ssh_known_hosts is automatically checked for known hosts.  Any new hosts are automatically added to the user's file.  If a host's identification ever changes, ssh warns about this and disables password authentication to prevent server spoofing or man-in-the-middle attacks, which could otherwise be used to circumvent the encryption.  The StrictHostKeyChecking option can be used to control logins to machines whose host key is not known or has changed.

     When the user's identity has been accepted by the server, the server either executes the given command in a non-interactive session or, if no command has been specified, logs into the machine and gives the user a normal shell as an interactive session.  All communication with the remote command or shell will be automatically encrypted.

     If an interactive session is requested ssh by default will only request a pseudo-terminal (pty) for interactive sessions when the client has one.  The flags -T and -t can be used to override this behaviour.

     If a pseudo-terminal has been allocated the user may use the escape characters noted below.

     If no pseudo-terminal has been allocated, the session is transparent and can be used to reliably transfer binary data.  On most systems, setting the escape character to “none” will also make the session transparent even if a tty is used.

     The session terminates when the command or shell on the remote machine exits and all X11 and TCP connections have been closed.

TCP FORWARDING
     Forwarding of arbitrary TCP connections over the secure channel can be specified either on the command line or in a configuration file.  One possible appli‐cation of TCP forwarding is a secure connection to a mail server; another is going through firewalls.

     In the example below, we look at encrypting communication between an IRC client and server, even though the IRC server does not directly support encrypted communications.  This works as follows: the user connects to the remote host using ssh, specifying a port to be used to forward connections to the remote server.  After that it is possible to start the service which is to be encrypted on the client machine, connecting to the same local port, and ssh will encrypt and forward the connection.

     The following example tunnels an IRC session from client machine “127.0.0.1” (localhost) to remote server “server.example.com”:

         $ ssh -f -L 1234:localhost:6667 server.example.com sleep 10
         $ irc -c '#users' -p 1234 pinky 127.0.0.1

     This tunnels a connection to IRC server “server.example.com”, joining channel “#users”, nickname “pinky”, using port 1234.  It doesn't matter which port is used, as long as it's greater than 1023 (remember, only root can open sockets on privileged ports) and doesn't conflict with any ports already in use.  The connection is forwarded to port 6667 on the remote server, since that's the standard port for IRC services.

     The -f option backgrounds ssh and the remote command “sleep 10” is specified to allow an amount of time (10 seconds, in the example) to start the service which is to be tunnelled.  If no connections are made within the time specified, ssh will exit.

VERIFYING HOST KEYS
     When connecting to a server for the first time, a fingerprint of the server's public key is presented to the user (unless the option StrictHostKeyChecking has been disabled).  Fingerprints can be determined using ssh-keygen(1):

           $ ssh-keygen -l -f /etc/ssh/ssh_host_rsa_key

     If the fingerprint is already known, it can be matched and the key can be accepted or rejected.  If only legacy (MD5) fingerprints for the server are available, the ssh-keygen(1) -E option may be used to downgrade the fingerprint algorithm to match.

     Because of the difficulty of comparing host keys just by looking at fingerprint strings, there is also support to compare host keys visually, using random art.  By setting the VisualHostKey option to “yes”, a small ASCII graphic gets displayed on every login to a server, no matter if the session itself is interactive or not.  By learning the pattern a known server produces, a user can easily find out that the host key has changed when a completely different pattern is displayed.  Because these patterns are not unambiguous however, a pattern that looks similar to the pattern remembered only gives a good probability that the host key is the same, not guaranteed proof.

     To get a listing of the fingerprints along with their random art for all known hosts, the following command line can be used:

           $ ssh-keygen -lv -f ~/.ssh/known_hosts

     If the fingerprint is unknown, an alternative method of verification is available: SSH fingerprints verified by DNS.  An additional resource record (RR), SSHFP, is added to a zonefile and the connecting client is able to match the fingerprint with that of the key presented.

     In this example, we are connecting a client to a server, “host.example.com”.  The SSHFP resource records should first be added to the zonefile for host.example.com:

           $ ssh-keygen -r host.example.com.

     The output lines will have to be added to the zonefile.  To check that the zone is answering fingerprint queries:

           $ dig -t SSHFP host.example.com

     Finally the client connects:

           $ ssh -o "VerifyHostKeyDNS ask" host.example.com
           [...]
           Matching host key fingerprint found in DNS.
           Are you sure you want to continue connecting (yes/no)?

## tcpdump

dump traffic on a network

SYNOPSIS
       tcpdump [ -AbdDefhHIJKlLnNOpqStuUvxX# ] [ -B buffer_size ]
               [ -c count ]
               [ -C file_size ] [ -G rotate_seconds ] [ -F file ]
               [ -i interface ] [ -j tstamp_type ] [ -m module ] [ -M secret ]
               [ --number ] [ -Q in|out|inout ]
               [ -r file ] [ -V file ] [ -s snaplen ] [ -T type ] [ -w file ]
               [ -W filecount ]
               [ -E spi@ipaddr algo:secret,...  ]
               [ -y datalinktype ] [ -z postrotate-command ] [ -Z user ]
               [ --time-stamp-precision=tstamp_precision ]
               [ --immediate-mode ] [ --version ]
               [ expression ]

OPTIONS
       -A     Print each packet (minus its link level header) in ASCII.  Handy for capturing web pages.

       -b     Print the AS number in BGP packets in ASDOT notation rather than ASPLAIN notation.

       -B buffer_size
       --buffer-size=buffer_size
              Set the operating system capture buffer size to buffer_size, in units of KiB (1024 bytes).

       -c count
              Exit after receiving count packets.

       -C file_size
              Before writing a raw packet to a savefile, check whether the file is currently larger than file_size and, if so, close the current savefile and open a new one.  Savefiles after the first savefile will have the name specified with the -w flag, with a number after it, starting at 1  and  continuing upward.  The units of file_size are millions of bytes (1,000,000 bytes, not 1,048,576 bytes).

       -d     Dump the compiled packet-matching code in a human readable form to standard output and stop.

       -dd    Dump packet-matching code as a C program fragment.

       -ddd   Dump packet-matching code as decimal numbers (preceded with a count).

       -D
       --list-interfaces
              Print the list of the network interfaces available on the system and on which tcpdump can capture packets.  For each network interface, a number and an interface name, possibly followed by a text description of the interface, is printed.  The interface name or the number can be supplied to the -i flag to specify an interface on which to capture.

              This can be useful on systems that don't have a command to list them (e.g., Windows systems, or UNIX systems lacking ifconfig -a); the number can be useful on Windows 2000 and later systems, where the interface name is a somewhat complex string.

       -e     Print the link-level header on each dump line.  This can be used, for example, to print MAC layer addresses for protocols such as Ethernet and  IEEE 802.11.

       -E     Use spi@ipaddr algo:secret for decrypting IPsec ESP packets that are addressed to addr and contain Security Parameter Index value spi. This combination may be repeated with comma or newline separation.

              Note that setting the secret for IPv4 ESP packets is supported at this time.

              Algorithms may be des-cbc, 3des-cbc, blowfish-cbc, rc3-cbc, cast128-cbc, or none.  The default is des-cbc.  The ability to decrypt packets  is  only present if tcpdump was compiled with cryptography enabled.

              secret is the ASCII text for ESP secret key.  If preceded by 0x, then a hex value will be read.

              The  option assumes RFC2406 ESP, not RFC1827 ESP.  The option is only for debugging purposes, and the use of this option with a true `secret' key is discouraged.  By presenting IPsec secret key onto command line you make it visible to others, via ps(1) and other occasions.

              In addition to the above syntax, the syntax file name may be used to have tcpdump read the provided file in. The file is opened upon  receiving  the first ESP packet, so any special permissions that tcpdump may have been given should already have been given up.

       -f     Print  `foreign' IPv4 addresses numerically rather than symbolically (this option is intended to get around serious brain damage in Sun's NIS server — usually it hangs forever translating non-local internet numbers).

              The test for `foreign' IPv4 addresses is done using the IPv4 address and netmask of the interface on which capture is being done.  If  that  address or  netmask  are not available, available, either because the interface on which capture is being done has no address or netmask or because the capture is being done on the Linux "any" interface, which can capture on more than one interface, this option will not work correctly.

       -F file
              Use file as input for the filter expression.  An additional expression given on the command line is ignored.

       -G rotate_seconds
              If specified, rotates the dump file specified with the -w option every rotate_seconds seconds.  Savefiles will have the name specified by  -w  which should include a time format as defined by strftime(3).  If no time format is specified, each new file will overwrite the previous.

              If used in conjunction with the -C option, filenames will take the form of `file<count>'.

       -h
       --help Print the tcpdump and libpcap version strings, print a usage message, and exit.

       --version
              Print the tcpdump and libpcap version strings and exit.

       -H     Attempt to detect 802.11s draft mesh headers.

       -i interface
       --interface=interface
              Listen  on  interface.  If unspecified, tcpdump searches the system interface list for the lowest numbered, configured up interface (excluding loopback), which may turn out to be, for example, ``eth0''.

              On Linux systems with 2.2 or later kernels, an interface argument of ``any'' can be used to capture packets from all interfaces.  Note that captures on the ``any'' device will not be done in promiscuous mode.

              If  the  -D  flag is supported, an interface number as printed by that flag can be used as the interface argument, if no interface on the system has that number as a name.

       -I
       --monitor-mode
              Put the interface in "monitor mode"; this is supported only on IEEE 802.11 Wi-Fi interfaces, and supported only on some operating systems.

              Note that in monitor mode the adapter might disassociate from the network with which it's associated, so that you will not be able to use any  wireless  networks  with that adapter.  This could prevent accessing files on a network server, or resolving host names or network addresses, if you are
              capturing in monitor mode and are not connected to another network with another adapter.

              This flag will affect the output of the -L flag.  If -I isn't specified, only those link-layer types available when not  in  monitor  mode  will  be shown; if -I is specified, only those link-layer types available when in monitor mode will be shown.

       --immediate-mode
              Capture  in  "immediate  mode".   In  this mode, packets are delivered to tcpdump as soon as they arrive, rather than being buffered for efficiency.
              This is the default when printing packets rather than saving packets to a ``savefile'' if the packets are being printed to a terminal rather than to a file or pipe.

       -j tstamp_type
       --time-stamp-type=tstamp_type
              Set  the  time  stamp type for the capture to tstamp_type.  The names to use for the time stamp types are given in pcap-tstamp(7); not all the types listed there will necessarily be valid for any given interface.

       -J
       --list-time-stamp-types
              List the supported time stamp types for the interface and exit.  If the time stamp type cannot be set for the interface, no  time  stamp  types  are listed.

       --time-stamp-precision=tstamp_precision
              When  capturing,  set  the time stamp precision for the capture to tstamp_precision.  Note that availability of high precision time stamps (nanoseconds) and their actual accuracy is platform and hardware dependent.  Also note that when writing captures made with nanosecond accuracy to  a  save‐
              file,  the  time  stamps  are  written  with nanosecond resolution, and the file is written with a different magic number, to indicate that the time stamps are in seconds and nanoseconds; not all programs that read pcap savefiles will be able to read those captures.

       When reading a savefile, convert time stamps to the precision specified by timestamp_precision, and display them with that resolution.   If  the  precision specified is less than the precision of time stamps in the file, the conversion will lose precision.

       The  supported  values for timestamp_precision are micro for microsecond resolution and nano for nanosecond resolution.  The default is microsecond resolution.

       -K
       --dont-verify-checksums
              Don't attempt to verify IP, TCP, or UDP checksums.  This is useful for interfaces that perform some or all of those checksum  calculation  in  hardware; otherwise, all outgoing TCP checksums will be flagged as bad.

       -L
       --list-data-link-types
              List the known data link types for the interface, in the specified mode, and exit.  The list of known data link types may be dependent on the specified  mode;  for  example,  on  some platforms, a Wi-Fi interface might support one set of data link types when not in monitor mode (for example, it might support only fake Ethernet headers, or might support 802.11 headers but not support 802.11 headers with radio information) and another set  of data link types when in monitor mode (for example, it might support 802.11 headers, or 802.11 headers with radio information, only in monitor mode).

       -m module
              Load SMI MIB module definitions from file module.  This option can be used several times to load several MIB modules into tcpdump.

       -M secret
              Use secret as a shared secret for validating the digests found in TCP segments with the TCP-MD5 option (RFC 2385), if present.

       -n     Don't convert addresses (i.e., host addresses, port numbers, etc.) to names.

       -N     Don't print domain name qualification of host names.  E.g., if you give this flag then tcpdump will print ``nic'' instead of ``nic.ddn.mil''.

       -#
       --number
              Print an optional packet number at the beginning of the line.

       -O
       --no-optimize
              Do not run the packet-matching code optimizer.  This is useful only if you suspect a bug in the optimizer.

       -p
       --no-promiscuous-mode
              Don't  put  the  interface into promiscuous mode.  Note that the interface might be in promiscuous mode for some other reason; hence, `-p' cannot be used as an abbreviation for `ether host {local-hw-addr} or ether broadcast'.

       -Q direction
       --direction=direction
              Choose send/receive direction direction for which packets should be captured. Possible values are `in', `out' and  `inout'.  Not  available  on  all platforms.

       -q     Quick (quiet?) output.  Print less protocol information so output lines are shorter.

       -r file
              Read packets from file (which was created with the -w option or by other tools that write pcap or pcap-ng files).  Standard input is used if file is
              ``-''.

       -S
       --absolute-tcp-sequence-numbers
              Print absolute, rather than relative, TCP sequence numbers.

       -s snaplen
       --snapshot-length=snaplen
              Snarf snaplen bytes of data from each packet rather than the default of 262144 bytes.  Packets truncated because of a limited snapshot are indicated in  the output with ``[|proto]'', where proto is the name of the protocol level at which the truncation has occurred.  Note that taking larger snapshots both increases the amount of time it takes to process packets and, effectively, decreases the amount of  packet  buffering.   This  may  cause packets  to  be  lost.   You  should  limit snaplen to the smallest number that will capture the protocol information you're interested in.  Setting snaplen to 0 sets it to the default of 262144, for backwards compatibility with recent older versions of tcpdump.

       -T type
              Force packets selected by "expression" to be interpreted the specified type.  Currently known types are aodv (Ad-hoc On-demand Distance Vector  protocol), carp (Common Address Redundancy Protocol), cnfp (Cisco NetFlow protocol), lmp (Link Management Protocol), pgm (Pragmatic General Multicast), pgm_zmtp1 (ZMTP/1.0 inside PGM/EPGM), resp (REdis Serialization Protocol), radius (RADIUS), rpc (Remote Procedure Call), rtp (Real-Time Applications protocol),  rtcp  (Real-Time  Applications  control protocol), snmp (Simple Network Management Protocol), tftp (Trivial File Transfer Protocol), vat (Visual Audio Tool), wb (distributed White Board), zmtp1 (ZeroMQ Message Transport Protocol 1.0) and vxlan (Virtual eXtensible Local Area Network).

              Note that the pgm type above affects UDP interpretation only, the native PGM is always recognised as IP protocol  113  regardless.  UDP-encapsulated PGM is often called "EPGM" or "PGM/UDP".

              Note that the pgm_zmtp1 type above affects interpretation of both native PGM and UDP at once. During the native PGM decoding the application data of an ODATA/RDATA packet would be decoded as a ZeroMQ datagram with ZMTP/1.0 frames.  During the UDP decoding in addition to that any UDP packet  would be treated as an encapsulated PGM packet.

       -t     Don't print a timestamp on each dump line.

       -tt    Print the timestamp, as seconds since January 1, 1970, 00:00:00, UTC, and fractions of a second since that time, on each dump line.

       -ttt   Print a delta (micro-second resolution) between current and previous line on each dump line.

       -tttt  Print a timestamp, as hours, minutes, seconds, and fractions of a second since midnight, preceded by the date, on each dump line.

       -ttttt Print a delta (micro-second resolution) between current and first line on each dump line.

       -u     Print undecoded NFS handles.

       -U
       --packet-buffered
              If  the  -w  option is not specified, make the printed packet output ``packet-buffered''; i.e., as the description of the contents of each packet is printed, it will be written to the standard output, rather than, when not writing to a terminal, being written only when the output buffer fills.

              If the -w option is specified, make the saved raw packet output ``packet-buffered''; i.e., as each packet is saved, it will be written to the output file, rather than being written only when the output buffer fills.

              The -U flag will not be supported if tcpdump was built with an older version of libpcap that lacks the pcap_dump_flush() function.

       -v     When parsing and printing, produce (slightly more) verbose output.  For example, the time to live, identification, total length and options in an IP packet are printed.  Also enables additional packet integrity checks such as verifying the IP and ICMP header checksum.

              When writing to a file with the -w option, report, every 10 seconds, the number of packets captured.

       -vv    Even more verbose output.  For example, additional fields are printed from NFS reply packets, and SMB packets are fully decoded.

       -vvv   Even more verbose output.  For example, telnet SB ... SE options are printed in full.  With -X Telnet options are printed in hex as well.

       -V file
              Read a list of filenames from file. Standard input is used if file is ``-''.

       -w file
              Write the raw packets to file rather than parsing and printing them out.  They can later be printed with the -r option.  Standard output is used  if file is ``-''.

              This output will be buffered if written to a file or pipe, so a program reading from the file or pipe may not see packets for an arbitrary amount of time after they are received.  Use the -U flag to cause packets to be written as soon as they are received.

              The MIME type application/vnd.tcpdump.pcap has been registered with IANA for pcap files. The filename extension .pcap appears to be  the  most  commonly used along with .cap and .dmp. Tcpdump itself doesn't check the extension when reading capture files and doesn't add an extension when writing them (it uses magic numbers in the file header instead). However, many operating systems and applications will use the extension if  it  is  present and adding one (e.g. .pcap) is recommended.

              See pcap-savefile(5) for a description of the file format.

       -W     Used  in  conjunction  with the -C option, this will limit the number of files created to the specified number, and begin overwriting files from the beginning, thus creating a 'rotating' buffer.  In addition, it will name the files with enough leading 0s to support the maximum  number  of  files, allowing them to sort correctly.

              Used  in  conjunction with the -G option, this will limit the number of rotated dump files that get created, exiting with status 0 when reaching the limit. If used with -C as well, the behavior will result in cyclical files per timeslice.

       -x     When parsing and printing, in addition to printing the headers of each packet, print the data of each packet (minus its link level header)  in  hex.
              The  smaller  of  the  entire  packet or snaplen bytes will be printed.  Note that this is the entire link-layer packet, so for link layers that pad (e.g. Ethernet), the padding bytes will also be printed when the higher layer packet is shorter than the required padding.

       -xx    When parsing and printing, in addition to printing the headers of each packet, print the data of each packet, including its link  level  header,  in hex.

       -X     When  parsing  and  printing, in addition to printing the headers of each packet, print the data of each packet (minus its link level header) in hex and ASCII.  This is very handy for analysing new protocols.

       -XX    When parsing and printing, in addition to printing the headers of each packet, print the data of each packet, including its link  level  header,  in hex and ASCII.

       -y datalinktype
       --linktype=datalinktype
              Set the data link type to use while capturing packets to datalinktype.

       -z postrotate-command
              Used  in conjunction with the -C or -G options, this will make tcpdump run " postrotate-command file " where file is the savefile being closed after each rotation. For example, specifying -z gzip or -z bzip2 will compress each savefile using gzip or bzip2.

              Note that tcpdump will run the command in parallel to the capture, using the lowest priority so that this doesn't disturb the capture process.

              And in case you would like to use a command that itself takes flags or different arguments, you can always write a shell script that will  take  the savefile name as the only argument, make the flags & arguments arrangements and execute the command that you want.

       -Z user
       --relinquish-privileges=user
              If  tcpdump  is running as root, after opening the capture device or input savefile, but before opening any savefiles for output, change the user ID to user and the group ID to the primary group of user.

              This behavior can also be enabled by default at compile time.

        expression
              selects which packets will be dumped.  If no expression is given, all packets on the net will be dumped.  Otherwise, only packets for which  expression is `true' will be dumped.

              For the expression syntax, see pcap-filter(7).

              The  expression  argument  can be passed to tcpdump as either a single Shell argument, or as multiple Shell arguments, whichever is more convenient.
              Generally, if the expression contains Shell metacharacters, such as backslashes used to escape protocol names, it is easier to pass it as a  single, quoted argument rather than to escape the Shell metacharacters.  Multiple arguments are concatenated with spaces before being parsed.

EXAMPLES
       To print all packets arriving at or departing from sundown:
              tcpdump host sundown

       To print traffic between helios and either hot or ace:
              tcpdump host helios and \( hot or ace \)

       To print all IP packets between ace and any host except helios:
              tcpdump ip host ace and not helios

       To print all traffic between local hosts and hosts at Berkeley:
              tcpdump net ucb-ether

       To print all ftp traffic through internet gateway snup: (note that the expression is quoted to prevent the shell from (mis-)interpreting the parentheses):
              tcpdump 'gateway snup and (port ftp or ftp-data)'

       To print traffic neither sourced from nor destined for local hosts (if you gateway to one other net, this stuff should never make it onto your local net).
              tcpdump ip and not net localnet

       To print the start and end packets (the SYN and FIN packets) of each TCP conversation that involves a non-local host.
              tcpdump 'tcp[tcpflags] & (tcp-syn|tcp-fin) != 0 and not src and dst net localnet'

       To  print all IPv4 HTTP packets to and from port 80, i.e. print only packets that contain data, not, for example, SYN and FIN packets and ACK-only packets.
       (IPv6 is left as an exercise for the reader.)
              tcpdump 'tcp port 80 and (((ip[2:2] - ((ip[0]&0xf)<<2)) - ((tcp[12]&0xf0)>>2)) != 0)'

       To print IP packets longer than 576 bytes sent through gateway snup:
              tcpdump 'gateway snup and ip[2:2] > 576'

       To print IP broadcast or multicast packets that were not sent via Ethernet broadcast or multicast:
              tcpdump 'ether[0] & 1 = 0 and ip[16] >= 224'

       To print all ICMP packets that are not echo requests/replies (i.e., not ping packets):
              tcpdump 'icmp[icmptype] != icmp-echo and icmp[icmptype] != icmp-echoreply'


## hostname

       hostname - show or set the system's host name
       domainname - show or set the system's NIS/YP domain name
       ypdomainname - show or set the system's NIS/YP domain name
       nisdomainname - show or set the system's NIS/YP domain name
       dnsdomainname - show the system's DNS domain name

SYNOPSIS
       hostname [-a|--alias] [-d|--domain] [-f|--fqdn|--long] [-A|--all-fqdns] [-i|--ip-address] [-I|--all-ip-addresses] [-s|--short] [-y|--yp|--nis]
       hostname [-b|--boot] [-F|--file filename] [hostname]
       hostname [-h|--help] [-V|--version]

       domainname [nisdomain] [-F file]
       ypdomainname [nisdomain] [-F file]
       nisdomainname [nisdomain] [-F file]

       dnsdomainname

DESCRIPTION
       Hostname is used to display the system's DNS name, and to display or set its hostname or NIS domain name.

   GET NAME
       When called without any arguments, the program displays the current names:

       hostname will print the name of the system as returned by the gethostname(2) function.

       domainname  will  print  the  NIS  domainname  of  the  system.   domainname uses the gethostname(2) function, while ypdomainname and nisdomainname use the
       yp_get_default_domain(3).

       dnsdomainname will print the domain part of the FQDN (Fully Qualified Domain Name). The complete FQDN of the system is returned with hostname  --fqdn  (but
       see the warnings in section THE FQDN below).

   SET NAME
       When  called with one argument or with the --file option, the commands set the host name or the NIS/YP domain name.  hostname uses the sethostname(2) function, while all of the three domainname, ypdomainname and nisdomainname use setdomainname(2).  Note, that this is effective only  until  the  next  reboot.
       Edit /etc/hostname for permanent change.

       Note, that only the super-user can change the names.

       It is not possible to set the FQDN or the DNS domain name with the dnsdomainname command (see THE FQDN below).

       The  host  name  is usually set once at system startup in /etc/init.d/hostname.sh (normally by reading the contents of a file which contains the host name, e.g.  /etc/hostname).

   THE FQDN
       The FQDN (Fully Qualified Domain Name) of the system is the name that the resolver(3) returns for the host name, such as, ursula.example.com.  It  is  usually  the  hostname  followed by the DNS domain name (the part after the first dot).  You can check the FQDN using hostname --fqdn or the domain name using dnsdomainname.

       You cannot change the FQDN with hostname or dnsdomainname.

       The recommended method of setting the FQDN is to make the hostname be an alias for the fully qualified name using /etc/hosts, DNS, or NIS. For example,  if the hostname was "ursula", one might have a line in /etc/hosts which reads

              127.0.1.1    ursula.example.com ursula

       Technically:  The  FQDN  is  the name getaddrinfo(3) returns for the host name returned by gethostname(2).  The DNS domain name is the part after the first dot.

       Therefore it depends on the configuration of the resolver (usually in /etc/host.conf) how you can change it. Usually the hosts file is parsed before DNS or NIS, so it is most common to change the FQDN in /etc/hosts.

       If  a  machine has multiple network interfaces/addresses or is used in a mobile environment, then it may either have multiple FQDNs/domain names or none at all. Therefore avoid using hostname --fqdn, hostname --domain and dnsdomainname.  hostname --ip-address is subject to the same limitations so it should  be avoided as well.

OPTIONS

       -A, --all-fqdns
              Displays all FQDNs of the machine. This option enumerates all configured network addresses on all configured network interfaces, and translates them to DNS domain names. Addresses that cannot be translated (i.e. because they do not have an appropriate reverse IP entry) are skipped. Note that different  addresses  may  resolve to the same name, therefore the output may contain duplicate entries. Do not make any assumptions about the order of the output.

       -b, --boot
              Always set a hostname; this allows the file specified by -F to be non-existant or empty, in which case the default hostname localhost will  be  used if none is yet set.

       -d, --domain
              Display  the  name  of the DNS domain.  Don't use the command domainname to get the DNS domain name because it will show the NIS domain name and not the DNS domain name. Use dnsdomainname instead. See the warnings in section THE FQDN above, and avoid using this option.

       -f, --fqdn, --long
              Display the FQDN (Fully Qualified Domain Name). A FQDN consists of a short host name and the DNS domain name. Unless you are using bind or  NIS  for host  lookups  you  can  change the FQDN and the DNS domain name (which is part of the FQDN) in the /etc/hosts file. See the warnings in section THE FQDN above und use hostname --all-fqdns instead wherever possible.

       -I, --all-ip-addresses
              Display  all  network  addresses  of the host. This option enumerates all configured addresses on all network interfaces. The loopback interface and IPv6 link-local addresses are omitted.

       -s, --short
              Display the short host name. This is the host name cut at the first dot.

       -y, --yp, --nis
              Display the NIS domain name. If a parameter is given (or --file name ) then root can also set a new NIS domain.

FILES
       /etc/hostname Historically this file was supposed to only contain the hostname and not the full canonical FQDN. Nowadays most software is able to cope with a full FQDN here. This file is read at boot time by the system initialization scripts to set the hostname.

       /etc/hosts Usually, this is where one sets the domain name by aliasing the host name to the FQDN.

## iptables

iptables/ip6tables — administration tool for IPv4/IPv6 packet filtering and NAT

* iptables [-t table] {-A|-C|-D} chain rule-specification
* ip6tables [-t table] {-A|-C|-D} chain rule-specification
* iptables [-t table] -I chain [rulenum] rule-specification
* iptables [-t table] -R chain rulenum rule-specification
* iptables [-t table] -D chain rulenum
* iptables [-t table] -S [chain [rulenum]]
* iptables [-t table] {-F|-L|-Z} [chain [rulenum]] [options...]
* iptables [-t table] -N chain
* iptables [-t table] -X [chain]
* iptables [-t table] -P chain target
* iptables [-t table] -E old-chain-name new-chain-name

  * rule-specification = [matches...] [target]
  * match = -m matchname [per-match-options]
  * target = -j targetname [per-target-options]

linux 内核中管理 filter rules tables

表里的每一项定义符合条件的包进行什么操作

* ACCEPT 允许通过
* DROP 丢弃
* RETURN means stop traversing this chain and resume at the next rule in the previous (calling) chain.  If the end of a built-in chain is reached or a rule in a built-in chain with target RETURN is matched, the target  specified by the chain policy determines the fate of the packet.

TABLES

有５个独立的表

       -t, --table table
              指定此命令针对的表

              filter:
                  默认表。It contains the built-in chains INPUT (for packets destined to local sockets), FORWARD (for packets being routed through the box), and OUTPUT (for locally-generated packets).

              nat:
                  This table is consulted when a packet that creates a new connection is encountered.  It consists of three built-ins:  PREROUTING  (for  altering packets  as soon as they come in), OUTPUT (for altering locally-generated packets before routing), and POSTROUTING (for altering packets as they are about to go out).

              mangle:
                  This table is used for specialized packet alteration.  Until kernel 2.4.17 it had two built-in chains: PREROUTING (for altering incoming packets before  routing)  and OUTPUT (for altering locally-generated packets before routing).  Since kernel 2.4.18, three other built-in chains are also supported: INPUT (for packets coming into the box itself), FORWARD (for altering packets being routed through the  box),  and  POSTROUTING  (for altering packets as they are about to go out).

              raw:
                  This  table is used mainly for configuring exemptions from connection tracking in combination with the NOTRACK target.  It registers at the netfilter hooks with higher priority and is thus called before ip_conntrack, or any other IP tables.  It provides the  following  built-in  chains: PREROUTING (for packets arriving via any network interface) OUTPUT (for packets generated by local processes)

              security:
                  This table is used for Mandatory Access Control (MAC) networking rules, such as those enabled by the SECMARK and CONNSECMARK targets.  Mandatory Access Control is implemented by Linux Security Modules such as SELinux.  The security table is called after the filter table, allowing any Discretionary  Access  Control (DAC) rules in the filter table to take effect before MAC rules.  This table provides the following built-in chains: INPUT (for packets coming into the box itself), OUTPUT (for altering locally-generated packets before routing), and FORWARD (for altering  pack‐ ets being routed through the box).

       -A, --append chain rule-specification
              Append one or more rules to the end of the selected chain.  When the source and/or destination names resolve to more than one address, a  rule  will be added for each possible address combination.

       -C, --check chain rule-specification
              Check  whether  a rule matching the specification does exist in the selected chain. This command uses the same logic as -D to find a matching entry

       -D, --delete chain rule-specification
       -D, --delete chain rulenum
              Delete one or more rules from the selected chain.  There are two versions of this command: the rule can be  specified  as  a  number  in  the  chain (starting at 1 for the first rule) or a rule to match.

       -I, --insert chain [rulenum] rule-specification
              Insert one or more rules in the selected chain as the given rule number.  So, if the rule number is 1, the rule or rules are inserted at the head of the chain.  This is also the default if no rule number is specified.

       -R, --replace chain rulenum rule-specification
              Replace a rule in the selected chain.  If the source and/or destination names resolve to multiple addresses, the command will fail.  Rules are  num‐ bered starting at 1.

       -L, --list [chain]
              List  all  rules in the selected chain.  If no chain is selected, all chains are listed. Like every other iptables command, it applies to the specified table (filter is the default), so NAT rules get listed by iptables -t nat -n -L
              Please note that it is often used with the -n option, in order to avoid long reverse DNS lookups.  It is legal to specify the -Z  (zero)  option  as well,  in  which case the chain(s) will be atomically listed and zeroed.  The exact output is affected by the other arguments given. The exact rules are suppressed until you use
               iptables -L -v

       -S, --list-rules [chain]
              Print all rules in the selected chain.  If no chain is selected, all chains are printed like iptables-save. 

       -F, --flush [chain]
              Flush the selected chain (all the chains in the table if none is given).  This is equivalent to deleting all the rules one by one.

       -Z, --zero [chain [rulenum]]
              Zero  the  packet and byte counters in all chains, or only the given chain, or only the given rule in a chain. It is legal to specify the -L, --list (list) option as well, to see the counters immediately before they are cleared. (See above.)

       -N, --new-chain chain
              Create a new user-defined chain by the given name.  There must be no target of that name already.

       -X, --delete-chain [chain]
              Delete the optional user-defined chain specified.  There must be no references to the chain.  If there are, you must delete or replace the referring rules  before  the  chain  can be deleted.  The chain must be empty, i.e. not contain any rules.  If no argument is given, it will attempt to delete every non-builtin chain in the table.

       -P, --policy chain target
              Set the policy for the built-in (non-user-defined) chain to the given target.  The policy target must be either ACCEPT or DROP.

       -E, --rename-chain old-chain new-chain
              Rename the user specified chain to the user supplied name.  This is cosmetic, and has no effect on the structure of the table.

   PARAMETERS
       -4, --ipv4
              This option has no effect in iptables and iptables-restore.  

       -6, --ipv6
              If a rule using the -6 option is inserted with (and only with) iptables-restore, it will be silently ignored. 

       [!] -p, --protocol protocol
              The protocol of the rule or of the packet to check.  The specified protocol can be one of tcp, udp, udplite, icmp, icmpv6,esp, ah, sctp, mh  or  the special keyword "all", or it can be a numeric value, representing one of these protocols or a different one.  A protocol name from /etc/protocols is also allowed.  A "!" argument before the protocol inverts the test.  The number zero is equivalent to all. "all" will match with all  protocols  and is  taken  as default when this option is omitted.  Note that, in ip6tables, IPv6 extension headers except esp are not allowed.  esp and ipv6-nonext can be used with Kernel version 2.6.11 or later.  The number zero is equivalent to all, which means that you cannot test the protocol field for  the value 0 directly. To match on a HBH header, even if it were the last, you cannot use -p 0, but always need -m hbh.

       [!] -s, --source address[/mask][,...]
              Source  specification. Address can be either a network name, a hostname, a network IP address (with /mask), or a plain IP address. Hostnames will be resolved once only, before the rule is submitted to the kernel.  Please note that specifying any name to be resolved with a remote query such as DNS is  a  really bad idea.  The mask can be either an ipv4 network mask (for iptables) or a plain number, specifying the number of 1's at the left side of the network mask.  Thus, an iptables mask of 24 is equivalent to 255.255.255.0.  A "!" argument before  the  address  specification  inverts  the sense of the address. The flag --src is an alias for this option.  Multiple addresses can be specified, but this will expand to multiple rules (when adding with -A), or will cause multiple rules to be deleted (with -D).

       [!] -d, --destination address[/mask][,...]
              Destination specification.  See the description of the -s (source) flag for a detailed description of the syntax.  The flag --dst is  an  alias  for this option.

       -m, --match match
              Specifies  a  match  to use, that is, an extension module that tests for a specific property. The set of matches make up the condition under which a target is invoked. Matches are evaluated first to last as specified on the command line and work in short-circuit fashion,  i.e.  if  one  extension yields false, evaluation will stop.

       -j, --jump target
              This  specifies  the target of the rule; i.e., what to do if the packet matches it.  The target can be a user-defined chain (other than the one this rule is in), one of the special builtin targets which decide the fate of the packet immediately, or an extension (see EXTENSIONS  below).   If  this option is omitted in a rule (and -g is not used), then matching the rule will have no effect on the packet's fate, but the counters on the rule will be incremented.

       -g, --goto chain
              This specifies that the processing should continue in a user specified chain. Unlike the --jump option return will not continue processing  in  this chain but instead in the chain that called us via --jump.

       [!] -i, --in-interface name
              Name of an interface via which a packet was received (only for packets entering the INPUT, FORWARD and PREROUTING chains).  When the "!" argument is used before the interface name, the sense is inverted.  If the interface name ends in a "+", then any interface which begins  with  this  name  will match.  If this option is omitted, any interface name will match.

       [!] -o, --out-interface name
              Name  of  an interface via which a packet is going to be sent (for packets entering the FORWARD, OUTPUT and POSTROUTING chains).  When the "!" argument is used before the interface name, the sense is inverted.  If the interface name ends in a "+", then any interface which begins with this  name will match.  If this option is omitted, any interface name will match.

       [!] -f, --fragment
              This means that the rule only refers to second and further IPv4 fragments of fragmented packets.  Since there is no way to tell the source or desti‐ nation ports of such a packet (or ICMP type), such a packet will not match any rules which specify them.  When the "!" argument  precedes  the  "-f" flag, the rule will only match head fragments, or unfragmented packets. This option is IPv4 specific, it is not available in ip6tables.

       -c, --set-counters packets bytes
              This enables the administrator to initialize the packet and byte counters of a rule (during INSERT, APPEND, REPLACE operations).

   OTHER OPTIONS
       -v, --verbose
              Verbose  output.   

       -w, --wait [seconds]
              Wait for the xtables lock.

       -n, --numeric
              Numeric output. 

       -x, --exact
              Expand  numbers.  

       --line-numbers
              When listing rules, add line numbers to the beginning of each rule, corresponding to that rule's position in the chain.

       --modprobe=command
              When adding or inserting rules into a chain, use command to load any necessary modules (targets, match extensions, etc).
