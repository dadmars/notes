# 安装

# redhat

## 安装

```bash
cd /etc/yum.repos.d
wget -O CentOS-Base.repo http://mirrors.aliyun.com/repo/Centos-7.repo

change $releasever -> 7
change $arch.. -> x86_64 

yum repolist
yum install -y yum-utils device-mapper-persistent-data lvm2
yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo
yum install docker-ce
systemctl start docker
systemctl enable docker

curl -L "https://github.com/docker/compose/releases/download/1.25.4/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose

chmod +x /usr/local/bin/docker-compose
```

## docker 内部无法访问主机

```bash
firewall-cmd --permanent --zone=public --add-rich-rule='rule family=ipv4 source address=172.17.0.0/16 accept'
firewall-cmd --permanent --zone=public --add-rich-rule='rule family=ipv4 source address=172.18.0.0/16 accept'
firewall-cmd --permanent --zone=public --add-rich-rule='rule family=ipv4 source address=172.19.0.0/16 accept'
firewall-cmd --permanent --zone=public --add-rich-rule='rule family=ipv4 source address=172.20.0.0/16 accept'
firewall-cmd --permanent --zone=public --add-rich-rule='rule family=ipv4 source address=172.21.0.0/16 accept'
firewall-cmd --reload
```

# centos

```bash
https://docs.docker.com/install/linux/docker-ee/centos/#package-install-and-upgrade
```

# ubuntu

```bash
sudo apt-get install apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo apt-key fingerprint 0EBFCD88
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
sudo apt-get update
sudo apt-get install docker-ce

sudo systemctl enable docker
sudo systemctl disable docker

# docker-compose
sudo curl -L https://github.com/docker/compose/releases/download/1.17.0/docker-compose-`uname -s`-`uname -m` -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
docker-compose --version

# 不通过root权限访问：
sudo groupadd docker
sudo usermod -aG docker $USER
logout and login

docker --version
docker version
docker info

# 下载 images
docker pull xxx
```

# 删除

```bash
sudo apt-get purge docker-ce
sudo rm -rf /var/lib/docker
```

# log

```bash
docker ps
docker logs xxxxx

# 看最后的30行
docker logs --tail 30 xxxxx
```

# 编译

```bash
# Create image using this directory's Dockerfile
docker build -t friendlyhello .
```

# image

```bash
# List Docker images
docker images
docker image ls
docker image ls --format "{{.Repository}}-{{.Tag}}"

# Remove specified image from this machine
docker rmi <image id>
docker image rm <image id>

# Remove all images from this machine
docker image rm $(docker image ls -a -q)

# Tag <image> for upload to registry
docker tag <image> username/repository:tag

# Log in this CLI session using your Docker credentials
docker login
docker login registry.example.com

# Upload tagged image to registry
docker push username/repository:tag

# save image to file
docker save IMG:VER -o IMG-VER.tar
gzip IMG-VER.tar
```

# 运行

```bash
docker run hello-world

# Run "friendlyname" mapping port 4000 to 80
docker run -p 4000:80 friendlyhello  

# Same thing, but in detached mode
docker run -d -p 4000:80 friendlyhello

# Run image from a registry
docker run username/repository:tag
docker run --name ubuntu_bash --rm -i -t ubuntu bash

docker exec -it <hash> /bin/bash
```

# 容器

```bash
docker container --help

# List Docker containers (running, all, all in quiet mode)
docker container ls
docker container ls --all
docker container ls -aq

# Inspect container
docker inspect <hash>

# Gracefully stop the specified container
docker container stop <hash>

# Force shutdown of the specified container
docker container kill <hash>

# Remove specified container from this machine
docker rm <hash>
docker container rm <hash>

# Remove all containers
docker container rm $(docker container ls -a -q)
docker container rm $(docker container ls -a -q -f status=exited)
```

# 网络

```bash
docker network --help
docker network ls
docker network inspect xxx
```
