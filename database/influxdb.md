# 配置

## 设置权限

```bash
docker ps | grep influxdb
docker exec -it xxx influx

> CREATE USER admin WITH PASSWORD 'admin' WITH ALL PRIVILEGES
> CREATE USER realtime WITH PASSWORD 'realtime' WITH ALL PRIVILEGES
> CREATE DATABASE realtime
```

## 修改权限

## 设置远程访问

# 连接数据库

# 重启服务

# 运行脚本

# 删除数据库内容

# 常用命令

```bash
# 显示所有数据库
show databases
use xxx

# 查看表格
show measurements

# 删除表格

############################

# 插入数据

# 更新数据

# 删除数据

# 查看数据
select * from table_name;
```

# 备份

## 备份数据库

## 还原数据库

# 主从同步

## 从服务器配置

# 修改数据库文件位置
