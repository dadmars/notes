# node.js

```bash
curl -sL https://deb.nodesource.com/setup_12.x | sudo -E bash -
sudo apt-get install nodejs
```

# 快捷键

## 打开终端时全屏或最大化

打开“系统设置”==>“键盘”==>“自定义快捷键”

分别自定义两个快捷键：

a)Full Terminal

命令：gnome-terminal --full-screen

b)Max Terminal

命令：gnome-terminal  --maximize

# vim

```bash
git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim
```

.vimrc
```bash
set guifont=Bitstream\ Vera\ Sans\ Mono\ 14,Fixed\ 14
set guifontwide=Microsoft\ Yahei\ 14,WenQuanYi\ Zen\ Hei\ 14
set linespace=4

map <F3> :BufExplorer <CR>
map <F9> :PluginInstall <CR>
map <F10> :BundleUpdate <CR>
map <F11> :BundleClean <CR>
map <C-n> :NERDTreeToggle<CR>

autocmd bufenter * if (winnr("$") == 1 && exists("b:NERDTree") && b:NERDTree.isTabTree()) | q | endif
autocmd FileType c,cpp,java,sh autocmd BufWritePre <buffer> :%s/\s\+$//e

set nocompatible
filetype off
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()
Plugin 'gmarik/Vundle.vim'
Plugin 'jlanzarotta/bufexplorer'

Plugin 'scrooloose/nerdtree'
Plugin 'scrooloose/nerdcommenter'

Plugin 'rust-lang/rust.vim'

Plugin 'vim-flake8'

Plugin 'pangloss/vim-javascript'
Plugin 'mxw/vim-jsx'

Plugin 'MarcWeber/vim-addon-mw-utils'
Plugin 'tomtom/tlib_vim'
Plugin 'garbas/vim-snipmate'
Plugin 'honza/vim-snippets'

call vundle#end()
filetype plugin indent on
syntax on

let NERDTreeIgnore = ['\.pyc$', '\~$', '\.pyo$']

autocmd BufWritePost *.py call Flake8()
let g:flake8_show_in_file=1
let g:flake8_show_in_gutter=1
let g:flake8_show_quickfix=1

let g:rustfmt_autosave = 1

set tabstop=4
set softtabstop=4
set shiftwidth=4
set expandtab
set autoindent

hi BadWhitespace guifg=gray guibg=red ctermfg=gray ctermbg=red
au BufRead,BufNewFile *.py,*.pyw,*.c,*.h,*.html,*.css,*.htm match BadWhitespace /\s\+$/
```

./config/flake8

```bash
[flake8]
max-line-length=150
```

# 安装中文输入法

language support安装中文语言

text entry 加入中文

language support -> keyboard input method system: fcitx

fcitx configuration 加入sogou pinyin和wubi

```bash
sudo apt-get install fonts-droid-fallback
sudo apt remove fcitx* && sudo apt autoremove
sudo dpkg -i ~/Downloads/sogoupinyin*.deb; sudo apt -f install
sudo reboot
```

# vscode

## setting font size

window.zoomlevel

# 添加sudo用户

```bash
adduser xx
usermod -aG sudo xx
```

# ssh 登录

```bash
ls -al ~/.ssh
ssh-keygen -t rsa -b 4096 -C "mcflym@N123456"
ssh-add ~/.ssh/id_rsa
cat ~/.ssh/id_rsa.pub | ssh username@server.address.com 'cat >> ~/.ssh/authorized_keys'
```

## sshpass

```bash
sshpass -p 'passwd' ssh xxxxx
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

# ibm mq

```bash
apt-get install python-dev
./mqlicense.sh -accept
apt-get install rpm
rpm --prefix /opt/mqm -ivh --nodeps --force-debian MQSeriesRuntime...
rpm --prefix /opt/mqm -ivh --nodeps --force-debian MQSeriesClien...
rpm --prefix /opt/mqm -ivh --nodeps --force-debian MQSeriesSDK...
pip install pymqi
```

Debug

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

安装中文语言包
sudo apt-get install  language-pack-zh-han*

Gnome Tweaks 优化工具

查看已经安装了哪些包

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
fcitx configuration 加入sogou pinyin和wubi