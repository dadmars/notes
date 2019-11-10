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

```bash
```

# 常用命令

```bash
# 显示所有数据库
show databases
use xxx

# 查看表格
show measurements
show tag keys from xx
show field keys from xx
show tag values from xx with key=msn

# 从一个表把一些列移到另一个表中, group by * 保证 tag key
SELECT useful_field, nice_tag INTO new_measurement FROM measurement GROUP BY *

# 删除表格

############################

# 插入数据
INSERT table,tag1=serverA,tag2=us_west field1=0.64

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
