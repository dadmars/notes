## 一个程序的生命周期

```c
#include <stdio>

int main(int argc, char** argv) {
    printf("hello world\n");
    return 0;
}
```

源文件以数字方式(ascii or utf8)保存在介质中(磁盘，内存)

数字的机器表示方式，是对真实数字的无限接近。这一特性会产生意想不到的行为。

* 预处理
  处理宏
* 编译器
  输出汇编
* 汇编器
  输出机器语言,目标文件
* 链接器
  目标文件合并成可执行程序

### 硬件

* 总线
  各模块传输数据
* I/O
  每个I/O设备都与I/O总线相连
* 内存
* CPU

### 程序运行

* 将程序加载到内存，可能是通过DMA
* CPU 的 PC(程序计数器)指向 main 的位置，开始执行。此时，要执行的指令加载到 CPU
* "hello world\n" 从内存加载到寄存器，再从寄存器复制到显示设备

大部分工作是内容的移动,所以加入cache，提高效率

### cache

意识到 cache 的程序可以将程序性能提高一个数量级。

L0(寄存器) -> L1 -> L2 -> L3 -> 内存 -> 磁盘 -> 远程数据

### 多核

一个 cpu 中有多个核，L3是多个核共享。L0-L2 各个核独立

L0(寄存器) -> L1(数据cache, 指令cache) -> L2 -> L3(多个核共享)

### 操作系统

* 文件是对I/O设备的抽象
* 虚拟内存是对主存和磁盘I/O设备的抽象
* 进程是对CPU,主存和磁盘I/O设备的抽象

## 进程

操作系统提供一个假象，好象系统只有一个程序在运行，此程序独立占用 cpu, 内存，I/O设备。这种抽象通过进程来实现。

## 虚拟内存

虚拟内存为进程提供一个假象，好象进程独立占用主存。每个进程看到的内存都是一致的，称为虚拟地址空间

虚拟内存划分为不同区域，从最低地址开始，分别是：

* 程序代码和全局数据
* 堆
* 共享库
  在地址空间的中间，存放 c 标准库等
* 栈
  内核虚拟内存
  栈随机化：为防止攻击，程序每次运行，在栈上随机分配一段随机大小的内存空间
  栈破坏检测： 入栈时在栈中插入一随机值,返回时，对此值进行检测

## 字节序

```bash
整数 0x1234567 位于地址 0x100

小端
0x100   0x101   0x102   0x103
67      45      23      01

大端
0x100   0x101   0x102   0x103
01      23      45      67
```

## 溢出

整数,浮点数运算有可能产生溢出，从而产生未知行为