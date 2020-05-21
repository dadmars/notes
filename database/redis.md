# command

docker exec -it af34b redis-cli

# keys

* 二进制安全，key 值不仅仅是字符串，还可以是图片。空字符串也可以作为一个 key
* 对 key 的命名可以定义模板： object-type:id
* 可以设置 key 的生存时间

# Hashes

一个 key 值可以有多个成员，对应于对象

```js
hmset user:1000 username antirez birthyear 1977 verified 1
> hget user:1000 username
"antirez"
> hget user:1000 birthyear
"1977"
> hgetall user:1000
1) "username"
2) "antirez"
3) "birthyear"
4) "1977"
5) "verified"
6) "1"

 hmget user:1000 username birthyear no-such-field
1) "antirez"
2) "1977"
3) (nil)

> hincrby user:1000 birthyear 10
(integer) 1987
> hincrby user:1000 birthyear 10
(integer) 1997

```