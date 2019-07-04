# 网络相关命令

## ip地址

### 本地ip地址

hostname -I
ip address

### 路由器ip地址

ip route | grep default
route -n
    注意 U 的含义是 route is 'up'，G 的含义是这是一个 gateway
netstat -r -n

## 扫描服务

nmap -v

## 无线网卡配置

iwconfig

# 键盘映射

## 查看按键值

xev

## 进行键盘映射

xmodmap -e "keycode 128 = F3"

### 配置文件

~/.Xmodmap

keycode 128 = F3
keycode 123 = Insert
