# influxdb

```bash
curl -G 'http://localhost:8086/query?pretty=true' --data-urlencode "db=realtime" --data-urlencode "q=SELECT \"value\" FROM \"cpu_load_short\" WHERE \"region\"='us-west'"

influx -precision rfc3339 -username realtime -password 'realtime'

docker run --rm influxdb influxd config > influxdb.conf
docker run -p 8086:8086 -v influxdb:/var/lib/influxdb influxdb  
docker run -p 8086:8086 -v $PWD:/var/lib/influxdb influxdb
docker run -p 8086:8086 -e INFLUXDB_AUTH_ENABLED=true influxdb
docker run -p 8086:8086 -p 2003:2003  -e INFLUXDB_GRAPHITE_ENABLED=true  influxdb

docker exec -it influxdb influx

insert cpu,host=serverA,region=us_west value=0.64
insert payment,device=mobile,product=Notepad,method=credit billed=33,licenses=3i 1434067467100293230
insert stock,symbol=AAPL bid=127.46,ask=127.48
insert temperature,machine=unit42,type=assembly external=25,internal=37 1434067467000000000

curl -i -XPOST http://localhost:8086/query --data-urlencode "q=CREATE DATABASE mydb"
curl -i -XPOST 'http://localhost:8086/write?db=mydb' --data-binary 'cpu_load_short,host=server01,region=us-west value=0.64 1434055562000000000'

cd /var/lib/influxdb
find -name "*.idx" | xargs rm
```

## 配置

### 设置权限

```bash
docker ps | grep influxdb
docker exec -it xxx influx

> CREATE USER admin WITH PASSWORD 'admin' WITH ALL PRIVILEGES
> CREATE USER realtime WITH PASSWORD 'realtime' WITH ALL PRIVILEGES
> CREATE DATABASE realtime
```

/etc/influxdb/influxdb.conf

* TCP port 8086 is used for client-server communication over InfluxDB’s HTTP API
* TCP port 8088 is used for the RPC service for backup and restore

InfluxDB uses a host’s local time in UTC to assign timestamps to data and for coordination purposes.

We recommend using two SSD volumes. One for the influxdb/wal and one for the influxdb/data. Depending on your load each volume should have around 1k-3k provisioned IOPS. The influxdb/data volume should have more disk space with lower IOPS and the influxdb/wal volume should have less disk space with higher IOPS.

Each machine should have a minimum of 8G RAM.

```bash
[meta]
  dir = "/mnt/db/meta"
[data]
  dir = "/mnt/db/data"
  ...
wal-dir = "/mnt/influx/wal"
  ...

[hinted-handoff]
    ...
dir = "/mnt/db/hh"
    ...

chown influxdb:influxdb /mnt/influx
chown influxdb:influxdb /mnt/db

influx -precision rfc3339
```

<measurement>[,<tag-key>=<tag-value>...] <field-key>=<field-value>[,<field2-key>=<field2-value>...] [unix-nano-timestamp]

## 多个查询用 ; 隔开

```bash
curl -G 'http://localhost:8086/query?pretty=true' --data-urlencode "db=mydb" --data-urlencode "q=SELECT \"value\" FROM \"cpu_load_short\" WHERE \"region\"='us-west';SELECT count(\"value\") FROM \"cpu_load_short\" WHERE \"region\"='us-west'"
```

## 多行写入，换行分隔

```bash
curl -i -XPOST 'http://localhost:8086/write?db=mydb' --data-binary 'cpu_load_short,host=server02 value=0.67
cpu_load_short,host=server02,region=us-west value=0.55 1422568543702900257
cpu_load_short,direction=in,host=server01,region=us-west value=2.0 1422568543702900257'
```

## 文件写入

不超过 5000

```bash
cpu_data.txt

cpu_load_short,host=server02 value=0.67
cpu_load_short,host=server02,region=us-west value=0.55 1422568543702900257
cpu_load_short,direction=in,host=server01,region=us-west value=2.0 1422568543702900257

curl -i -XPOST 'http://localhost:8086/write?db=mydb' --data-binary @cpu_data.txt
```

## 返回值

* 2xx: If your write request received HTTP 204 No Content, it was a success!
* 4xx: InfluxDB could not understand the request.
* 5xx: The system is overloaded or significantly impaired.

## 常用命令

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

# 插入数据
INSERT table,tag1=serverA,tag2=us_west field1=0.64

# 查看数据
select * from table_name;
```

## 备份

### 备份数据库

```bash
docker exec -it xxxxx /bin/bash
influxd backup -portable -database realtime ./back
tar czf back.tar.gz ./back
rm -rf ./back
exit

docker cp xxxxx:/back.tar.gz .
```

### 还原数据库

```bash
docker ps | grep infl # 得到容器id
docker cp back.tar.gz xxxxx:/
docker exec -it xxxxx /bin/bash
tar xf ./back.tar.gz
ls  # 假设这里输出目录为 back_2019-12-15
influxd restore -portable ./back_2019-12-15
```

## tsi

```bash
docker stop influxdb
docker rm influxdb

INFLUXDB_DATA_INDEX_VERSION=tsi1

docker run influxdb

influx_inspect buildtsi -datadir=/var/lib/influxdb/data -waldir=/var/lib/influxdb/wal

find /var/lib/influxdb -type d -name index

influxd cofig

docker stop influxdb
docker rm influxdb
docker run influxdb
```

## cpu 数量

lscpu
nproc

GOMAXPROCS=4

alter retention policy "autogen" on "mydb" duration 2w shard duration 1w replication 1 default
show retention policies on realtime
alter retention policy "autogen" on "realtime" shard duration 72h

SHOW RETENTION POLICIES ON mydb

show retention policies on "groundwork"
create retention policy "groundwork_10_weeks" on "groundwork" duration 10w replication 1 default
alter retention policy "groundwork_56_weeks" on "groundwork" default

duration: 保存多长时间数据
replication factor: 在集群中保存多少份拷贝
shard group duration: 在 shard groups 中覆盖多长时间的数据
          时间为经常查询的2倍
          each shard group ends up with at least 100,000 points per group—you want more data per shard, but not too much data.
          each shard group has at least 1,000 points per series.

/usr/lib/influxdb/scripts/influxd-systemd-start.sh
comment out line 34 (exit 1) – < this is a bad idea in the long run but will allow you to test if it works
or set the sleep command in line 29 to 10 seconds (depending on how long does it take your influx to start, for me 10 cycles (line 25) of 10 seconds was enough)