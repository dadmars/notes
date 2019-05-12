# 修改数据库文件位置

## 复制原数据

```bash
mkdir -p /data/mysql
cp -R /var/lib/mysql/* /data/mysql
chown -R mysql:mysql /data/mysql
```

## 修改配置文件

```bash
sudo vim /etc/mysql/my.cnf  
```

设置:

```bash
datadir = /data/mysql  
```

## 修改启动文件

```bash
vim /etc/apparmor.d/usr.sbin.mysqld  
```

找到:

```bash
/var/lib/mysql r  
/var/lib/mysql/** rwk  
```

修改成  

```bash
/data/mysql r  
/data/mysql/** rwk
```

## 重启服务

```bash
/etc/init.d/mysql restart
```

或

```bash
systemctl restart mysql
```

# 使用 mysqldump 备份数据库

## 备份数据库

```bash
mysqldump -u username -p dbname table1 table2 ...-> BackupName.sql
```

* dbname: 数据库名称
* table1和table2: 要备份的表的名称，为空则整个数据库备份
* BackupName.sql: 计备份文件名称。通常将数据库被分成一个后缀名为sql的文件

如： 使用root用户备份test数据库下的person表

```bash
mysqldump -u root -p test person > ./backup.sql
```

## 还原数据库

```bash
mysql -u root -p [dbname] < backup.sq
```

如：

```bash
mysql -u root -p < ./backup.sql
```

# mysql 数据库常用命令

## 显示所有数据库

```bash
show databases;
```

## 新建数据库

```bash
create database realtime;
```

## 新建用户

新建用户并分配权限使其可访问数据库

```bash
create user realtime identified by 'realtime';
grant all on realtime.* to 'realtime';
```

## 查看表格

使用数据库，显示所有表，显示表结构

```bash
use realtime;
show tables;
describe table_name;
```

## 编辑表格

新建表格

```bash
CREATE TABLE table_name (column_name column_type);
```

删除表格

```bash
drop table table_name;
```

删除表格列

```bash
alter table table_name drop column column_name;
```

增加表格列

```bash
alter table table_name add column column_name varchar(30);
```

## 编辑数据

删除数据

```bash
DELETE FROM table_name where xxx
```

更新数据

```bash
update table_name set age=age+1 where xxx;
```

插入数据

```bash
INSERT INTO table_name ( field1, field2,...fieldN ) VALUES ( value1, value2,...valueN );
```

# 删除 mysql 数据库

```bash
sudo rm -rf  /var/lib/mysql
sudo rm -rf /etc/mysql
```

# 设置 mysql 支持中文

## 进入 root 权限

```bash
sudo -i
```

## 编辑配置文件

```bash
cd /etc/mysql/mysql.conf.d
cp mysqld.cnf mysqld.cnf_bk
vim mysqld.cnf
```

找到下面字段：

```bash
[mysqld]
```

添加：

```bash
character-set-server=utf8
```

## 重启服务

```bash
/etc/init.d/mysql restart
```

或

```bash
systemctl restart mysql
```

# 设置 mysql 远程访问

默认情况， mysql 禁止远程访问。

## 进入 root 权限

```bash
sudo -i
```

## 编辑配置文件

```bash
cd /etc/mysql/mysql.conf.d
cp mysqld.cnf mysqld.cnf_bk
vim mysqld.cnf
```

找到下面字段：

```bash
[mysqld]
```

将 skip-networking 字段删除或注释掉。并修改：

```bash
bind-address=YOUR-SERVER-IP
```

如：

```bash
[mysqld]
...
port            = 3306
bind-address    = 65.55.55.2
# skip-networking
...
```

* bind-address : 机器的 IP 地址
* skip-networking : 不能使用 TCP/IP 连接，只能使用 Unix sockets。如果只允许本地访问 mysql，可以设置此属性。

## 重启服务

```bash
/etc/init.d/mysql restart
```

或

```bash
systemctl restart mysql
```

## 为数据库和用户分配权限

```bash
mysql -u root -p xxx

mysql> CREATE DATABASE dbxxx;
mysql> GRANT ALL ON dbxxx.* TO userxxx@'192.868.10.20' IDENTIFIED BY 'PASSWORD';
```

上面表示，用户名为 userxxx，且用户所在的机器 IP 为: 192.868.10.20。此用户可以访问数据库 dbxxx。

## 为已存在的数据库和用户分配权限

```bash
mysql> use mysql;
mysql> update db set Host='192.868.10.20' where Db='dbxx';
mysql> update user set Host='192.868.10.20' where user='userxx';
```

如果不限制访问者的IP，可以将 Host 设为 %

```bash
mysql> use mysql;
mysql> update db set Host='%' where Db='dbxx';
mysql> update user set Host='%' where user='userxx';
```