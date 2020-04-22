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

systemctl stop firewalld
systemctl disable firewalld

firewall-cmd --state
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

docker network create –-driver drivername name
```

## 创建网络

create a network with a subnet and a gateway

```bash
docker network create --driver=bridge --subnet=192.168.2.0/24 --gateway=192.168.2.10 new_subnet
docker network create –-driver bridge new_nw

docker run –it –network=new_nw ubuntu:latest /bin/bash
```

## 网络 driver

### bridge

* creates a private network internal to the host
* all containers get an internal IP address
* containers can access each other, using this internal IP
* your applications run in standalone containers that need to communicate.
* it only provides service discovery, IPAM, and connectivity on a single host

the Docker Engine creates:

* Linux bridges
* internal interfaces
* iptables rules
* host routes

A built-in IPAM driver provides the container interfaces with private IP addresses from the subnet of the bridge network.

### Overlay

* Creates an internal private network that spans across all the nodes participating in the swarm cluster
* Overlay networks facilitate communication between a swarm service and a standalone container, or between two standalone containers on different Docker Daemons.
* IPAM, service discovery, multi-host connectivity, encryption, and load balancing are built in.

### Macvlan

* assign a MAC address to a container, making it appear as a physical device on your network
* the Docker daemon routes traffic to containers by their MAC addresses
* the best choice when you are expected to be directly connected to the physical network, rather than routed through the Docker host’s network stack.
* rather than using any Linux bridging or port mapping, it connects container interfaces directly to host interfaces
* Containers are addressed with routable IP addresses that are on the subnet of the external network.

The macvlan driver uses the concept of a parent interface.This interface can be a host interface such as eth0, a sub-interface, or even a bonded host adaptor which bundles Ethernet interfaces into a single logical interface.

### Host

* removes the network isolation between the docker host
* docker containers to use the host’s networking directly
* you will not be able to run multiple web containers on the same host, on the same port as the port is now common to all containers in the host network.

### None

* containers are not attached to any network and do not have any access to the external network or other containers
* only create a loopback device.
* used when you want to completely disable the networking stack on a container