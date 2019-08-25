# Hello, world

```bash
#!/bin/bash

echo "hello word"
```

# 特性

# 生存期( Lifetimes )

## ownership

## Lifetimes

# 注释

```bash
# comment
```

# 对内存的表示

## 连续的内存

### 整形

#### 1个字节(8位)

#### 2个字节(16位)

#### 4个字节(32位)

#### 8个字节(64位)

#### 16个字节(128位)

#### 平台相关

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

### 向量

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

## destructuring

## 非连续的内存

### map

### 链表

### 树

# 空值

# 空对象

## unit type

# 结构化

## 元组( tuple )

## 结构( struct )

### Tuple Structs

### Unit-Like Structs

## 枚举( enum )

# 扩展

## Range

# 程序指令的执行

## 主函数（ 程序进入点 ）

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

### for

```bash
for v in a b c
do
    wc $v
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

# 数据封装

## struct

# 错误处理

# 代码组织( 模块化 )

## 工具

## workspace

## package

## key: mod, pub

## key: crate, self, super

## key: use, as

## External Packages

## Separating Modules into Different Files

# 自动化测试

## 单元测试

## 集成测试

# 修饰器编程( Decorator )

## Display

# 面向对象

## trait

## trait object

## 动态类型检测

# 函数式编程

# 泛型

## 迭代器( Iterators )

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
```

## 打开文件

## 新建文件

## 读写

# 环境参数

# 进程

# 时间

# 线程

## 创立线程

## 线程结束

## 线程间通迅

## 线程间同步

# 异步

# 网络

# 调用外部程序

# 宏

# Unsafe Rust
