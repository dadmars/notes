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

# 环境参数

# 进程

# 时间

## rfc3339

```js
import dateutil.parser

v = dateutil.parser.parse(d)
v.strftime("%Y-%m-%d %H:%M:%S")
```

## 当前时间

```js
import time
d = time.strftime("%Y%m%d")
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
