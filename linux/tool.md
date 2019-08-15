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
/etc/vsftpd.conf
write_enable=YES
local_umask=022
chroot_local_user=YES

# add new line
allow_writeable_chroot=YES
pasv_min_port=40000
pasv_max_port=40100
########################################

sudo systemctl restart vsftpd

sudo useradd -m ftpuser -s /usr/sbin/nologin
sudo passwd useradd
echo "/usr/sbin/nologin" | sudo tee -a /etc/shells
```

ftp 会阻止 shell 不在 /etc/shells 的用户登录

# 花生壳

phddns

# Wireshark

```bash
cat /etc/group
sudo usermod -a -G wireshark username
```

# server 安装 gui

sudo apt-get install --no-install-recommends ubuntu-desktop

# 制作iso systemback

```bash
sudo add-apt-repository ppa:nemh/systemback
sudo apt-get update && sudo apt-get install systemback unionfs-fuse

/usr/lib/systemback/sbsustart systemback
/usr/lib/systemback/sbsustart systemback gtk+
```
