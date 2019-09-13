# sed

```bash
```

# 用户

## 添加sudo用户

```bash
adduser xx
usermod -aG sudo xx
```

## ssh 登录

```bash
ls -al ~/.ssh
ssh-keygen -t rsa -b 4096 -C "mcflym@N123456"
ssh-add ~/.ssh/id_rsa
cat ~/.ssh/id_rsa.pub | ssh username@server.address.com 'cat >> ~/.ssh/authorized_keys'
```

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

## ip地址

### 本地ip地址

```bash
hostname -I
ip address
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

# 键盘映射

## 查看按键值

xev

## 进行键盘映射

xmodmap -e "keycode 128 = F3"

### 配置文件

```bash
~/.Xmodmap

keycode 128 = F3
keycode 123 = Insert
```
