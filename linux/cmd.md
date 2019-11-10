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

# echo

```bash
# 输出时不换行
echo -n "abc: "
```

# download

## 多线程下载

```bash
axel
```

# GUI 用户登录设置

```bash
cd /etc/lightdm/lightdm.conf.d
touch my.conf

[SeatDefaults]
allow-guest=false
greeter-hide-users=true
greeter-show-manual-login=true
```

# ftp

```bash
sudo apt-get install vsftpd

sudo cp /etc/vsftpd.conf /etc/vsftpd.conf.bk

########################################
# 编辑文件： /etc/vsftpd.conf
sudo vim /etc/vsftpd.conf

# 在文件中找到下面的行，并设置值与下面一样
write_enable=YES
local_umask=022
chroot_local_user=YES

# 在文件最后添加下面的行
allow_writeable_chroot=YES
pasv_min_port=40000
pasv_max_port=40100
########################################

sudo systemctl restart vsftpd

sudo useradd -m ftpuser -s /usr/sbin/nologin
sudo passwd ftpuser
echo "/usr/sbin/nologin" | sudo tee -a /etc/shells
```

ftp 会阻止 shell 不在 /etc/shells 的用户登录

# 花生壳

phddns

# server 安装 gui

sudo apt-get install --no-install-recommends ubuntu-desktop

# 制作iso systemback

```bash
sudo add-apt-repository ppa:nemh/systemback
sudo apt-get update && sudo apt-get install systemback unionfs-fuse

/usr/lib/systemback/sbsustart systemback
/usr/lib/systemback/sbsustart systemback gtk+
```

# Wireshark

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

# npm

## 命令

```bash
npm install tailwindcss --save-dev
```

## 修改成华为云镜像源

```bash
npm config set registry https://mirrors.huaweicloud.com/repository/npm/
npm config get registry
```

# centos

## 修改成阿里云镜像源

```bash
mv /etc/yum.repos.d/CentOS-Base.repo /etc/yum.repos.d/CentOS-Base.repo.backup
wget -O /etc/yum.repos.d/CentOS-Base.repo http://mirrors.aliyun.com/repo/Centos-6.repo
yum makecache
```

## install docker

```bash
sudo yum install -y yum-utils device-mapper-persistent-data lvm2
sudo yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo

sudo yum install docker-ce docker-ce-cli containerd.io
    or:
        yum list docker-ce --showduplicates | sort -r
        sudo yum install docker-ce-<VERSION_STRING> docker-ce-cli-<VERSION_STRING> containerd.io

sudo systemctl start docker
```

## ip address

```bash
nmtui edit eno1
```

# curl

```bash
curl --header "Content-Type: application/json" --request POST --data '{"cmd":"update"}'  http://localhost:9998

curl --header "Content-Type: application/json" --request POST --data '{"cmd":"update"}'  http://localhost/cfg
```
