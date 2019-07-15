# 网络相关命令

## tcpdump

网卡工作在 promiscuous mode 下。

```bash
ifconfig wlo1 down
iwconfig wlo1 mode managed
iwconfig wlo1 mode monitor
ifconfig wlo1 up

tcpdump -i wlo1

// 添加一个别名mon0，monitor模式
iwconfig wlo1 interface add mon0 type monitor
// 启用mon0，默认不启用
ifconfig mon0 up
```

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

# 键盘映射

## 查看按键值

xev

## 进行键盘映射

xmodmap -e "keycode 128 = F3"

### 配置文件

~/.Xmodmap

keycode 128 = F3
keycode 123 = Insert
