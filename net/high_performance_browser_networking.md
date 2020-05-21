----Ilya Grigorik

Latency
    The time from the source sending a packet to the destination receiving it

Bandwidth
    Maximum throughput of a logical or physical communication path

In most cases, latency, not bandwidth, is the bottleneck for TCP

Propagation delay
    Amount of time required for a message to travel from the sender to receiver
    It is a function of distance over speed with which the signal propagates.

Transmission delay
    Amount of time required to push all the packet’s bits into the link
    It is a function of the packet’s length and data rate of the link.

Processing delay
    Amount of time required to process the packet header, check for bit-level errors, and determine the packet’s destination.

Queuing delay
    Amount of time the incoming packet is waiting in the queue until it can be processed.

traceroute 47.96.151.120
    identifying the routing path of the packet and the latency of each network hop in an IP network.
    it sends a sequence of packets toward the destination with an increasing “hop limit” (1, 2, 3, and so on).
    When the hop limit is reached, the intermediary returns an ICMP Time Exceeded message

TCP Fast Open

Window Scaling (RFC 1323)
    sysctl net.ipv4.tcp_window_scaling
    sysctl -w net.ipv4.tcp_window_scaling=1

the maximum amount of data in flight (not ACKed) between the client and the server is the minimum of the rwnd and cwnd variables.

slow-start restart (SSR)
    sysctl net.ipv4.tcp_slow_start_after_idle
    sysctl -w net.ipv4.tcp_slow_start_after_idle=0

    resets the congestion window of a connection after it has been idle for a defined period of time.
    it is recommended to disable SSR on the server.

the core principles and their implications remain unchanged:
    TCP three-way handshake introduces a full roundtrip of latency.
    TCP slow-start is applied to every new connection.
    TCP flow and congestion control regulate throughput of all connections.
    TCP throughput is regulated by current congestion window size.

it is good practice to ensure that your server is configured to use the following best practices:
    “Increasing TCP’s Initial Congestion Window”
    Disabling slow-start after idle will improve performance of long-lived TCP connections, which transfer data in bursts.
    Enabling window scaling increases the maximum receive window size and allows high-latency connections to achieve better throughput.
    “TCP Fast Open”
        Allows application data to be sent in the initial SYN packet in certain situations.
        TFO is a new optimization, which requires support both on client and server; 

inspect various statistics for open sockets
    ss --options -- extended --memory --processes --info

TCP connection can have an even greater impact:
    No bit is faster than one that is not sent; send fewer bits.
    We can’t make the bits travel faster, but we can move the bits closer.
    TCP connection reuse is critical to improve performance.

A short list to put on the agenda:
    Upgrade server kernel to latest version (Linux: 3.2+).
    Ensure that cwnd size is set to 10.
    Disable slow-start after idle.
    Ensure that window scaling is enabled.
    Eliminate redundant data transfers.
    Compress transferred data.
    Position servers closer to the user to reduce roundtrip times.
    Reuse established TCP connections whenever possible.

With that in mind, we can now summarize all the UDP non-services:
    No guarantee of message delivery
    No acknowledgments, retransmissions, or timeouts
    No guarantee of order of delivery
    No packet sequence numbers, no reordering, no head-of-line blocking
    No connection state tracking
    No connection establishment or teardown state machines
    No congestion control
    No built-in client or network feedback mechanisms

NAT devices
    each of which would be responsible for maintaining a table mapping of local IP and port tuples to one or more globally unique (public) IP and port tuples. The local IP address space behind the translator could then be reused among many different networks, thus solving the address depletion problem.

Reserved Private Network Ranges
    reserved three well-known ranges for private networks, most often residing behind a NAT device:

    10.0.0.0    – 10.255.255.255      16,777,216
    172.16.0.0  – 172.31.255.255      1,048,576
    192.168.0.0 – 192.168.255.255     65,536

    One or all of the preceding ranges should look familiar. Chances are, your local router has assigned your computer an IP address from one of those ranges. That’s your private IP address on the internal network, which is then translated by the NAT device when communicating with an outside network.

    To avoid routing errors and confusion, no public computer is allowed to be assigned an IP address from any of these reserved private network ranges.

Encryption
    A mechanism to obfuscate what is sent from one computer to another.

Authentication
    A mechanism to verify the validity of provided identification material.

Integrity
    A mechanism to detect message tampering and forgery.

Finally, with encryption and authentication in place, the TLS protocol also provides its own message framing mechanism and signs each message with a message authentication code (MAC). The MAC algorithm is a one-way cryptographic hash function (effectively a checksum), the keys to which are negotiated by both connection peers. Whenever a TLS record is sent, a MAC value is generated and appended for that message, and the receiver is then able to compute and verify the sent MAC value to ensure message integrity and authenticity.

To illustrate the difference, if you have OpenSSL installed on your computer, you can run the following tests:
    openssl speed rsa
    openssl speed aes

(e.g., port 80 for HTTP, port 443 for TLS)

what if the server wants to host multiple independent sites, each with its own TLS certificate, on the same IP address—how does that work? Trick question; it doesn’t.  To address the preceding problem, the Server Name Indication (SNI) extension was introduced to the TLS protocol, which allows the client to indicate the hostname the client is attempting to connect to at the start of the handshake. As a result, a web server can inspect the SNI hostname, select the appropriate certificate, and continue the handshake.

Picking the right record size for your application, if you have the ability to do so, can be an important optimization.
    Small records incur a larger overhead due to record framing, whereas large records will have to be delivered and reassembled by the TCP layer before they can be processed by the TLS layer and delivered to your application.

You should double-check and verify your configuration:
    Servers with multiple processes or workers should use a shared session cache.
    Size of the shared session cache should be tuned to your levels of traffic.
    AIn a multiserver setup, routing the same client IP, or the same TLS session ID, to the same server is one way to provide good session cache utilization.
    Where “sticky” load balancing is not an option, a shared cache should be used between different servers to provide good session cache utilization.
    Check and monitor your SSL/TLS session cache statistics for best performance.

Alternatively, if the client and server both support session tickets, then all session data will be stored on the client, and none of these steps are required—much, much simpler!

The maximum size of each record is 16 KB, and depending on the chosen cipher, each record will add anywhere from 20 to 40 bytes of overhead for the header, MAC, and optional padding. If the record then fits into a single TCP packet, then we also have to add the IP and TCP overhead: 20-byte header for IP, and 20-byte header for TCP with no options. As a result, there is potential for 60 to 100 bytes of overhead for each record. For a typical maximum transmission unit (MTU) size of 1,500 bytes on the wire, this packet structure translates to a minimum of 6% of framing overhead.

Small records incur overhead, large records incur latency, and there is no one answer for the “right” record size. However, for web applications, which are consumed by the browser, the recommended best practice is simple: each TCP packet should carry exactly one TLS record, and the TLS record should occupy the full maximum segment size (MSS) allocated by TCP. In other words, do not use TLS record sizes that span multiple TCP packets, and ship as much data as you can within each record. To determine the optimal TLS record size for your deployment:
    Allocate 20 bytes for IPv4 framing overhead and 40 bytes for IPv6.
    Allocate 20 bytes for TCP framing overhead.
    Allocate 40 bytes for TCP options overhead (timestamps, SACKs).

    Assuming a common 1,500-byte starting MTU, this leaves 1,420 bytes for a TLS record delivered over IPv4, and 1,400 bytes for IPv6. To be future-proof, use the IPv6 size: 1,400 bytes, or less if your MTU is lower.

If your servers are handling a large number of TLS connections, then minimizing memory usage per connection can be a vital optimization. By default, popular libraries such as OpenSSL will allocate up to 50 KB of memory per connection, but as with the record size, it may be worth checking the documentation or the source code for how to adjust this value. Google’s servers reduce their OpenSSL buffers down to about 5 KB.

in practice, you should disable TLS compression on your server for several reasons:
    The “CRIME” attack, published in 2012, leverages TLS compression to recover secret authentication cookies and allows the attacker to perform session hijacking.
    Transport-level TLS compression is not content aware and will end up attempting to recompress already compressed data (images, video, etc.).

the first optimization you should make is to verify that the server does not forget to include all the intermediate certificates when the handshake is performed. If you forget, but they will instead be forced to pause the verification and fetch the intermediate certificate on their own, verify it, and then continue. This will most likely require a new DNS lookup, TCP connection, and an HTTP GET request, adding hundreds of milliseconds to your handshake.

Conversely, make sure you do not include unnecessary certificates in your chain! Recall that server certificates are sent during the TLS handshake, which is likely running over a new TCP connection that is in the early stages of its slow-start algorithm. If the certificate chain exceeds TCP’s initial congestion window, then we will inadvertently add yet another roundtrip to the handshake: certificate length will overflow the congestion window and cause the server to stop and wait for a client ACK before proceeding.

In addition, you should investigate if it is possible to reduce the size of the sent certificates:
    Minimize the number of intermediate CAs. Ideally, your sent certificate chain should contain exactly two certificates: your site and the CA’s intermediary certificate; use this as a criteria in the selection of your CA. The third certificate, which is the CA root, should already be in the browser’s trusted root and hence should not be sent.
    It is not uncommon for many sites to include the root certificate of their CA in the chain, which is entirely unnecessary: if your browser does not already have the certificate in its trust store, then it won’t be trusted, and including the root certificate won’t change that.
    A carefully managed certificate chain can be as low as 2 or 3 KB in size, while providing all the necessary information to the browser to avoid unnecessary roundtrips or out-of-band requests for the certificates themselves. Optimizing your TLS handshake mitigates a critical performance bottleneck, since every new TLS connection is subject to its overhead.

the server can include (staple) the OCSP response from the CA to its certificate chain, allowing the browser to skip the online check. Moving the OCSP fetch to the server allows the server Optimizing for TLS to cache the signed OCSP response and save the extra request for many clients. However, there are also a few things to watch out for:
    OCSP responses can vary from 400 to 4,000 bytes in size. Stapling this response to your certificate chain may once again overflow your TCP congestion window—pay close attention to the total size.
    Only one OCSP response can be included, which may still mean that the browser will have to issue an OCSP request for other intermediate certificates, if it has not been cached already.

A short list to put on the agenda:
    Get best performance from TCP
    Upgrade TLS libraries to latest release, and (re)build servers against them.
    Enable and configure session caching and stateless resumption
    Monitor your session caching hit rates and adjust configuration accordingly.
    Terminate TLS sessions closer to the user to minimize roundtrip latencies.
    Configure your TLS record size to fit into a single TCP segment.
    Ensure that your certificate chain does not overflow the initial congestion window.
    Remove unnecessary certificates from your chain; minimize the depth.
    Disable TLS compression on your server.
    Configure SNI support on your server.
    Configure OCSP stapling on your server.
    Append HTTP Strict Transport Security header.

inspect the entire handshake and configuration of your server locally.
    openssl s_client -state -CAfile startssl.ca.crt -connect igvita.com:443

the performance of any wireless network is fundamentally limited by a small number of well-known parameters. Specifically, the amount of allocated bandwidth and the signal-to-noise ratio between receiver and sender. Further, all radio-powered communication is:
• Done over a shared communication medium (radio waves)
• Regulated to use specific bandwidth frequency ranges
• Regulated to use specific transmit power rates
• Subject to continuously changing background noise and interference
• Subject to technical constraints of the chosen wireless technology
• Subject to constraints of the device: form factor, power, etc.

Just a few factors that may affect the performance of your wireless network:
• distance between receiver and sender
• background noise in current location
• interference from users in the same network (intra-cell)
• interference from users in other, nearby networks (inter-cell)
• available transmit power, both at receiver and sender
• processing power and the chosen modulation scheme

network waterfall 
Navigation Timing 

• Critical resources such as CSS and JavaScript should be discoverable as early as possible in the document.
• CSS should be delivered as early as possible to unblock rendering and JavaScript execution.
• Noncritical JavaScript should be deferred to avoid blocking DOM and CSSOM construction.
• The HTML document is parsed incrementally by the parser; hence the document should be periodically flushed for best performance.

we can also embed additional hints into the document itself to tip off the browser about additional optimizations it can perform on our behalf:
<link rel="dns-prefetch" href="//hostname_to_resolve.com">
Pre-resolve specified hostname.

<link rel="subresource" href="/javascript/myapp.js">
Prefetch critical resource found later on this page.

<link rel="prefetch" href="/images/big.jpeg">
Prefetch resource for this or future navigation.

<link rel="prerender" href="//example.org/next_page.html">
Prerender specified page in anticipation of next user destination.

head-of-line blocking
    the server must wait to send the full HTML response before it can proceed with delivery of the CSS asset. 

In practice, most modern browsers, both desktop and mobile, open up to six connections per host.

• Get the most out of TCP first
• The browser will automatically open up to six connections on your behalf.
• Number, size, and response time of each resource will affect the optimal number of shards.
• Client latency and bandwidth will affect the optimal number of shards.
• Domain sharding can hurt performance due to additional DNS lookups and TCP slow-start.

Reducing the transferred header data, which is highly repetitive and uncompressed, could save entire roundtrips of network latency and significantly improve the performance of many web applications.

Reduce DNS lookups

Reuse TCP connections

Minimize number of HTTP redirects
    HTTP redirects can be extremely costly, especially when they redirect the client to a different hostname, which results in additional DNS lookup, TCP handshake latency, and so on. The optimal number of redirects is zero.

Use a Content Delivery Network (CDN)

Eliminate unnecessary resources
    No request is faster than a request not made.

• Cache-Control header can specify the cache lifetime (max-age) of the resource.
• Last-Modified and ETag headers provide validation mechanisms.

Leverage HTTP pipelining
    If your application controls both the client and the server, then pipelining can help eliminate significant amounts of network latency.

Apply domain sharding
    If your application performance is limited by the default six connections per origin limit, consider splitting resources across multiple origins.

Bundle resources to reduce HTTP requests
    Techniques such as concatenation and spriting can both help minimize the protocol overhead and deliver pipelining-like performance benefits.

Inline small resource
    Consider embedding small resources directly into the parent document to minimize the number of requests.