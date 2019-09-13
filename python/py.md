# 安装

## Installation

## Hello, world

# 特性

# 生存期( Lifetimes )

# 注释

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

### 向量

```py
v = [1, 2, 3]

v[-2] # 2
len(v) # 3

v.append(5)
del v[0]

v = [1, 2, 3] + [4, 5] # [1, 2, 3, 4, 5]

for x in v:
    print(x)

if 3 in v:
    None
```

## 内存数据的读写

### 直接读写

### 只读

### shadwing

### 间接读写

#### 指针

##### 指针类型

##### 空指针

##### 得到指向的值

##### 指针运算

#### 智能指针

#### 引用

##### 连续空间部分内容的引用( Slice )

###### String Slice

###### Other Slice

```py
v = [1, 2, 3]
v[1:] # [2, 3]
```

## 类型转换

## destructuring

## 非连续的内存

### map

### 链表

### 树

# 空值

# 空对象

# 结构化

## 元组( tuple )

## 结构( struct )

### Tuple Structs

## 枚举( enum )

# 扩展

## Range

# 程序指令的执行

## 主函数（ 程序进入点 ）

## 函数

## 一般函数

## 闭包( closure )

## 分支

### if

## match

### if let

## 循环

### loop

### for

### while

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

## 用户输入

# 文件处理

## 列出目录中的内容

```py
import os
import re

rx = re.compile(r'\.(rs|md)')

for x in os.listdir('.'):
    if rx.search(x)]:
        None

    if x.endswith('.js'):
        None

    if os.path.isfile(x):
        None

    if os.path.isdir(x):
        None

    if os.path.islink(x):
        None

# 循环遍历
for x in os.walk('.'):
    print(x)

for path, dnames, fnames in os.walk('.'):
    None

# more
# finding files
rx = re.compile(r'\.(js|md)')
r = []
for path, dnames, fnames in os.walk('.'):
    r.extend([os.path.join(path, x) for x in fnames if rx.search(x)])
print r

# find files larger than 10000 bytes
r = []
for path, dnames, fnames in os.walk('.'):
    r.extend([os.path.join(path, x) for x in fnames if os.path.getsize(os.path.join(path, x)) > 10000])
print r

# files that have been modified less than 8 hours ago?
r = []
now = int(time.time())
for path, dnames, fnames in os.walk('.'):
    r.extend([os.path.join(path, x) for x in fnames if (now - int(os.path.getmtime(os.path.join(path, x))))/(60*60) < 8])
print r
```

# 环境参数

# 进程

# 时间

## rfc3339

```py
import dateutil.parser

v = dateutil.parser.parse(d)
v.strftime("%Y-%m-%d %H:%M:%S")
```

## 当前时间

```py
import time
import datetime

######
d = time.strftime("%Y%m%d")

######
d = datetime.datetime.now()
w = d.weekday()
h = d.hour
s = d.strftime("%Y-%m-%d %H:%M:%S")

######
d += datetime.timedelta(hours=8) # 当前时间增加８小时
d += datetime.timedelta(days=8)
d += datetime.timedelta(seconds=8)
d += datetime.timedelta(microseconds=8)
d += datetime.timedelta(milliseconds=8)
d += datetime.timedelta(minutes=8)
d += datetime.timedelta(hours=8)
d += datetime.timedelta(weeks=8)
```

# 线程

## 创立线程

## 线程结束

## 线程间通迅

# 异步

# 网络

# 调用外部程序

# 宏

# Unsafe Rust
