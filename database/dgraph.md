# 配置

## 支持中文

## 设置权限

## 修改权限

## 设置远程访问

# 连接数据库

# 重启服务

# 运行脚本

# 删除数据库内容

# 常用命令

```bash
# 插入数据

# 更新数据

# 删除数据

# 查看数据
curl localhost:8080/query -XPOST -d '
{
    me(func: uid(0x1)) {
        name
        uid
        equip {
            name
            uid
        }
    }
}' | python -m json.tool | less
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

```bash
--replicas
    we recommend setting --replicas to 1, 3 or 5 (not 2 or 4). This allows 0, 1, or 2 nodes serving the same group to be down, respectively without affecting the overall health of that group.
```

## 从服务器配置

# 修改数据库文件位置
