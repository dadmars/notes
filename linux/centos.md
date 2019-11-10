# 查看版本号

```bash
cat /etc/centos-release
```

# yum 缓存

```bash
# 保留已下载的文件而不是删除
# 设置 /etc/yum.conf 中的 keepcache 选项为 1
/var/cache/yum/
```

# docker 安装

```bash
sudo yum sudo yum install -y yum-utils device-mapper-persistent-data lvm2
```
