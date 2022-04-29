# bash

<!-- TOC -->

- [bash](#bash)
    - [shell 种类](#shell-%E7%A7%8D%E7%B1%BB)
        - [log shell登录系统后运行的 shell 启动时读取的配置文件](#log-shell%E7%99%BB%E5%BD%95%E7%B3%BB%E7%BB%9F%E5%90%8E%E8%BF%90%E8%A1%8C%E7%9A%84-shell-%E5%90%AF%E5%8A%A8%E6%97%B6%E8%AF%BB%E5%8F%96%E7%9A%84%E9%85%8D%E7%BD%AE%E6%96%87%E4%BB%B6)
        - [non_log shell启动一个 shell终端 启动时读取的配置文件](#non_log-shell%E5%90%AF%E5%8A%A8%E4%B8%80%E4%B8%AA-shell%E7%BB%88%E7%AB%AF-%E5%90%AF%E5%8A%A8%E6%97%B6%E8%AF%BB%E5%8F%96%E7%9A%84%E9%85%8D%E7%BD%AE%E6%96%87%E4%BB%B6)
    - [设置时间](#%E8%AE%BE%E7%BD%AE%E6%97%B6%E9%97%B4)
    - [磁盘](#%E7%A3%81%E7%9B%98)
    - [内存](#%E5%86%85%E5%AD%98)
    - [文件系统](#%E6%96%87%E4%BB%B6%E7%B3%BB%E7%BB%9F)
        - [通配符](#%E9%80%9A%E9%85%8D%E7%AC%A6)
    - [shell 种类](#shell-%E7%A7%8D%E7%B1%BB)
    - [启动 bash 时读取的配置文件](#%E5%90%AF%E5%8A%A8-bash-%E6%97%B6%E8%AF%BB%E5%8F%96%E7%9A%84%E9%85%8D%E7%BD%AE%E6%96%87%E4%BB%B6)
        - [login shell](#login-shell)
        - [no-login shell](#no-login-shell)
    - [sha-bang  #!](#sha-bang--)
    - [debug](#debug)
    - [运行](#%E8%BF%90%E8%A1%8C)
    - [别名](#%E5%88%AB%E5%90%8D)
    - [变量](#%E5%8F%98%E9%87%8F)
        - [位置变量](#%E4%BD%8D%E7%BD%AE%E5%8F%98%E9%87%8F)
        - [全局变量](#%E5%85%A8%E5%B1%80%E5%8F%98%E9%87%8F)
        - [本地变量， 只针对当前 shell](#%E6%9C%AC%E5%9C%B0%E5%8F%98%E9%87%8F-%E5%8F%AA%E9%92%88%E5%AF%B9%E5%BD%93%E5%89%8D-shell)
    - [特殊参数](#%E7%89%B9%E6%AE%8A%E5%8F%82%E6%95%B0)
    - [数组](#%E6%95%B0%E7%BB%84)
    - [转义](#%E8%BD%AC%E4%B9%89)
    - [扩展](#%E6%89%A9%E5%B1%95)
        - [{}](#)
        - [~](#)
        - [参数，命令，数学计算 $](#%E5%8F%82%E6%95%B0%E5%91%BD%E4%BB%A4%E6%95%B0%E5%AD%A6%E8%AE%A1%E7%AE%97-)
    - [字符串查找](#%E5%AD%97%E7%AC%A6%E4%B8%B2%E6%9F%A5%E6%89%BE)
        - [expr](#expr)
    - [预定义变量](#%E9%A2%84%E5%AE%9A%E4%B9%89%E5%8F%98%E9%87%8F)
    - [函数](#%E5%87%BD%E6%95%B0)
    - [条件判断](#%E6%9D%A1%E4%BB%B6%E5%88%A4%E6%96%AD)
    - [if](#if)
    - [case](#case)
    - [until](#until)
    - [for](#for)
    - [while](#while)
    - [命令行参数](#%E5%91%BD%E4%BB%A4%E8%A1%8C%E5%8F%82%E6%95%B0)
    - [read 用户输入](#read-%E7%94%A8%E6%88%B7%E8%BE%93%E5%85%A5)
    - [文件处理](#%E6%96%87%E4%BB%B6%E5%A4%84%E7%90%86)
    - [文件 symbolic link](#%E6%96%87%E4%BB%B6-symbolic-link)
    - [文件重定向](#%E6%96%87%E4%BB%B6%E9%87%8D%E5%AE%9A%E5%90%91)
    - [/dev/null 文件系统](#devnull-%E6%96%87%E4%BB%B6%E7%B3%BB%E7%BB%9F)
    - [/proc 文件系统](#proc-%E6%96%87%E4%BB%B6%E7%B3%BB%E7%BB%9F)
    - [here 文档](#here-%E6%96%87%E6%A1%A3)
    - [目录](#%E7%9B%AE%E5%BD%95)
    - [echo](#echo)
    - [sed 行编辑器](#sed-%E8%A1%8C%E7%BC%96%E8%BE%91%E5%99%A8)
    - [awk 表格编辑器](#awk-%E8%A1%A8%E6%A0%BC%E7%BC%96%E8%BE%91%E5%99%A8)
    - [sudo](#sudo)
    - [ssh](#ssh)
    - [计算秒级时间差](#%E8%AE%A1%E7%AE%97%E7%A7%92%E7%BA%A7%E6%97%B6%E9%97%B4%E5%B7%AE)
    - [grep](#grep)
    - [color](#color)

<!-- /TOC -->

## shell 种类

```bash
cat /etc/shells
```

### log shell(登录系统后运行的 shell) 启动时读取的配置文件

```bash
/etc/profile
    应用于所有用户环境的设置都放在此文件中。被所有类型的 shell 读取

~/.bash_profile
~/.bash_login
    login 到系统时读取
~/.profile
    上面三个文件哪个第一个找到，读取哪个

~/.bash_logout logout 时读取
```

### non_log shell(启动一个 shell终端) 启动时读取的配置文件

```bash
~/.bashrc
```

## 设置时间

```bash
date
cal    # 日历
```

```bash
timedatectl

输出:
    Local time: 三 2020-10-28 09:40:44 CST    # 本地当前时间
    Time zone: Asia/Shanghai (CST, +0800)     # 时区

timedatectl list-timezones    # 列出所有时区

timedatectl set-timezone Asia/Shanghai  # 设置时区

# 设置日期时间
    date -s '2019-12-12 16:18:45'
# 或
    timedatectl set-ntp 0 # 关闭自动同步时间
    timedatectl set-time '2019-12-12 16:18:45'  # 设置日期和时间
    timedatectl set-time 2019-12-12  # 设置日期
    timedatectl set-time 16:18:45  # 设置时间
```

## 磁盘

```bash
df    # 磁盘空间
```

## 内存

```bash
free    # 内存空间
```

## 文件系统

```bash
pwd    # 当前路径

ls     # 列出当前目录内容
ls aa bb # 列出aa bb目录的内容
    -a    # 同时列出隐藏文件
    -h    # 文件大小更易读
    -F    # 后面显示辅助字符
    -l    # 文件详细信息
    -i    # 显示 inode
    -ld    # 显示目录本身信息
    -lt    # 修改时间排序(降序)
    -ltr   # 修改时间排序(升序)
    -lS    # 文件大小排序(降序)
    -lSr    # 文件大小排序(升序)

cd     # 改变当前目录
    cd    # home 目录
    cd -  # 上一个目录
    cd ~username  # username 的home目录

file   # 文件类型

mkdir a b c   # 创建目录

cp a b        # 拷贝 a 到 b
cp a b c      # 拷贝 a, b 到 c
    -a   # 拷贝所有文件和目录, 并保留它们所有的属性
    -i   # 覆盖文件时提醒
    -r   # 拷贝目录及子目录
    -u   # 从一个目录向另一个目录拷贝文件时，只拷贝目标目录中不存在和比目标目录中的文件要新的文件
    -v   # 显示过程信息

mv a b        # 移动或重命名
    -i   # 覆盖文件时提醒
    -u   # 从一个目录向另一个目录移动文件时，只拷贝目标目录中不存在和比目标目录中的文件要新的文件
    -v   # 显示过程信息

rm b        # 删除
    -i   # 删除文件时提醒
    -r   # 删除目录及子目录
    -f   # 强制删除
    -v   # 显示过程信息
    tip:
       为防止误删， 先用 ls, 然后将 ls 替换为rm
       rm *.html
       rm * .html  # * 后多了一个空格，悲剧了

ln -s a link    # 符号链接
ln a link    # 硬链接

type command  # 命令的类型

which cp  # 可执行程序的路径

man cp # 帮助手册
```

### 通配符

```bash
*               # 任何字符 
?               # 任何单个字符 
[characters]    # 集合中的任何字符
[!characters]   # 不属于集合的任何字符
[[:class:]]     # 集合的字符符合指定 class
   [:alnum:]    # 字母和数字
   [:alpha:]    # 字母
   [:digit:]    # 数字
   [:lower:]    # 小写字母
   [:upper:]    # 大写字母

*         # 所有文件
g*        # 以g开头的所有文件
b*.txt    # 以b开头, 最后为 .txt 的所有文件
Data???   # 以Data开头，后面跟3个字符的所有文件
[abc]*    # 以a或b或c开头的所有文件
BACKUP.[0-9][0-9][0-9]  # 以BACKUP.开头，所面跟3个数字的所有文件
[[:upper:]]*            # 以大写字母开头的所有文件
[![:digit:]]*           # 不以数字开头的所有文件
*[[:lower:]123]         # 以小定字母或1或2或3结尾的所有文件
```

## shell 种类

```bash
cat /etc/shells   # 系统支持的 shell 类型
cat /etc/passwd   # 用户的默认 shell
```

## 启动 bash 时读取的配置文件

### login shell

* /etc/profile : 应用于所有的用户环境的设置
  * /etc/bashrc : 由于 /etc/proflile 被所有类型的 shell 读取，可以将 bash 的配置放在此文件
* ~/.bash_profile, ~/.bash_login or ~/.profile : 优先找到的进行执行
* ~/.bash_logout : logout

### no-login shell

~/.bashrc

## sha-bang ( #!)

脚本的第一行说明此脚本运行哪个shell

```bash
#!/bin/bash
```

第一行也可以不是 shell

```bash
#!/bin/sh
#!/bin/bash
#!/usr/bin/perl
#!/usr/bin/tcl
#!/bin/sed -f
#!/usr/awk -f
```

## debug

```bash
bash -x test.sh
```

## 运行

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

## 变量

```bash
# 全局变量(环境变量）
env

# 局部变量
set

# 引用方式
${VAR}

# 简写
$VAR

# 设置选项
set -o noclobber
# 取消选项
set +o noclobber

# 只能用在本 shell ，不能用在子 shell 中
VARNAME="value"

# 可以用在子 shell 中
export VARNAME="value"
```

### 位置变量

$0, $1, $2, $3 . . . ${10}, ${11}, ${12}.

$@ 输出所有参数

$# 参数个数

shift  $1 <--- $2, $2 <--- $3, $3 <--- $4

```bash
#!/bin/bash

if [ -n "$1" ] # 参数不为空
then
    echo "Parameter #1 is $1"
fi

if [ -z $1 ] # 参数为空
then
    exit $E_MISSING_POS_PARAM
fi

case `basename $0` in
"wh"       ) whois $1@whois.tucows.com;;
"wh-ripe"  ) whois $1@whois.ripe.net;;
"wh-apnic" ) whois $1@whois.apnic.net;;
"wh-cw"    ) whois $1@whois.cw.net;;
*          ) echo "Usage: `basename $0` [domain-name]";;
esac

exit $?

until [ -z "$1" ]
do
    echo -n "$1 "
    shift  # $1 <--- $2, $2 <--- $3, $3 <--- $4
done

shift 3 # Shift 3 positions.
```

### 全局变量

```bash
env
```

### 本地变量， 只针对当前 shell

```bash
# 查看本地变量
set

# shll 选项
set -o
set -o noclobber   # 设置选项
set +o noclobber   # 取消设置

# 设置本地变量, 子shell无法使用
W="a"
echo $W

# 删除本地变量
unset W

# 设置本地变量, 子shell也可以使用
# 在子shell调用 unset W, 不会影响其它子shell和父shell
export W="a
```

## 特殊参数

```bash
$@  # 所有的参数, $* 有同样含义但有问题不推荐使用
$#  # 参数个数
$?  # 最近命令的返回值
$-  # 当前选项( 通过 set 设置)
$$  # 当前进程 ID
$!  # 最近命令的进程 ID
$0  # 脚本的文件名
$_  # 最近命令的最后一个参数，如果没有参数，则是命令本身

;  # 同一行运行多个命令
    echo hello; echo there

    if [ -x "$filename" ]; then
        echo "File $filename exists."; cp $filename $filename.bak
    else
        echo "File $filename not found."; touch $filename
    fi

: # null command
    :
    echo $?  # 0

    while :
    do
        operation-1
        ...
        operation-n
    done

    if condition
    then :
        # Do nothing and branch ahead
    else
    fi

    : > data.xxx # 同 cat /dev/null >data.xxx
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

## 转义

```bash
# 转义
\

''
# 单引号内的字符没有任何特殊含义
# 下面两条错误，单引号不能在单引号中间。有 \ 也不行
'''
'\''

""
# $, `, \ 还是保留原来的意思。
# *, @ 在双引号中间有特殊的含义。

List="one two three"

for a in $List
do
    echo $a   # one
              # two
              # three
done

echo "---"
for a in "$List"
do
    echo $a  # one two three
done
```

## 扩展

### {}

注意 ${} 表示引用变量， 不能搞混

```bash
echo sp{el, al, ab}l  # spell spall spabl
```

### ~

```bash
~    # 扩展为 $HOME
~+   # 扩展为 $PWD
~-   # 扩展为 $OLDPWD
~a   # 不扩展
```

### 参数，命令，数学计算 $

```bash
# 参数
${PARAMETER}
${!PARAMETER}   # 前面有 ! 间接扩展

args=$#             # Number of args passed.
lastarg=${!args}    # Note: This is an *indirect reference* to $args ...
# Or:
lastarg=${!#}       # This is an *indirect reference* to the $# variable. Note that lastarg=${!$#} doesn't work.

${VAR:=value}   # 如果 VAR 没有定义，为其赋值

# 命令
$(command)
`command`

# 数学计算
$((command))
$[command]
```

## 字符串查找

```bash
# 长度
${#VAR}

A="abc"
${#A} # 3

# 如果 A 没有定义或为空，值为 WORD
${A:-WORD}

# 如果 A 没有定义或为空，设置 A 的值为 WORD
${A:=WORD}

# 截取变量 A 的子串，LENGTH 没有，截取后面全部,从左边计数，从０开始
${A:OFFSET:LENGTH}
# 截取变量 A 的子串，从右边计数，从1开始
${A:-OFFSET:LENGTH}
${*:OFFSET:LENGTH}  # 得到第几个参数

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

${string/#substring/replacement}  # If substring matches front end of string, substitute replacement for substring
${string/%substring/replacement}  # If substring matches back end of string, substitute replacement for substring
```

### expr

```bash
# 匹配开头字符串， substring 为正则表达式, 结果返回最后的位置
expr match "$string" '$substring'

stringZ=abcABC123ABCabc
        |------|
      # 12345678
echo `expr match "$stringZ" 'abc[A-Z]*.2'   # 8

# 匹配开头字符串， substring 为正则表达式, 结果为开始的位置
expr index "$string" '$substring'

stringZ=abcABC123ABCabc
      # 123456 ...
echo `expr index "$stringZ" C12`    # 6
```

## 预定义变量

```bash
# 当前用户的 USER ID, root 用户的 USER ID 为 0
$UID
```

## 函数

```bash
square() {
    echo "$1 * $1" | bc
}

square 2

# 读入文件的每行
file_excerpt () {
  while read line
  do
    echo "$line" | grep $1 | awk -F":" '{ print $5 }'
  done
} <$file    # 重定向函数输入为 stdin
```

## 条件判断

```bash
[]
[[]]  # 不对文件名进行扩展， a*，不会解释为以 a 开头的所有文件。
(())  # 数学判断

[ -z STRING ]           # 字符串长度为0
[ STRING ]              # 字符串长度不为0 [ -n STRING ]
[ STRING1 == STRING2 ]  # 字符串相等
[ STRING1 != STRING2 ]
[ STRING1 < STRING2 ]
[ STRING1 > STRING2 ] 

# 整型数字相比:
[ INT_NUM1 OP INT_NUM2 ]
 -eq
 -ne
 -lt
 -le
 -gt
 -ge

[ !EXPR ]               not
[ EXPR1 -a EXPR2 ]      and
[ EXPR1 -o EXPR2 ]      or

[ -o noclobber ]        检测环境变量

read v
if [ -a $v] # 如果文件存在
if [ -b $v] # 如果文件存在, 是一个block-special文件
if [ -c $v] # 如果文件存在, 是一个character-special文件
if [ -d $v] # 如果目录存在
if [ -e $v] # 如果文件存在
if [ -f $v] # 如果文件存在，并是一个普通文件
if [ -g $v] # 如果文件存在，SGID 标志位被设置
if [ -h $v] # 如果文件存在，并是一个 symbolic link
if [ -k $v] # 如果文件存在，sticky 标志位被设置
if [ -p $v] # 如果文件存在，是一个 pipe
if [ -r $v] # 如果文件可读
if [ -u $v] # 如果文件存在，SUID 标志位被设置
if [ -w $v] # 如果文件可写
if [ -x $v] # 如果文件可执行
if [ -s $v] # 如果文件存在，大小不为 0
if [ -O $v] # 如果文件存在，owned by the effective user ID
if [ -G $v] # 如果文件存在，owned by the effective group ID
if [ -N $v] # 如果文件存在，在最后一次读取后，被修改过
if [ -S $v] # 如果文件存在，是一个 socket
if [ $v1 -nt $v2] # 如果文件v1修改时间比v2大，或者v1存在，v2不存在
if [ $v1 -ot $v2] # 如果文件v1修改时间比v2小，或者v1不存在，v2存在
if [ $v1 -ef $v2] # 如果文件v1和v2指向同一个 device and inode numbers
```

## if

```bash
read v

# v 是一个可读的文件名
if [ -r $v ]
then
elif []
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

## until

运行命令直到测试条件返加值为非0

```bash
until test-commands
do
    consequent-commands
done
```

## for

```bash
for name
do
    commands
done

for name in words …
do
    commands
done

for (( expr1 ; expr2 ; expr3 ))
do
    commands
done

for v in a b c
do
    wc $v
done

for (( N=1001; N<=1101; N++ ))
do
    echo $N
done

read v
for d in $v
do
    wc $d
done

# $* 表示收到的所有参数
for d in $*
do
    more $d
done
```

## while

运行命令直到测试条件返加值为0

```bash
while test-commands
do
    consequent-commands
done

v=no
while [ $v = no ]
do
    read v
done

while :
do
    echo "无限循环"
done
```

## 命令行参数

* -d -c -l -w
* option    变量名
* c:        -c 后要有数据参数，此参数保存在 $OPTARG 中
* $OPTIND   下一个参数的索引

```bash
while getopts dc:lw option
do
    echo $option
    if [ $option = c ]
    then
        echo $OPTARG
    fi
done

shift $[$OPTIND - 1]
echo $1
```

## read 用户输入

```bash
read -n v           输入 n 个字符就返回，而不是等回车
```

## 文件处理

## 文件 symbolic link

* /dev/fd/N 文件描述 N
* /dev/stdin 文件描述 0
* /dev/stdout 文件描述 1
* /dev/stderr 文件描述 2

## 文件重定向

每一个打开的文件都有一个描述符。stdin为０，stdout为1，stderr为2。

> < 后面如里是描述符，前面加 &

* 上个命令的输出重定向到当前命令的输入， /proc/curr_process_id/fd/0 与 /proc/prev_process_id/fd/1 相同
* N>&M 和 N<&M，表示 /proc/self/fd/N 与 /proc/self/fd/M 相同
* N>file 和 N<file，表示 symbolic link /proc/self/fd/N 与 file 的文件描述符相同
* N>&- 表示删除 symbolic link /proc/self/fd/N
* &>FILE 等于 >FILE 2>&1

```bash
: > file          文件变为空
1>file            stdout输出到文件
2>file            stderr输出到文件
&>file            stdout和stderr输出到文件
fid>file          文件描述符输出到文件
fid>&fid-n        文件描述符输出到另一个文件描述符
>&fid             stdout(默认)输出到文件描述符
0< file           从文件接收输入
exec 3<> file     打开file读写，描述符为３
exec 3>&-         关闭描述符３
0<&-              关闭stdin
<&-               关闭stdin
fid<&-            关闭输入描述符
fid>&-            关闭输出描述符
```

## /dev/null 文件系统

```bash
# 清空文件
cat /dev/null > file
```

## /proc 文件系统

在内存中的虚拟文件系统，提供接口指向内核的数据结构。

* /proc/PID     子目录，每个正在运行的进程，PID 为进程ID
* /proc/self    子目录，当前访问 /proc 的进程，指向当前进程的 /proc/PID

## here 文档

```bash
cat << xxx
a
b
c
xxx

# - 忽略行前面的 tab ，不包含空格
cat <<-xxx
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

## echo

```bash
echo -n "aa"        输出不换行
echo -e "a\ta"      处理转义字符
```

## sed 行编辑器

```bash
a   # 当前行下面添加文字
c   # 当前行对文字进行替换
i   # 当前行上面添加文字
d   # 删除文字
p   # 打印文字
r   # 读文件
w   # 写文件
s   # 查找并替换文字

-n  # 不输出多余信息
-e  # 执行多个命令
-f  # 命令文件
```

```bash
sed -n '/aa/=' file      =表示只输出行号
sed -n '3,5 !p' file    !p表示不匹配的行。

sed -n '/errors/p' example      找到所有包含 errors 的行并打印

sed '2 d' file                  删除第2行
sed -n '2,4d' example           删除 2-4 行
sed -n '2,$d' example           删除 2 行到文件结尾的行
sed -n '/errors/d' example      删除所有包含 errors 的行
sed -n '/abc/,/aaa/d' example   删除第一个包含abc的行，到第一个包含aaa的行
sed '3,/bbb/ d' file            删除第 3 行到第一个包含bbb的行
sed '/^$/d' file                删除空行

sed -n 's/errors/aaa/' example      找到第一个包含erros的行，将其替换为aaa
sed -n 's/errors/aaa/g' example     找到所有包含erros的行，将其替换为aaa
sed -n 's/^/> /g' example           所有行的前面插入 "> "
sed -n 's/$/EOL/g' example          所有行的结尾插入 EOL
sed '/cheese/ s/a/AAA/g' file       找到包含cheese的行，在行中进行替换
sed '7,/fish/ s/a/AAA/g' file       从第7行到包含fish的行,在行中进行替换

sed -e 's/^/> /g' -e 's/$/EOL/g' example    执行多个命令

sed '7 q' file          读到第7行时退出。
sed '/aa/q' file        读到匹配的行时退出。
sed '/[0-9]/q' file

sed '/fish/ r aaa.txt' file     将aaa.txt的所有内容加到包含fish的行后
sed '/ab/,/zz/ w a.txt' file    将范围之内的行写出到文件

sed -f mod.rec file     可以将参数放在mod.rec文件中
```

## awk 表格编辑器

数据驱动，描述你要操作的数据，然后对找到的数据进行操作

```bash
awk COMMAND inputfiles
awk -f COMMAND-FILE inputfiles

# 将读入的内容分为列

$0            # 整行
$1, $2 $3 ... # 第 1 列， 第 2 列， ...
NR            # 行号
NR            # 行号
BEGIN         # 前面输出
END           # 后面输出

ls -l | awk '{print $4 $5}'
ls -l | awk '{print "aaa"\t: $4 "bbb" $5}'
ls -l | awk '{print "aaa"\t: NR $4 "bbb" $5}'   NR 代表行号

df -h | awk '/dev\/hd/ {print $6 "/t: " $5}'    找到包含 dev/hd 的行并输出
awk '/fish/ {print}' file       选择包含fish的行，并打印
awk '/fish/' file               不指定动作，默认为打印
awk '{print}' file              不指定模式，默认为匹配全部

df -h | awk 'BEGIN {print "begin :\n"} /dev\/hd/ {print $6 "/t: " $5} END {print "end"}'

awk 'BEGIN { FS=“:” } {print $1}'                       输入以 : 进行分割
awk 'BEGIN { FS=“:” ; ORS="\n===>\n" } {print $1}'      设置输入和输出的分割符

awk '{ total=total + $5 } { print total }' xx       设置变量
awk -v item='hello aaa' '{print item,$1}' file      -v指示后面会传一个变量
awk '{print item,$1}' file                          当无法识别变量时，被忽略
awk '{print "item:", $1,"Price:",$3}' file          在引号内的字符串直接输出
awk '{print 123,$1}' file                           数字不被当作变量
```

## sudo

* -S    密码从 stdin 读入

```bash
echo 999 | sudo -S ./up.sh
```

## ssh

```bash
ssh -t zs@40.90.101.100 "cd tmp"    执行远程命令
ssh -t zs@40.90.101.100 "cd tmp && echo 789 | sudo -S ./update.sh"  使用 sudo 并输入密码
```

## 计算秒级时间差

```bash
s=$(date +"%s")
e=$(date +"%s")

diff=$(( e -s ))
echo $diff
```

## grep

```bash
-n    # 输出行号
-r    # 包含子目录。
-l    # 只列出文件名
-c    # 在文件中的匹配数
-x    # 整行匹配
-v    # 反意搜索
-i    # 不分大小写
-e    # 不把 “-” 解释为选项
```

```bash
grep -n -r a file1 file2 hhh*   # 在file1，file2，当前目录下所有以hhh开头的文件中查找a
grep -e -r file1                # 在file1中查找 "-r"，-e可以不把“-”解释为选项

grep -n '^a' file      # 匹配行的开头
grep -n '^^' file

grep -n 'a$' file      # 匹配行的结尾
grep -n '^$' file

grep 'a[b7]' file       # 一个字符 ，范围在[]中
grep '[sdfsf]' file
grep '[ad8]d[sf]' file
grep '[09]b' file
grep '[0-9]b' file
grep '[a-z]' file
grep '[25a-z]' file
grep '[a0-5m-z]' file
grep '[A-Z0-9a-z]' file

grep '[^ab]' file       # 选择一个字符，不是a或b
grep '[a^b$]' file      # 当 ^ 不是在 [] 中的第一个时，当作普通字符，所有元字符在[]中都当作普通字符。

grep -n '.' file        # 任意一个字符
grep -v -n '.' file
grep '^.b' file
grep '^...$' file

grep '\<the\>' file      # 完整匹配 
grep '\<the' file

grep 'ab*' file         # * 表示前面字符中的0个或多个实例，a, ab, abb, abb....
grep '.*' file          # .*可匹配任意字符

grep -w / /etc/fstab    # -w 匹配 / ，此字符两边是空格
```

## color

Color|Code for making normal color|Code for making Bold color
---|---|---
Red | 0;31 | 1;31
Green | 0;32 | 1;32
Blue | 0;34 | 1;34
Black | 0;30 | 1;30
Yellow | 0;33 | 1;33

```bash
echo $'\e[1;33m'Welcome to linux hint$'\e[0m'

Red=$'\e[1;31m'
Green=$'\e[1;32m'
Blue=$'\e[1;34m'
Normal=$'\e[0m'

echo "$Blue I like chocolate cake "
```
