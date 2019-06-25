# 基本

```bash
docker --version
docker version
docker info

docker logs

docker container --help
```

# 运行 image

```bash
docker run hello-world

# Run "friendlyname" mapping port 4000 to 80
docker run -p 4000:80 friendlyhello  

# Same thing, but in detached mode
docker run -d -p 4000:80 friendlyhello

# Create image using this directory's Dockerfile
docker build -t friendlyhello .

# List Docker images
docker image ls
docker image ls --format "{{.Repository}}-{{.Tag}}"

# Remove specified image from this machine
docker image rm <image id>

# Remove all images from this machine
docker image rm $(docker image ls -a -q)

# List Docker containers (running, all, all in quiet mode)
docker container ls
docker container ls --all
docker container ls -aq

# Gracefully stop the specified container
docker container stop <hash>

# Force shutdown of the specified container
docker container kill <hash>

# Remove specified container from this machine
docker container rm <hash>

# Remove all containers
docker container rm $(docker container ls -a -q)
docker container rm $(docker container ls -a -q -f status=exited)

# Log in this CLI session using your Docker credentials
docker login
docker login registry.example.com

# Tag <image> for upload to registry
docker tag <image> username/repository:tag

# Upload tagged image to registry
docker push username/repository:tag

# Run image from a registry
docker run username/repository:tag
docker run --name ubuntu_bash --rm -i -t ubuntu bash

docker swarm init --advertise-addr <myvm1 ip>
docker swarm join  --token <token>  <myvm ip>:<port>
docker swarm join-token manager
# Make the worker leave the swarm
docker swarm leave
# Make master leave, kill swarm
docker swarm leave -f

# View nodes in swarm (while logged on to manager)
docker node ls

# Deploy an app; command shell must be set to talk to manager (myvm1), uses local Compose file
docker stack deploy -c <composefile> <appname>
docker stack deploy --with-registry-auth -c docker-compose.yml getstartedlab
docker stack ps getstartedlab
docker stack rm getstartedlab
# List stacks or apps
docker stack ls
# Tear down an application
docker stack rm <appname>

# List running services associated with an app
docker service ls
# List tasks associated with an app
docker service ps <service>

# Inspect task or container
docker inspect <task or container>

docker exec -it ubuntu_bash bash
```

# 安装

```bash
sudo apt-get install apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo apt-key fingerprint 0EBFCD88
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
sudo apt-get update
sudo apt-get install docker-ce
sudo docker run hello-world
```

# 删除

```bash
sudo apt-get purge docker-ce
sudo rm -rf /var/lib/docker
```

# 不通过root权限访问：

```bash
sudo groupadd docker
sudo usermod -aG docker $USER
logout and login
```

# 启动时自动运行

```bash
sudo systemctl enable docker
sudo systemctl disable docker
```

# docker-compose

```bash
sudo curl -L https://github.com/docker/compose/releases/download/1.17.0/docker-compose-`uname -s`-`uname -m` -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
docker-compose --version
```
