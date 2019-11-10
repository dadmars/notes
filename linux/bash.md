# 配置

```bash
# shell 种类
cat /etc/shells

# 用户的默认 shell
cat /etc/passwd
```

## 登录 bash 的配置文件

* /etc/profile : 应用于所有的用户环境的设置
* ~/.bash_profile, ~/.bash_login or ~/.profile : 优先找到的进行执行
* ~/.bash_logout : upon logout.

## no-login bash 的配置文件

* ~/.bashrc

# Hello, world

```bash
#!/bin/bash

echo "hello word"
```

## debug

```bash
bash -x test.sh
```

# 注释

```bash
# comment
```

# 运行

```bash
# cmd1 的标准输出，作为 cmd2 的标准输入
cmd1 | cmd2

# cmd1 的标准输出加上错误输出，作为 cmd2 的标准输入
cmd1 |& cmd2

# 统计命令运行时间
time cmd1

# 命令在后台运行
cmd&

# 命令依次运行，结果为最后一个命令的返回结果
cmd1; cmd2; cmd3;
```

## 别名

```bash
alias .. cd ..
alias ... cd ../..

unalias ..
unalias ...
```

# 变量

```bash
# 引用方式
${VAR}

# 简写
$VAR

# 环境变量（全局变量）
env

# 局部变量
set
set -o

# 设置选项
set -o noclobber
# 取消选项
set +o noclobber

# 只能用在本 shell ，不能用在子 shell 中
VARNAME="value"
# 可以用在子 shell 中
export VARNAME="value"
```

## 字符串查找

```bash
# 变量长度
${#VAR}

A="abc"
${#A} # 3

# 如果 A 没有定义或为空，值为 WORD
${A:-WORD}

# 如果 A 没有定义或为空，设置 A 的值为 WORD
${A:=WORD}

# 截取变量 A 的子串，LENGTH 没有，截取后面全部
${A:OFFSET:LENGTH}

# 删除 WORD, # 最短匹配， ## 最长匹配， 如果是数组，对数组中的每项进行匹配
${A#WORD}
${A##WORD}

${A[@]#w}

# 反向删除 WORD, % 最短匹配， %% 最长匹配， 如果是数组，对数组中的每项进行匹配
${A%WORD}
${A%%WORD}

# 替换, / 替换第一个, // 替换所有
${VAR/PATTERN/STRING}
${VAR//PATTERN/STRING}
```

## 数组

```bash
a[1]=1
declare -a a
a=(1 2 3)

echo ${a[@]}
echo ${a[1]}

unset a[1]
unset a
```

## 字符串

```bash
# 转义
\

# ''
# 下面两条错误，单引号不能在单引号中间。有 \ 也不行
'''
'\''

# ""
# $, `, \ 还是保留原来的意思。
# *, @ 在双引号中间有特殊的含义。
```

## 预定义变量

```bash
# 当前用户的 USER ID
# root 用户的 USER ID 为 0
$UID
```

# 扩展 {}

注意 ${} 表示引用变量， 不能搞混

```bash
echo sp{el, al, ab}l  # spell spall spabl
```

# 运行子命令

```bash
$()
```

# 数学运算

$[3 + 2]
$(()) 也可以计算

# 函数

```bash
square() {
    echo "$1 * $1" | bc
}

square 2
```

## 函数参数

```bash
$@ 所有参数, "$*" 每个参数都为一个字符串, 结果为一数组
$# 参数个数
$? 上一命令的返回值
$$ 本 shell 的进程ID: PID
$! 最近的在后台运行的进程ID
$0 本脚本或函数的名称
$_ 在刚启动脚本时调用，返回本脚本的路径名。在一个命令之后调用，返回上一个命令的最后一个参数
```

# 分支

## 条件判断

* [[]]: 不对文件名进行扩展， a*，不会解释为以 a 开头的所有文件。
* []
* test

```bash
[ -z STRING ] # 字符串长度为0
[ STRING ] # 字符串长度不为0 [ -n STRING ]
[ STRING1 == STRING2 ] # 字符串相等
[ STRING1 != STRING2 ] # 字符串相等
[ STRING1 < STRING2 ] # 字符串相等
[ STRING1 > STRING2 ] # 字符串相等

# 整型数字相比:
[ INT_NUM1 OP INT_NUM2 ]
 -eq
 -ne
 -lt
 -le
 -gt
 -ge

[ !EXPR ]
[ EXPR1 -a EXPR2 ] # and
[ EXPR1 -o EXPR2 ] # or

# 检测环境变量
[ -o noclobber ]
```

## if

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

# 循环

## until

运行命令直到测试条件返加值为非0

```bash
until test-commands; do consequent-commands; done
```

## for

```bash
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

## while

运行命令直到测试条件返加值为0

```bash
while test-commands; do consequent-commands; done

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

# 命令行参数

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

## read 用户输入

```bash
# 输入 n 个字符就返回，而不是等回车
read -n v
```

# 文件处理

```bash
read v
if [ -a $v] # 如果文件存在
if [ -e $v] # 如果文件存在
if [ -f $v] # 如果文件存在，并是一个普通文件
if [ -s $v] # 如果文件存在，大小不为 0
if [ -d $v] # 如果目录存在
if [ -h $v] # 如果文件存在，并是一个 symbolic link
if [ -r $v] # 如果文件可读
if [ -w $v] # 如果文件可写
if [ -x $v] # 如果文件可执行
```

## 文件 symbolic link

* /dev/fd/N 文件描述 N
* /dev/stdin 文件描述 0
* /dev/stdout 文件描述 1
* /dev/stderr 文件描述 2

## 文件重定向

* 上个命令的输出重定向到当前命令的输入， /proc/curr_process_id/fd/0 与 /proc/prev_process_id/fd/1 相同
* N>&M 和 N<&M，表示 /proc/self/fd/N 与 /proc/self/fd/M 相同
* N>file 和 N<file，表示 symbolic link /proc/self/fd/N 与 file 的文件描述符相同
* N>&- 表示删除 symbolic link /proc/self/fd/N
* &>FILE 等于 >FILE 2>&1

## /dev/null 文件系统

```bash
# 清空文件
cat /dev/null > file
```

## /proc 文件系统

在内存中的虚拟文件系统，提供接口指向内核的数据结构。

* /proc/PID 子目录，每个正在运行的进程，PID 为进程ID
* /proc/self 子目录，当前访问 /proc 的进程，指向当前进程的 /proc/PID

## here 文档

```bash
cat << xxx
a
b
c
xxx

# - 忽略行前面的 tab ，不包含空格
cat << -xxx
    a
    b
    c
xxx
```

## 目录

```bash
pushd
popd
dirs
```

# echo

```bash
# 输出不换行
echo -n "aa"
# 处理转义字符
echo -e "a\ta"
```

# sed 行编辑器

```bash
命令

a\  当前行下面添加文字
c\  当前行对文字进行替换
i\  当前行上面添加文字
d   删除文字
p   打印文字
r   读文件
w   写文件
s   查找并替换文字

选项

-n  不输出多余信息
-e  执行多个命令

set -n '/errors/p' example # 找到所有包含 errors 的行并打印
set -n '/errors/d' example # 删除所有包含 errors 的行

set -n '2,4d' example # 删除 2-4 行
set -n '2,$d' example # 删除 2 行到文件结尾的行
set -n '/abc/,/aaa/d' example # 删除第一个包含abc的行，到第一个包含aaa的行

set -n 's/errors/aaa/' example # 找到第一个包含erros的行，将其替换为aaa
set -n 's/errors/aaa/g' example # 找到所有包含erros的行，将其替换为aaa
set -n 's/^/> /g' example # 所有行的前面插入 "> "
set -n 's/$/EOL/g' example # 所有行的结尾插入 EOL

set -e 's/^/> /g' -e 's/$/EOL/g' example # 执行多个命令
```

# awk 表格编辑器

数据驱动，描述你要操作的数据，然后对找到的数据进行操作

```bash
awk COMMAND inputfiles
awk -f COMMAND-FILE inputfiles

将读入的内容分为列，$0 代表整行，各列分别为 $1, $2 $3 ...

ls -l | awk '{print $4 $5}'
ls -l | awk '{print "aaa"\t: $4 "bbb" $5}'
ls -l | awk '{print "aaa"\t: NR $4 "bbb" $5}'   # NR 代表行号

df -h | awk '/dev\/hd/ {print $6 "/t: " $5}'    # 找到包含 dev/hd 的行并输出

# 前面和后面输出
df -h | awk 'BEGIN {print "begin :\n"} /dev\/hd/ {print $6 "/t: " $5} END {print "end"}'

awk 'BEGIN { FS=“:” } {print $1}' # 输入以 : 进行分割
awk 'BEGIN { FS=“:” ; ORS="\n===>\n" } {print $1}' # 设置输入和输出的分割符

# 设置变量
awk '{ total=total + $5 } { print total }' xx
```

# 执行远程命令

```bash
ssh -t zs@40.90.101.100 "cd tmp"

# 使用 sudo 并输入密码
ssh -t zs@40.90.101.100 "cd tmp && echo 789 | sudo -S ./update.sh"
```

# sudo

```bash
# -S : 密码从 stdin 读入
echo 999 | sudo -S ./up.sh
```
