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
