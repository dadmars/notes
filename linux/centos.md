# centos

- [centos](#centos)
  - [自定义安装 (kickstart)](#自定义安装-kickstart)
  - [查看版本号](#查看版本号)
  - [yum 缓存](#yum-缓存)
  - [修改成阿里云镜像源](#修改成阿里云镜像源)
  - [docker 安装](#docker-安装)
  - [python2 安装](#python2-安装)
  - [设置ip](#设置ip)

## 自定义安装 (kickstart)

```bash
mkdir -p kickstart_build/isolinux
mkdir -p kickstart_build/isolinux/images
mkdir -p kickstart_build/isolinux/ks
mkdir -p kickstart_build/isolinux/LiveOS
mkdir -p kickstart_build/isolinux/Packages
mkdir -p kickstart_build/utils
mkdir -p kickstart_build/all_rpms
mkdir -p kickstart_build/isolinux/postinstall

cp ~/cdrom/isolinux/* ~/kickstart_build/isolinux
cp ~/cdrom/.discinfo ~/kickstart_build/isolinux
cp -r ~/cdrom/images/* ~/kickstart_build/isolinux/images
cp ~/cdrom/LiveOS ~/kickstart_build/isolinux/LiveOS
cp ~/cdrom/repodata/*.-comps.xml.gz ~/kickstart_build/comps.xml.gz

cd ~/kickstart_build/
gunzip comps.xml.gz
cd ~

cp ks.cfg kickstart_build/isolinux/ks

cd ~/kickstart_build/utils/

wget http://www.smorgasbork.com/content/gather_packages.txt
mv gather_packages.txt gather_packages.pl
chmod +x gather_packages.pl

cd ~/kickstart_build/all_rpms
sudo rpm -Uvh perl-5*-*.el7.x86_64.rpm \
  perl-Business-ISBN-*.el7.noarch.rpm \
  perl-Carp-*.el7.noarch.rpm \
  perl-Compress-Raw-Bzip2-*.el7.x86_64.rpm \
  perl-Compress-Raw-Zlib-*.el7.x86_64.rpm \
  perl-constant-*.el7.noarch.rpm \
  perl-Data-Dumper-*.el7.x86_64.rpm \
  perl-devel-*.el7.x86_64.rpm \
  perl-Digest-*.el7.noarch.rpm \
  perl-Digest-MD5-*.el7.x86_64.rpm \
  perl-Digest-SHA-*.el7.x86_64.rpm \
  perl-Encode-*.el7.x86_64.rpm \
  perl-Encode-Locale-*.el7.noarch.rpm \
  perl-Exporter-*.el7.noarch.rpm \
  perl-ExtUtils-Install-*.el7.noarch.rpm \
  perl-ExtUtils-MakeMaker-*.el7.noarch.rpm \
  perl-ExtUtils-Manifest-*.el7.noarch.rpm \
  perl-ExtUtils-ParseXS-*.el7.noarch.rpm \
  perl-File-Listing-*.el7.noarch.rpm \
  perl-File-Path-*.el7.noarch.rpm \
  perl-File-Temp-*.el7.noarch.rpm \
  perl-Filter-*.el7.x86_64.rpm \
  perl-Getopt-Long-*.el7.noarch.rpm \
  perl-HTML-Parser-*.el7.x86_64.rpm \
  perl-HTML-Tagset-*.el7.noarch.rpm \
  perl-HTTP-Cookies-*.el7.noarch.rpm \
  perl-HTTP-Daemon-*.el7.noarch.rpm \
  perl-HTTP-Date-*.el7.noarch.rpm \
  perl-HTTP-Message-*.el7.noarch.rpm \
  perl-HTTP-Negotiate-*.el7.noarch.rpm \
  perl-HTTP-Tiny-*.el7.noarch.rpm \
  perl-IO-Compress-*.el7.noarch.rpm \
  perl-IO-HTML-*.el7.noarch.rpm \
  perl-IO-Socket-IP-*.el7.noarch.rpm \
  perl-IO-Socket-SSL-*.el7.noarch.rpm \
  perl-libs-*.el7.x86_64.rpm \
  perl-libwww-perl-*.el7.noarch.rpm \
  perl-LWP-MediaTypes-*.el7.noarch.rpm \
  perl-macros-*.el7.x86_64.rpm \
  perl-Net-HTTP-*.el7.noarch.rpm \
  perl-Net-LibIDN-*.el7.x86_64.rpm \
  perl-Net-SSLeay-*.el7.x86_64.rpm \
  perl-parent-*.el7.noarch.rpm \
  perl-PathTools-*.el7.x86_64.rpm \
  perl-Pod-Escapes-*.el7.noarch.rpm \
  perl-Pod-Perldoc-*.el7.noarch.rpm \
  perl-Pod-Simple-*.el7.noarch.rpm \
  perl-Pod-Usage-*.el7.noarch.rpm \
  perl-podlators-*.el7.noarch.rpm \
  perl-Scalar-List-Utils-*.el7.x86_64.rpm \
  perl-Socket-*.el7.x86_64.rpm \
  perl-srpm-macros-*.el7.noarch.rpm \
  perl-Storable-*.el7.x86_64.rpm \
  perl-Test-Harness-*.el7.noarch.rpm \
  perl-Text-ParseWords-*.el7.noarch.rpm \
  perl-Thread-Queue-*.el7.noarch.rpm \
  perl-threads-*.el7.x86_64.rpm \
  perl-Time-Local-*.el7.noarch.rpm \
  perl-TimeDate-*.el7.noarch.rpm \
  perl-URI-*.el7.noarch.rpm \
  perl-WWW-RobotRules-*.el7.noarch.rpm \
  perl-XML-Filter-BufferText-*.el7.noarch.rpm \
  perl-XML-NamespaceSupport-*.el7.noarch.rpm \
  perl-XML-Parser-*.el7.x86_64.rpm \
  perl-XML-SAX-*.el7.noarch.rpm \
  perl-XML-Simple-*.el7.noarch.rpm \
  gdbm-devel-*.x86_64.rpm \
  glibc-devel-*.x86_64.rpm \
  glibc-headers-*.x86_64.rpm \
  kernel-headers-*.x86_64.rpm \
  libdb-devel-*.x86_64.rpm \
  systemtap-sdt-devel-*.x86_64.rpm \
  mailcap-*.noarch.rpm

~/kickstart_build/utils/gather_packages.pl ~/kickstart_build/comps.xml \
  ~/kickstart_build/all_rpms ~/kickstart_build/isolinux/Packages x86_64 \
  ftp-server \
	
cd ~/kickstart_build/all_rpms
sudo rpm -Uvh \
  createrepo-*.el7.noarch.rpm \
  deltarpm-*.el7.x86_64.rpm \
  python-deltarpm-*.el7.x86_64.rpm \
  libxml2-python-*.el7.x86_64.rpm

cd ~/kickstart_build/isolinux
createrepo -g ~/kickstart_build/comps.xml . 

cd ~/kickstart_build/all_rpms
sudo rpm -Uvh \
  genisoimage-*.el7.x86_64.rpm \
  libusal-*.el7.x86_64.rpm
	
isolinux/isolinux.cfg file like this:
label ks
  menu label ^Kickstart
  menu default
  kernel vmlinuz
  append initrd=initrd.img inst.stage2=hd:LABEL=CentOS\x207\x20x86_64 inst.ks=cdrom:/dev/cdrom:/ks/ks.cfg

cd ~/kickstart_build
chmod 664 isolinux/isolinux.bin
mkisofs -o custom.iso -b isolinux.bin -c boot.cat -no-emul-boot \
  -V 'CentOS 7 x86_64' \
  -boot-load-size 4 -boot-info-table -R -J -v -T isolinux/

%post --nochroot
#!/bin/sh

set -x -v
exec 1>/mnt/sysimage/root/kickstart-stage1.log 2>&1

echo "==> copying files from media to install drive..."
cp -r /run/install/repo/postinstall /mnt/sysimage/root

%end

%post
#!/bin/sh

set -x -v
exec 1>/root/kickstart-stage2.log 2>&1

ls -l /root/postinstall

%end
```

正常安装，记录保存在 /root/anaconda-ks.cfg, 对此文件进行修改

确认修改正确

```bash
yum install pykickstart
ksvalidator /path/to/kickstart.ks
```

查看系统各版本的语法差异

```bash
ksverdiff -f RHEL6 -t RHEL7
```

启动时使用选项

```bash
inst.ks=location

To load your Kickstart file automatically without having to specify the inst.ks= boot option, name the file ks.cfg and place it on a storage volume labeled OEMDRV.
```

语法

```bash
firewall --enabled|--disabled device [options]
```

## 查看版本号

```bash
cat /etc/centos-release
7.9.2009
7.5.1004
```

## yum 缓存

```bash
# 保留已下载的文件而不是删除
# 设置 /etc/yum.conf 中的 keepcache 选项为 1
/var/cache/yum/

yum check-update
yum update

yum install -y net-tools bzip2 zip unzip vim tcpdump nmap
```

## 修改成阿里云镜像源

```bash
mv /etc/yum.repos.d/CentOS-Base.repo /etc/yum.repos.d/CentOS-Base.repo.backup
wget -O /etc/yum.repos.d/CentOS-Base.repo http://mirrors.aliyun.com/repo/Centos-6.repo
yum makecache
```

## docker 安装

```bash
sudo yum install -y yum-utils device-mapper-persistent-data lvm2
sudo yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo

sudo yum install docker-ce docker-ce-cli containerd.io
    or:
        yum list docker-ce --showduplicates | sort -r
        sudo yum install docker-ce-<VERSION_STRING> docker-ce-cli-<VERSION_STRING> containerd.io

sudo systemctl start docker
```

## python2 安装

```bash
sudo yum install epel-release
sudo yum install python-pip
```

## 设置ip

```bash
nmtui
systemctl start network.service
```