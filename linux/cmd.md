# cmd

- [cmd](#cmd)
  - [cubic](#cubic)
  - [文件](#文件)
  - [ethtool](#ethtool)
  - [cache](#cache)
  - [tmux](#tmux)
    - [Tmux Plugin Manager](#tmux-plugin-manager)
  - [nc / ncat](#nc--ncat)
  - [磁盘监测](#磁盘监测)
  - [tcpdump](#tcpdump)
  - [Wireshark](#wireshark)
    - [充许普通用户使用](#充许普通用户使用)
    - [工作流](#工作流)
  - [ip地址](#ip地址)
    - [本地ip地址](#本地ip地址)
    - [路由器ip地址](#路由器ip地址)
  - [netstat](#netstat)
  - [nmap](#nmap)
  - [curl](#curl)
  - [Network bridge](#network-bridge)
  - [iptables](#iptables)
  - [xxd](#xxd)
  - [strace](#strace)
  - [系统调用号](#系统调用号)
  - [node.js](#nodejs)
  - [vim](#vim)
    - [flod command](#flod-command)
    - [auto pair](#auto-pair)
  - [vscode](#vscode)
    - [setting font size](#setting-font-size)
  - [添加sudo用户](#添加sudo用户)
    - [ubuntu](#ubuntu)
    - [centos](#centos)
  - [ssh 登录](#ssh-登录)
    - [登录慢](#登录慢)
  - [sshpass](#sshpass)
  - [键盘映射](#键盘映射)
    - [查看按键值](#查看按键值)
    - [进行键盘映射](#进行键盘映射)
      - [配置文件](#配置文件)
  - [download](#download)
    - [多线程下载](#多线程下载)
  - [GUI 用户登录设置](#gui-用户登录设置)
  - [ftp](#ftp)
  - [制作iso systemback](#制作iso-systemback)
  - [npm](#npm)
    - [修改成华为云镜像源](#修改成华为云镜像源)
  - [ibm mq](#ibm-mq)
  - [安装 ubuntu](#安装-ubuntu)
    - [打开终端时全屏或最大化](#打开终端时全屏或最大化)
  - [other](#other)
  - [firefox](#firefox)
  - [opcua](#opcua)

## cubic

ubuntu20.04.2.0 LTS

2G cpu
4G mem
25G hard drive space

sudo apt-add-repository ppa:cubic-wizard/release
sudo apt update
sudo apt install cubic

add-apt-repository --yes main
add-apt-repository --yes restricted
add-apt-repository --yes universe
add-apt-repository --yes multiverse

```bash
rm -f /etc/resolv.conf
ln -s ../run/resolvconf/resolv.conf resolv.conf

seed
default live
label live
    menu label ^Install Ubuntu
    kernel /casper/vmlinuz
    append  file=/cdrom/preseed/ks.seed auto=true priority=critical automatic-ubiquity keyboard-configuration/layoutcode=pl boot=casper initrd=/casper/initrd quiet splash ---

得到安装时的选项
debconf-get-selections --installer > alloptions.cfg

得到所有选项
debconf-get-selections >> alloptions.cfg

检测文件是否正确
debconf-set-selections -c preseed.cfg

第一次启动时运行
/etc/rc.local

initrd

preseed.cfg:

ubiquity ubiquity/minimal_install boolean true
d-i pkgsel/update-policy select none
d-i pkgsel/upgrade select none

d-i debian-installer/locale string zh_CN

d-i console-setup/ask_detect boolean false
d-i keyboard-configuration/layoutcode string us
d-i keyboard-configuration/xkb-keymap select us

d-i passwd/user-fullname string zs
d-i passwd/username string zs
d-i passwd/user-password password 123456789
d-i passwd/user-password-again password 123456789
d-i passwd/auto-login boolean true

d-i time/zone string Asia/Shanghai

d-i partman-auto/disk string /dev/sda

d-i finish-install/reboot_in_progress note
```

[seed](https://help.ubuntu.com/lts/installation-guide/s390x/apbs05.html)


## 文件

```bash
# log 文件
exec 1>$LOGFILE 2>&1

# 文件类型
file
```

## ethtool

## cache

```bash
# 缓存信息
dmesg -H | grep cache

# cpu 信息
/proc/cpuinfo
/sys/devices/system/cpu/cpu0/cache/index1

1:
    cache line: 64
    size: 32K
    set: 64
    ways: 8
    share: 0,4

2:
    cache line: 64
    size: 256k
    set: 1024
    ways: 4
    share: 0,4

3:
    cache line: 64
    size: 6144K
    set: 8192
    ways: 12
    share: 0-7
```

## tmux

```bash
sudo apt-get install xclip
```

```bash
copy and paste

copy
  CTRL`+`b [
    Use the arrow keys to go to the position from where you want to start copying.
  CTRL`+SPACE` # to start copying.
    Use arrow keys to go to the end of text you want to copy
  ALT`+`w or CTRL`+`w # to copy into Tmux buffer.

paste
    CTRL`+`b ]


C+b ?  # help

C+b s  # 列出所有会话

C+b "  # 划分上下两个窗格。
C+b %  # 划分左右两个窗格。
C-d    # Closing Panes

C+b c # 创建一个新窗口，状态栏会显示多个窗口的信息。
C+b w # 从列表中选择窗口。
C+b p # 切换到上一个窗口（按照状态栏上的顺序）。
C+b n # 切换到下一个窗口。

# 分离会话
C+b d
tmux detach
C-b D   # give you a choice which of your sessions you want to detach

tmux attach -t 0 # 接入会话
tmux kill-session -t <session-name> # 杀死会话
tmux switch -t <session-name> # 切换会话
tmux new -smes # -s 任务名称

tmux rename-session -t oldname <new-name>  # 重命名
C+b $  # 重命名当前会话

tmux attach -t <session-name>  # 接入会话

C+b s # 切换 session

C+b <arrow key> # 光标切换到其他窗格。<arrow key>是指向要切换到的窗格的方向键，比如切换到下方窗格，就按方向键↓。
C+b ; # 光标切换到上一个窗格。
C+b o # 光标切换到下一个窗格。
C+b { # 当前窗格与上一个窗格交换位置。
C+b } # 当前窗格与下一个窗格交换位置。
C+b Ctrl+o # 所有窗格向前移动一个位置，第一个窗格变成最后一个窗格。
C+b Alt+o # 所有窗格向后移动一个位置，最后一个窗格变成第一个窗格。
C+b x # 关闭当前窗格。
C+b ! # 将当前窗格拆分为一个独立窗口。
C+b z # 当前窗格全屏显示，再使用一次会变回原来大小。
C+b Ctrl+<arrow key> # 按箭头方向调整窗格大小。
C+b q # 显示窗格编号。

# 窗口
C+b <number> # 切换到指定编号的窗口，其中的<number>是状态栏上的窗口编号。
C+b , # 窗口重命名。

tmux -d new -s xx -c /build/ r
```

```bash
tmux new -d -c mes/server/cfg -s cfg ./r
tmux new -d -c mes/server/dc -s dc ./r
tmux new -d -c mes/server/filecmd -s filecmd ./r
tmux new -d -c mes/server/fileserver -s fileserver ./r
tmux new -d -c mes/server/graphdb -s graphdb ./r
tmux new -d -c mes/server/machine -s machine ./r
tmux new -d -c mes/server/oee -s oee ./r
tmux new -d -c mes/server/plc -s plc ./r
tmux new -d -c mes/server/real -s real ./r
tmux new -d -c mes/server/spc -s spc ./r
tmux new -d -c mes/server/timedb -s timedb ./r
tmux new -d -c mes/server/upload -s upload ./r
tmux new -d -c mes/server/mt -s mt ./r
tmux new -d -c mline/line/core -s line ./s

tmux new -d -c mes -s mes
tmux new -d -c mes/deploy/pj -s deploy
tmux new -d -c mline/test -s test
tmux new -d -c mclient -s client
tmux new -d -c mclient/maintain -s mtc
```

### Tmux Plugin Manager

```bash
git clone https://github.com/tmux-plugins/tpm ~/.tmux/plugins/tpm

~/.tmux.conf

# List of plugins
# set -g @plugin 'tmux-plugins/tpm'
# set -g @plugin 'tmux-plugins/tmux-sensible'

set -g mouse on
set -g status-fg  green
set -g status-bg  black
set -g default-terminal "tmux-256color"
set-option -g default-command bash

# Other examples:
# set -g @plugin 'github_username/plugin_name'
# set -g @plugin 'git@github.com:user/plugin'
# set -g @plugin 'git@bitbucket.com:user/plugin'

# Initialize TMUX plugin manager (keep this line at the very bottom of tmux.conf)
# run '~/.tmux/plugins/tpm/tpm'

tmux source ~/.tmux.conf

prefix + I # Installs
prefix + U # updates
prefix + alt + u # uninstall plugins not on the plugin list
```

## nc / ncat

测试端口

```bash
nc localhost 9000 // 向端口写入数据

nc -z -w5 -v SERVER_IP PORT
```

* -z    数据包不包含 payload
* -w5   最多等待 5 秒
* -v    冗余模式

## 磁盘监测

```bash
iostat -x 1
```

## tcpdump

网卡工作在 promiscuous mode 下。

```bash
ifconfig wlo1 down
iwconfig wlo1 mode managed
iwconfig wlo1 mode monitor
ifconfig wlo1 up

tcpdump -i wlo1
```

## Wireshark

### 充许普通用户使用

```bash
cat /etc/group
sudo usermod -a -G wireshark username
```

### 工作流

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

 # 注意 U 的含义是 route is 'up'，G 的含义是这是一个 gateway
route -n

# 显示和管理路由，设备，policy routing and tunnels
ip route
ip link
```

## netstat

输出网络连接，路由表，网卡信息，无效连接，多播成员

```bash
apt-get install net-tools

netstat -tulpn

# list of open sockets
netstat

# routing tables
netstat -r

# table of all network interfaces
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

## curl

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

## xxd

输出16进制数

```bash
echo a | xxd -g 1
```

## strace

查看系统调用

```bash
strace cat /etc/hosts
```

## 系统调用号

```bash
/usr/include/asm-generic/unistd.h
```

## node.js

```bash
curl -sL https://deb.nodesource.com/setup_12.x | sudo -E bash -
sudo apt-get install nodejs
```

## vim

```bash
sudo add-apt-repository ppa:jonathonf/vim
sudo apt-get update

git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim

# complete
plugin: AutoComplPop-master

rustup toolchain add nightly
cargo +nightly install racer
rustup component add rust-src
racer complete std::io::B

Plugin 'racer-rust/vim-racer'
set hidden
let g:racer_cmd = "/home/user/.cargo/bin/racer"
let g:racer_experimental_completer = 1   # show the complete function definition
let g:racer_insert_paren = 1 # insert the parenthesis in the completio
augroup Racer
    autocmd!
    autocmd FileType rust nmap <buffer> gd         <Plug>(rust-def)
    autocmd FileType rust nmap <buffer> gs         <Plug>(rust-def-split)
    autocmd FileType rust nmap <buffer> gx         <Plug>(rust-def-vertical)
    autocmd FileType rust nmap <buffer> gt         <Plug>(rust-def-tab)
    autocmd FileType rust nmap <buffer> <leader>gd <Plug>(rust-doc)
    autocmd FileType rust nmap <buffer> <leader>gD <Plug>(rust-doc-tab)
augroup END


Ctrl+ x, followed by Ctrl+ o

autocmd FileType javascript set omnifunc=javascriptcomplete#CompleteJS
:set omnifunc=htmlcomplete#CompleteTags
:set omnifunc=csscomplete#CompleteCSS
```

./config/flake8

```bash
[flake8]
max-line-length=150
```

### flod command

:autocmd FileType rust setlocal foldmethod=expr foldexpr=getline(v:lnum)=~'^\\s*//'

### auto pair

Plugin 'jiangmiao/auto-pairs'

```bash
    input: [
    output: [|]

    Delete in pair

    input: foo[<BS>]
    output: foo

    Insert new indented line after Return

    input: {|} (press <CR> at |)
    output: {
        |
    }          (press } to close the pair)
    output: {
    }|         (the inserted blank line will be deleted)

    Insert spaces before closing characters, only for [], (), {}

    input: {|} (press <SPACE> at |)
    output: { | }

    input: {|} (press <SPACE>foo} at |)
    output: { foo }|

    input: '|' (press <SPACE> at |)
    output: ' |'

    Skip ' when inside a word

    input: foo| (press ' at |)
    output: foo'

    Skip closed bracket.

    input: []
    output: []

    Ignore auto pair when previous character is \

    input: "\'
    output: "\'"

    Fast Wrap

    input: |[foo, bar()] (press (<M-e> at |)
    output: ([foo, bar()])

    Quick move char to closed pair

    input: (|){["foo"]} (press <M-}> at |)
    output: ({["foo"]}|)

    input: |[foo, bar()] (press (<M-]> at |)
    output: ([foo, bar()]|)

    Quick jump to closed pair.

    input:
    {
        something;|
    }

    (press } at |)

    output:
    {

    }|

    Fly Mode

     input: if(a[3)
     output: if(a[3])| (In Fly Mode)
     output: if(a[3)]) (Without Fly Mode)

     input:
     {
         hello();|
         world();
     }

     (press } at |)

     output:
     {
         hello();
         world();
     }|

     (then press <M-b> at | to do backinsert)
     output:
     {
         hello();}|
         world();
     }

     See Fly Mode section for details

    Multibyte Pairs

     Support any multibyte pairs such as <!-- -->, <% %>, """ """
     See multibyte pairs section for details

    <CR>  : Insert new indented line after return if cursor in blank brackets or quotes.
    <BS>  : Delete brackets in pair
    <M-p> : Toggle Autopairs (g:AutoPairsShortcutToggle)
    <M-e> : Fast Wrap (g:AutoPairsShortcutFastWrap)
    <M-n> : Jump to next closed pair (g:AutoPairsShortcutJump)
    <M-b> : BackInsert (g:AutoPairsShortcutBackInsert)

If <M-p> <M-e> or <M-n> conflict with another keys or want to bind to another keys, add

    let g:AutoPairsShortcutToggle = '<another key>'

to .vimrc, if the key is empty string '', then the shortcut will be disabled.

Options

    g:AutoPairs

    Default: {'(':')', '[':']', '{':'}',"'":"'",'"':'"', "`":"`", '```':'```', '"""':'"""', "'''":"'''"}

    b:AutoPairs

    Default: g:AutoPairs

    Buffer level pairs set.

    g:AutoPairsShortcutToggle

    Default: '<M-p>'

    The shortcut to toggle autopairs.

    g:AutoPairsShortcutFastWrap

    Default: '<M-e>'

    Fast wrap the word. all pairs will be consider as a block (include <>).
    (|)'hello' after fast wrap at |, the word will be ('hello')
    (|)<hello> after fast wrap at |, the word will be (<hello>)

    g:AutoPairsShortcutJump

    Default: '<M-n>'

    Jump to the next closed pair

    g:AutoPairsMapBS

    Default : 1

    Map <BS> to delete brackets, quotes in pair
    execute 'inoremap <buffer> <silent> <BS> <C-R>=AutoPairsDelete()<CR>'

    g:AutoPairsMapCh

    Default : 1

    Map <C-h> to delete brackets, quotes in pair

    g:AutoPairsMapCR

    Default : 1

    Map <CR> to insert a new indented line if cursor in (|), {|} [|], '|', "|"
    execute 'inoremap <buffer> <silent> <CR> <C-R>=AutoPairsReturn()<CR>'

    g:AutoPairsCenterLine

    Default : 1

    When g:AutoPairsMapCR is on, center current line after return if the line is at the bottom 1/3 of the window.

    g:AutoPairsMapSpace

    Default : 1

    Map <space> to insert a space after the opening character and before the closing one.
    execute 'inoremap <buffer> <silent> <CR> <C-R>=AutoPairsSpace()<CR>'

    g:AutoPairsFlyMode

    Default : 0

    set it to 1 to enable FlyMode.
    see FlyMode section for details.

    g:AutoPairsMultilineClose

    Default : 1

    When you press the key for the closing pair (e.g. `)`) it jumps past it.
    If set to 1, then it'll jump to the next line, if there is only whitespace.
    If set to 0, then it'll only jump to a closing pair on the same line.

    g:AutoPairsShortcutBackInsert

    Default : <M-b>

    Work with FlyMode, insert the key at the Fly Mode jumped postion

    g:AutoPairsMoveCharacter

    Default: "()[]{}\"'"

    Map <M-(> <M-)> <M-[> <M-]> <M-{> <M-}> <M-"> <M-'> to
    move character under the cursor to the pair.
```

## vscode

settings.json

```json
  {
    "workbench.colorTheme": "Visual Studio Light",
    "window.zoomLevel": 2,
    "markdownlint.config": {
      "MD024": false
    },
}
```

## 添加sudo用户

### ubuntu

```bash
adduser xx
usermod -aG sudo xx
```

### centos

```bash
usermod -aG wheel xx

or

visudo

root ALL=(ALL) ALL
user ALL=(ALL) ALL
```

## ssh 登录

```bash
ls -al ~/.ssh
ssh-keygen -t rsa -b 4096 -C "mcflym@N123456"
ssh-add ~/.ssh/id_rsa
cat ~/.ssh/id_rsa.pub | ssh username@server.address.com 'cat >> ~/.ssh/authorized_keys'
```

### 登录慢

```bash
vim /etc/ssh/sshd_config

UseDNS no
GSSAPIAuthentication no
IgnoreRhosts no

vim /etc/nsswitch.conf
hosts: remove dns

service sshd restart
```

## sshpass

```bash
sshpass -p 'passwd' ssh xxxxx

sshpass -p '12345678' ssh -o StrictHostKeyChecking=no zs@172.16.2.172 -t "./t.sh"
```

## 键盘映射

### 查看按键值

xev

### 进行键盘映射

xmodmap -e "keycode 128 = F3"

#### 配置文件

```bash
~/.Xmodmap

keycode 128 = F3
keycode 123 = Insert
```

## download

### 多线程下载

```bash
axel
```

## GUI 用户登录设置

```bash
cd /etc/lightdm/lightdm.conf.d
touch my.conf

[SeatDefaults]
allow-guest=false
greeter-hide-users=true
greeter-show-manual-login=true
```

## ftp

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

## 制作iso systemback

```bash
sudo add-apt-repository ppa:nemh/systemback
sudo apt-get update && sudo apt-get install systemback unionfs-fuse

/usr/lib/systemback/sbsustart systemback
/usr/lib/systemback/sbsustart systemback gtk+
```

## npm

```bash
npm install tailwindcss --save-dev
```

### 修改成华为云镜像源

```bash
npm config set registry https://mirrors.huaweicloud.com/repository/npm/
npm config get registry
```

## ibm mq

```bash
apt-get install python-dev
./mqlicense.sh -accept
apt-get install rpm
rpm --prefix /opt/mqm -ivh --nodeps --force-debian MQSeriesRuntime...
rpm --prefix /opt/mqm -ivh --nodeps --force-debian MQSeriesClien...
rpm --prefix /opt/mqm -ivh --nodeps --force-debian MQSeriesSDK...
pip install pymqi
```

## 安装 ubuntu

```bash
startup disk creator  # 刻录
esc 进入 bios

efi 2Gb

lshw # 查看硬件信息
lshw -C display  # 查看显卡信息

sudo ubuntu-drivers devices
sudo ubuntu-drivers install
# set UEFI secure boot enabled
sudo nvidia-smi
```

### 打开终端时全屏或最大化

打开“系统设置”==>“键盘”==>“自定义快捷键”

分别自定义两个快捷键：

a)Full Terminal

命令：gnome-terminal --full-screen

b)Max Terminal

命令：gnome-terminal  --maximize

## other

```bash
dspmqaut -m SVR　-n SVR.LQ -t q -p guest
setmqaut -m SVR　-n SVR.LQ -t q -p guest +put
```

npm config set registry https://registry.npm.taobao.org
npm config get registry

./manage.py oscar_fork_app order yourappsfolder/
git clone https://github.com/django-oscar/django-oscar.git
cd django-oscar
sudo make install
make sandbox
sandbox/manage.py runserver

sudo update-alternatives --config vim
apt-cache search py2

./manage.py oscar_fork_app dashboard.reports gmapp/
./manage.py oscar_fork_statics

mysqldump -ugmshop -pgmshop -d gmshop >db.sql
from oscar.core.loading import get_class
>>> get_class('shipping.repository', 'Repository')

Netcat is a tool for quickly creating TCP sockets from the command line. The following command starts a listening TCP socket on the previously specified port.

$ nc -l 6142

cpu信息
    不为0，支持虚拟化  
    egrep -c '(vmx|svm)' /proc/cpuinfo 
    为0，不是64位      
    egrep -c ' lm ' /proc/cpuinfo  

内核是否为64位
uname -m

版本
cat /etc/issue
lsb_release -a

查看端口
lsof -i:80
netstat -tunlp |grep 80 

systemctl
systemctl list-units            ##列出当前系统服务的状态
systemctl list-unit-files       ##列出服务的开机状态
systemctl status sshd           ##查看指定服务的状态
systemctl stop sshd             ##关闭指定服务
systemctl start sshd            ##开启指定服务
systemctl restart sshd          ##从新启动服务
systemctl enable sshd           ##设定指定服务开机开启
systemctl disable sshd          ##设定指定服务开机关闭
systemctl reload sshd           ##使指定服务从新加载配置
systemctl list-dependencies sshd    ##查看指定服务的倚赖关系
systemctl mask  sshd            ##冻结指定服务
systemctl unmask sshd           ##启用服务
systemctl set-default multi-user.target ##开机不开启图形
systemctl set-default graphical.target  ##开机启动图形

Gnome Tweaks 优化工具

查看已经安装了哪些包

aptitude 

dpkg -l

如果系统使用时间长，安装了许多包查看不便时，可以使用翻页查看

dpkg -l | less

查看软件xxx安装内容

dpkg -L xxx


清除所有已删除包的残余配置文件

dpkg -l |grep ^rc|awk '{print $2}' |sudo xargs dpkg -P

可以清除一些残留无用的配置。


显示系统安装包的统计信息

apt-cache stats

可以统计已经安装包的数量，大小，占用空间等

显示xxx包的信息

apt-cache show xxx

查找文件属于哪个包

dpkg -S filename ：在当前安装的包里查找文件。

apt-file search filename ： 在所有源包里查找文件。

查询软件xxx依赖哪些包

apt-cache depends xxx

查询软件xxx被哪些包依赖

apt-cache rdepends xxx

设置firefox包的状态为 hold；
echo " firefox hold" | dpkg --set-selections

如果想恢复可以更新的状态(install)，执行下面的命令；

echo "firefox install" | dpkg --set-selections

查询所有包的状态；

sudo dpkg --get-selections | more  

查询状态为hold的所有包;

sudo dpkg --get-selections | grep hold

sudo apt-get remove libreoffice-common  
sudo apt-get remove unity-webapps-common  
sudo apt-get -y purge rhythmbox*

lock
/var/lib/dpkg/lock-frontend

查看进程

   pidof   后面跟进程名称，显示此进程的进程号。可以得知此程序有多少个实例在运行。

   打开终端时全屏或最大化：

language support安装中文语言
text entry 加入中文
下载sogou输入法并安装，如果有依赖错误：sudo apt-get -f install
language support->keyboard input method system: fcitx

## firefox

about:config

print.always_print_silent

cp /usr/share/applications/firefox.desktop  ~/.config/autostart/
chmod +x ~/.config/autostart/firefox.desktop

firefox --kiosk

## opcua

```bash
pip3.6 install opcua -i https://mirrors.aliyun.com/pypi/simple/ 
pip install opcua-client
pip install opcua-modeler
```