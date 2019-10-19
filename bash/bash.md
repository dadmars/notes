# shell 种类

cat /etc/shells

## 用户的默认 shell

cat /etc/passwd

## 登录 bash 的配置文件

* /etc/profile
* ~/.bash_profile, ~/.bash_login or ~/.profile: first existing readable file is read
* ~/.bash_logout upon logout.

## no-login bash 的配置文件

* ~/.bashrc

# Hello, world

```bash
#!/bin/bash

echo "hello word"
```

# 注释

```bash
# comment
```

# 转义

## \

## ''

```bash
# 错误，单引号不能在单引号中间。有 \ 也不行
'''
'\''
```

## ""

$, `, \ 还是保留原来的意思。

*, @ 在双引号中间有特殊的含义。

# 管道( | 和 |& )

cmd1 | cmd2 : cmd1 的标准输出，作为 cmd2 的标准输入

cmd1 |& cmd2 : cmd1 的标准输出加上错误输出，作为 cmd2 的标准输入

time cmd1 : 统计命令运行时间

# 命令列表

## &

cmd& : 命令在后台运行

## ;

cmd1; cmd2; cmd3; : 命令依次运行，结果为最后一个命令的返回结果。

## || &&

## 连续的内存

### 整形

```bash
# 数学运算
A=$[ 3 + 2 ]
```

#### 字符类型( char )

### 浮点形

#### 32位

#### 64位

### 数组

### 字符串

```bash
cat << xxx
a
b
c
xxx
```

## 内存数据的读写

### 直接读写

### 只读

### shadwing

### 间接读写

#### 引用

##### 连续空间部分内容的引用( Slice )

###### String Slice

###### Other Slice

## 类型转换

# 空值

# 空对象

# 扩展

# 程序指令的执行

## 函数

```bash
square() {
    echo "$1 * $1" | bc
}

square 2
```

## 分支

### if

```bash
read v

# v 是一个可读的文件名
if [ -r $v ]
then
else
fi

#####################
if ...
then
elif ...
then
else
fi

#数字判断
if (( 3 > 2 ))
then
fi
```

## case

```bash
case $v in
1) abc ;;
2) bbb ;;
q|Q|e|E) ccc ;;
*) ddd ;;
esac
```

## 循环

### until

until test-commands; do consequent-commands; done

运行命令直到测试条件返加值为非0

### for

for name; do commands; done

for name in words …; do commands; done

for (( expr1 ; expr2 ; expr3 )) ; do commands ; done

```bash
for v in a b c
do
    wc $v
done

##########
for (( N=1001; N<=1101; N++ ))
do
    echo $N
done

##########
read v
for d in $v
do
    wc $d
done

##########
# $* 表示收到的所有参数
for d in $*
do
    more $d
done
```

### while

while test-commands; do consequent-commands; done

运行命令直到测试条件返加值为0

```bash
v=no

while [ $v = no ]
do
    read v
done

################
while :
do
    echo "无限循环"
done
```

# 范围

# 常量

## 字符和字符串常量

## 数字字面常量

# 错误处理

# 命令行

## 命令行参数

```bash
# 查看 -d -c -l -w ，option为变量名
# c: 表示 -c 后要有数据参数，此参数保存在 $OPTARG 中
while getopts dc:lw option
do
    echo $option
    if [ $option = c ]
    then
        echo $OPTARG
    fi
done
```

## 用户输入

# 文件处理

```bash
read v
if [ -f $v] # -f 如果文件存在
if [ -r $v] # -r 如果文件可读
if [ -d $v] # -d 如果目录存在
```

## 打开文件

## 新建文件

## 读写

# 环境参数

# 进程

# 时间

# echo

```bash
# 输出不换行
echo -n "aa"
```
