# netcat

test which of your ports are open or closed.

```bash
nc -z -w5 -v SERVER_IP PORT
```

* nc is the netcat command.
* -z just send a packet without payload.
* -w5 wait up to 5 seconds for a response.
* -v verbose mode.

# 网络相关命令

## tcpdump

网卡工作在 promiscuous mode 下。

```bash
ifconfig wlo1 down
iwconfig wlo1 mode managed
iwconfig wlo1 mode monitor
ifconfig wlo1 up

tcpdump -i wlo1

// 添加一个别名mon0，monitor模式
iwconfig wlo1 interface add mon0 type monitor
// 启用mon0，默认不启用
ifconfig mon0 up
```

## Wireshark

## 一般用户使用

```bash
cat /etc/group
sudo usermod -a -G wireshark username
```

## 工作流

```bash
sudo tcpdump port 443 -w output.pcap
wireshark output.pcap
```

## ip地址

### 本地ip地址

```bash
hostname -I
ip address
nmtui edit eno1
```

### 路由器ip地址

```bash
ip route | grep default
route -n
    注意 U 的含义是 route is 'up'，G 的含义是这是一个 gateway
```

## ip

显示和管理路由，设备，policy routing and tunnels

```bash
ip route
ip link
```

## netstat

输出网络连接，路由表，网卡信息，无效连接，多播成员

```bash
netstat -tulpn

# displays  a list of open sockets
netstat

# Display the kernel routing tables
netstat -r

# Display a table of all network interfaces
netstat -i

# Show numerical addresses instead of trying to determine symbolic host, port or user names
-n
```

## nmap

Network Mapper 网络探测及安全和端口扫描工具。用来快速扫描大型网络（也可用于本机）。可以探测网络上哪些主机可用，主机提供哪些服务，运行什么操作系统，何种类型的防火墙等等。

```bash
# -T4 faster execution
# -A enable OS and version detection, script scanning, and traceroute
nmap -A -T4 scanme.nmap.org

# TCP Connect() scans
nmap -sT <ipaddress>

# scan 1000 TCP ports
nmap <ipaddress>
nmap -v <ipaddress>
```

# curl

```bash
curl --header "Content-Type: application/json" --request POST --data '{"cmd":"update"}'  http://localhost:9998

curl --header "Content-Type: application/json" --request POST --data '{"cmd":"update"}'  http://localhost/cfg
```

## Network bridge

A bridge is a piece of software used to unite two or more network segments.A bridge behaves like a virtual network switch, working transparently (the other machines do not need to know or care about its existence). Any real devices (e.g. eth0) and virtual devices (e.g. tap0) can be connected to it.

```bash
ip link add name bridge_name type bridge
ip link set bridge_name up
ip link set eth0 up
ip link set eth0 master bridge_name
ip addr add dev bridge_name 192.168.66.66/24

bridge link

ip link set eth0 nomaster
ip link set eth0 down
ip link delete bridge_name type bridge
```

## iptables

```bash
sudo iptables --line -vnL

sudo iptables -A INPUT -p tcp -m tcp --dport 8088 -j ACCEPT
service iptables save
service iptables restart
```

```bash
sudo iptables -A INPUT -p tcp -m tcp --dport 22 -j ACCEPT
```

* -A INPUT add a rule to the INPUT chain, a chain is a group of rules
* -p tcp set tcp as the protocol this rule will apply to
* -m tcp use the tcp module.
* -- indicate additional options for the previously used module
* --dport 22 tcp module to only apply to port 22.
* -j ACCEPT the -j argument tells iptables what to do if a packet matches the constraints specified in the previous arguments. In this case it will ACCEPT those packets, other options are REJECT, DROP and more.

```bash
sudo iptables -L
sudo iptables -S
```

delete a specific rule choose a rule from sudo iptables -S

```bash
sudo iptables -D INPUT -p tcp -m tcp --dport 22 -j ACCEPT

sudo iptables -L INPUT --line-numbers
sudo iptables -D INPUT 2
```

Allow SSH on eth0 interface

```bash
sudo iptables -A INPUT -i eth0 -p tcp -m tcp --dport 22 -j ACCEPT
```

* -i eth0 apply rule to a specific interface, to allow from any interface remove this command.

To limit incoming packets to a specific IP (i.e. 10.0.3.1/32).

```bash
sudo iptables -A INPUT -i eth0 -s 10.0.3.1/32 -p tcp -m tcp --dport 22 -j ACCEPT
```

* -s 10.0.3.1/32 specifies an IP/subnet to allow connections from.

# ncat