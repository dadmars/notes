# centos

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
