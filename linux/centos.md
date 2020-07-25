# centos

## 自定义安装 (kickstart)

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
```

## yum 缓存

```bash
# 保留已下载的文件而不是删除
# 设置 /etc/yum.conf 中的 keepcache 选项为 1
/var/cache/yum/
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
