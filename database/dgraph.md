# 配置

## 支持中文

## 设置权限

## 修改权限

## 设置远程访问

# 连接数据库

# 重启服务

# 运行脚本

# 删除数据库内容

# mysql 数据库常用命令

```bash
# 显示所有数据库

# 新建表格

# 查看表格

# 删除表格

############################

# 增加表格列

# 删除表格列

############################

# 插入数据

# 更新数据

# 删除数据
```

# 备份

## 备份数据库

```bash
container id: xxxxx

docker exec -it xxxxx /bin/bash
curl localhost:8080/admin/export
ls export
exit

docker cp xxxxx:/dgraph/export/dgraph-1-2019-08-13-09-52.rdf.gz .
docker cp xxxxx:/dgraph/export/dgraph-1-2019-08-13-09-52.schema.gz .
```

## 还原数据库

```bash
docker cp dgraph-1-2019-08-13-09-52.rdf.gz xxxxx:/dgraph/
docker cp dgraph-1-2019-08-13-09-52.schema.gz xxxxx:/dgraph/

# get zeroip
docker network ls
docker network inspect xxx

docker exec -it xxxxx /bin/bash
dgraph live -r <path-to-rdf-gzipped-file> -s <path-to-schema-file> -z zeroip:5080
rm -f *.gz
exit
```

# 主从同步

## 从服务器配置

# 修改数据库文件位置
