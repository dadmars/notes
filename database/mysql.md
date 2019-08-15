# 配置

## 支持中文

```bash
sudo -i

cd /etc/mysql/mysql.conf.d
cp mysqld.cnf mysqld.cnf_bk

# 添加
[mysqld]
character-set-server=utf8
```

## 设置权限

用户名为 userxxx，且用户所在的机器 IP 为: 192.868.10.20。此用户可以访问数据库 dbxxx。

```bash
mysql -u root -p xxx

mysql> create user realtime identified by 'realtime';
mysql> CREATE DATABASE dbxxx;
mysql> GRANT ALL ON dbxxx.* TO userxxx@'192.868.10.20' IDENTIFIED BY 'PASSWORD';
```

## 修改权限

```bash
mysql> use mysql;
mysql> update db set Host='192.868.10.20' where Db='dbxx';
mysql> update user set Host='192.868.10.20' where user='userxx';

# 如果不限制访问者的IP，可以将 Host 设为 %
mysql> update db set Host='%' where Db='dbxx';
mysql> update user set Host='%' where user='userxx';
```

## 设置 mysql 远程访问

默认情况， mysql 禁止远程访问。

```bash
sudo -i

# 编辑配置文件
cd /etc/mysql/mysql.conf.d
cp mysqld.cnf mysqld.cnf_bk

# 找到下面字段：

# 将 skip-networking 字段删除或注释掉。并修改： bind-address=YOUR-SERVER-IP
# bind-address : 机器的 IP 地址
# skip-networking : 不能使用 TCP/IP 连接，只能使用 Unix sockets。如果只允许本地访问 mysql，可以设置此属性。

[mysqld]
port            = 3306
bind-address    = 65.55.55.2
# skip-networking
```

# 连接数据库

mysql -h 192.168.5.116 -P 3306 -u root -p123456

# 重启服务

```bash
/etc/init.d/mysql restart

systemctl restart mysql
```

# 运行脚本

```bash
mysql> source xxx
```

# 删除数据库内容

```bash
sudo rm -rf  /var/lib/mysql
sudo rm -rf /etc/mysql
```

# mysql 数据库常用命令

```bash
# 显示所有数据库
show databases;

# 新建表格
CREATE TABLE table_name (column_name column_type);

# 查看表格
use realtime;
show tables;
describe table_name;

# 删除表格
drop table table_name;

############################

# 增加表格列
alter table table_name add column column_name varchar(30);

# 删除表格列
alter table table_name drop column column_name;

############################

# 插入数据
INSERT INTO table_name ( field1, field2,...fieldN ) VALUES ( value1, value2,...valueN );

# 更新数据
update table_name set age=age+1 where xxx;

# 删除数据
DELETE FROM table_name where xxx
```

# 备份

## 备份数据库

```bash
# dbname: 数据库名称
# table1和table2: 要备份的表的名称，为空则整个数据库备份
# BackupName.sql: 计备份文件名称。通常将数据库被分成一个后缀名为sql的文件
mysqldump -u username -p dbname table1 table2 ...-> BackupName.sql

mysqldump -u root -p test person > ./backup.sql
```

## 还原数据库

```bash
mysql -u root -p [dbname] < backup.sq

mysql -u root -p < ./backup.sql
```

# 主从同步

## 主服务器配置

```bash
cd /etc/mysql/mysql.conf.d
cp mysqld.cnf mysqld.cnf_bk

# bind-address : 机器的 IP 地址
# server-id : 服务器唯一标识；
# log_bin : 启动MySQL二进制日志；
# binlog_do_db : 指定记录二进制日志的数据库；
# binlog_ignore_db : 指定不记录二进制日志的数据库。

[mysqld]
bind-address = 65.55.55.2
server-id = 1
log_bin = /var/log/mysql/mysql-bin.log
binlog_do_db = include_database_name
binlog_ignore_db = include_database_name

# 查看log_bin是否生效
mysql> show variables like 'log_bin';

# 创建从服务器用到的账户和权限
mysql> grant replication slave on *.* to 'rep'@'192.168.119.137' identified by 'xxx';
mysql> grant replication slave on *.* to 'masterbackup' @' 192.168.119.137' identified by 'masterbackup';

systemctl restart mysql

# 查看状态
mysql> show master status;
mysql> show master logs;
```

## 从服务器配置

```bash
cd /etc/mysql/mysql.conf.d
cp mysqld.cnf mysqld.cnf_bk

# 授权数据库远程连接
# master_host: 主服务器的IP地址:
# master_port: 主服务器的端口
# master_log_file: 对应show master status显示的File列：mysql-bin.000007
# master_log_pos: 对应Position列：107

mysql> grant all privileges on *.* to root@"%" identified by "xxx" with grant option;
mysql> CHANGE MASTER TO
    -> MASTER_HOST='192.168.119.95',
    -> MASTER_PORT=3306,
    -> MASTER_USER='masuserxxx',
    -> MASTER_PASSWORD='maxxxx',
    -> MASTER_LOG_FILE='mysql-bin.000007',
    -> MASTER_LOG_POS=107;

# 启动slave数据同步
mysql> start slave;

# 查看Slave信息
# Slave_IO_Running和Slave_SQL_Running都为yes才表示同步成功。
mysql> show slave status;
```

# 修改数据库文件位置

```bash
# 复制原数据
mkdir -p /data/mysql
cp -R /var/lib/mysql/* /data/mysql
chown -R mysql:mysql /data/mysql

# 修改配置文件
/etc/mysql/my.cnf  
datadir = /data/mysql  

# 修改启动文件
/etc/apparmor.d/usr.sbin.mysqld  

# 找到:
# /var/lib/mysql r  
# /var/lib/mysql/** rwk  
# 修改成  
/data/mysql r  
/data/mysql/** rwk

# 重启服务
systemctl restart mysql
```
