<!-- TOC -->autoauto- [特性](#特性)auto- [生存期( Lifetimes )](#生存期-lifetimes-)auto- [注释](#注释)auto- [对内存的表示](#对内存的表示)auto    - [连续的内存](#连续的内存)auto        - [数字](#数字)auto        - [数组](#数组)auto        - [字符串](#字符串)auto        - [向量](#向量)auto    - [内存数据的读写](#内存数据的读写)auto        - [直接读写](#直接读写)auto        - [只读](#只读)auto        - [shadwing](#shadwing)auto        - [间接读写](#间接读写)auto            - [指针](#指针)auto                - [指针类型](#指针类型)auto                - [空指针](#空指针)auto                - [得到指向的值](#得到指向的值)auto                - [指针运算](#指针运算)auto            - [智能指针](#智能指针)auto            - [引用](#引用)auto                - [连续空间部分内容的引用( Slice )](#连续空间部分内容的引用-slice-)auto                    - [String Slice](#string-slice)auto                    - [Other Slice](#other-slice)auto    - [类型转换](#类型转换)auto    - [destructuring](#destructuring)auto    - [非连续的内存](#非连续的内存)auto        - [对象](#对象)auto        - [Set](#set)auto        - [链表](#链表)auto        - [树](#树)auto- [空值](#空值)auto- [空对象](#空对象)auto- [结构化](#结构化)auto    - [元组( tuple )](#元组-tuple-)auto    - [结构( struct )](#结构-struct-)auto        - [Tuple Structs](#tuple-structs)auto        - [Unit-Like Structs](#unit-like-structs)auto    - [枚举( enum )](#枚举-enum-)auto- [扩展](#扩展)auto    - [Range](#range)auto- [程序指令的执行](#程序指令的执行)auto    - [函数](#函数)auto    - [一般函数](#一般函数)auto    - [函数参数列表: arguments](#函数参数列表-arguments)auto    - [闭包( closure )](#闭包-closure-)auto    - [分支](#分支)auto        - [if](#if)auto    - [match](#match)auto    - [循环](#循环)auto        - [loop](#loop)auto        - [for](#for)auto        - [while](#while)auto- [范围](#范围)auto- [常量](#常量)auto    - [字符和字符串常量](#字符和字符串常量)auto    - [数字字面常量](#数字字面常量)auto- [数据封装](#数据封装)auto    - [struct](#struct)auto- [错误处理](#错误处理)auto- [代码组织( 模块化 )](#代码组织-模块化-)auto    - [工具](#工具)auto    - [workspace](#workspace)auto    - [package](#package)auto    - [key: mod, pub](#key-mod-pub)auto    - [key: crate, self, super](#key-crate-self-super)auto    - [key: use, as](#key-use-as)auto    - [External Packages](#external-packages)auto    - [Separating Modules into Different Files](#separating-modules-into-different-files)auto- [自动化测试](#自动化测试)auto    - [单元测试](#单元测试)auto    - [集成测试](#集成测试)auto- [修饰器编程( Decorator )](#修饰器编程-decorator-)auto- [面向对象](#面向对象)auto    - [trait](#trait)auto    - [trait object](#trait-object)auto    - [动态类型检测](#动态类型检测)auto- [函数式编程](#函数式编程)auto- [泛型](#泛型)auto    - [迭代器( Iterators )](#迭代器-iterators-)auto- [命令行](#命令行)auto    - [命令行参数](#命令行参数)auto    - [用户输入](#用户输入)auto- [文件处理](#文件处理)auto    - [打开文件](#打开文件)auto    - [新建文件](#新建文件)auto    - [读写](#读写)auto- [环境参数](#环境参数)auto- [进程](#进程)auto- [时间](#时间)auto- [线程](#线程)auto    - [创立线程](#创立线程)auto    - [线程结束](#线程结束)auto    - [线程间通迅](#线程间通迅)auto    - [线程间同步](#线程间同步)auto- [异步](#异步)auto- [网络](#网络)autoauto<!-- /TOC -->

# 特性

# 生存期( Lifetimes )

# 注释

```js
// comment
/* comment */
```

# 对内存的表示

## 连续的内存

### 数字

```js
NaN + 5; // NaN

isNaN(NaN); // true

1 / 0;   //  Infinity
-1 / 0;  // -Infinity

isFinite(1 / 0);      // false
isFinite(-Infinity);  // false
isFinite(NaN);        // false
```

### 数组

```js
['dog', 'cat', 'hen'].forEach(function(currentValue, index, array) {});

a.push(item);
a.push(item1, ..., itemN);
a.pop() // Removes and returns the last item.
a.unshift(item1[, item2[, ...[, itemN]]]) // Prepends items to the start of the array.
a.shift() // Removes and returns the first item.

a.concat(item1[, item2[, ...[, itemN]]]) // Returns a new array with the items added on to it.
a.reverse() // Reverses the array.
a.sort([cmpfn]) // Takes an optional comparison function.
a.splice(start, delcount[, item1[, ...[, itemN]]]) //Lets you modify an array by deleting a section and replacing it with more items.


// 转成字符串
const t = ["a", "b", "c"];
t.join("-");  // "a-b-c"
t.join();     // "a,b,c"

t.toString()  // "a,b,c"
```

### 字符串

```js
// 模板字符串
let val = 1;
`abc ${val} xxx
ddd`

// 查找
indexOf()
lastIndexOf()

// 截取子字符串
const t = "abcd";
t.slice(1,2);  // "b"
t.slice(0,-2); // "ab"
t.slice(2);  // "cd"

// 替换
const t = "abcd";
t.replace("a", "d");

// 大小写
'hello'.toUpperCase(); // "HELLO"
radData.toLowerCase();
```

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

```js
// 字符串转为数字
parseInt('123', 10); // 123
parseInt('010', 10); // 10
parseInt('hello', 10); // NaN
parseFloat("3.4"); // always uses base 10.

// 字符串转为数组
const t = "a,b,c";
t.split(",");  // ["a", "b", "c"]

const t = "a.b.c";
t.split(".");  // ["a", "b", "c"]
```

## destructuring

```js
let [a, b, ...rest] = [10, 20, 30, 40, 50];
console.log(a); // 10
console.log(b); // 20
console.log(rest); // [30, 40, 50]

let {a, b, ...rest} = {a: 10, b: 20, c: 30, d: 40};
console.log(a); // 10
console.log(b); // 20
console.log(rest); // {c: 30, d: 40}

[a=5, b=7] = [1];
console.log(a); // 1
console.log(b); // 7

var a = 1;
var b = 3;
[a, b] = [b, a];
console.log(a); // 3
console.log(b); // 1

var [a, , b] = [1, 2, 3];
console.log(a); // 1
console.log(b); // 3

[,,] = [1, 2, 3];

var {a: aa = 10, b: bb = 5} = {a: 3};
console.log(aa); // 3
console.log(bb); // 5

var user = {
  id: 42,
  displayName: 'jdoe',
  fullName: {
    firstName: 'John',
    lastName: 'Doe'
  }
};
function whois({displayName, fullName: {firstName: name}}) {
  return `${displayName} is ${name}`;
}

let {a, b, ...rest} = {a: 10, b: 20, c: 30, d: 40}
a; // 10
b; // 20
rest; // { c: 30, d: 40 }
```

## 非连续的内存

### 对象

```js
// 得到keys
const t = {
    a: 1,
    b: 2,
    c: 3
};

Object.keys(t);   // ["a", "b", "c"]

// 遍历对象
for (let [key, value] of Object.entries(t)) {
  console.log(`${key}: ${value}`);
}
Object.entries(t).forEach(([key, value]) => {
  console.log(`${key}: ${value}`);
});
// a: 1
// b: 2
// c: 3
```

### Set

```js
////////////////////////////////////////////
const set1 = new Set([1, 2, 3, 4, 5]);

set1.has(1); // true
set1.has(6); // false

////////////////////////////////////////////
var mySet = new Set();

mySet.add(1); // Set [ 1 ]
mySet.add('some text'); // Set [ 1, 'some text' ]
mySet.delete(1); // removes 1 from the set

for (let item of mySet) console.log(item);
for (let item of mySet.keys()) console.log(item);
for (let item of mySet.values()) console.log(item);

let a = [...mySet2]; // to array
var myArr = Array.from(mySet); // to array
```

### 链表

### 树

# 空值

# 空对象

# 结构化

## 元组( tuple )

## 结构( struct )

### Tuple Structs

### Unit-Like Structs

## 枚举( enum )

# 扩展

```js
const arr1 = ["a", "b", "c"];
const arr2 = [...arr1, "d", "e", "f"]; // ["a", "b", "c", "d", "e", "f"]

function myFunc(x, y, ...params) {
  console.log(x);
  console.log(y);
  console.log(params)
}
myFunc("a", "b", "c", "d", "e", "f")
// "a"
// "b"
// ["c", "d", "e", "f"]

const { x, y, ...z } = { x: 1, y: 2, a: 3, b: 4 };
console.log(x); // 1
console.log(y); // 2
console.log(z); // { a: 3, b: 4 }

const n = { x, y, ...z };
console.log(n); // { x: 1, y: 2, a: 3, b: 4 }
```

## Range

# 程序指令的执行

## 函数

## 一般函数

## 函数参数列表: arguments

```js
function myFunc() {
  for (var i = 0; i < arguments.length; i++) {
    console.log(arguments[i]);
  }
}

myFunc("Nick", "Anderson", 10, 12, 6);
// "Nick"
// "Anderson"
// 10
// 12
// 6
```

## 闭包( closure )

## 分支

### if

## match

## 循环

### loop

### for

```js
// for .. of
const num = [4, 5, 6];
for (let i of num) {
    console.log(i);
}
// 4
// 5
// 6

// for .. in
const t = {
    a: 1,
    b: 2,
    c: 3
};
for (let t in num) {
    console.log(i);
}
// a
// b
// c
```

### while

# 范围

# 常量

对象和数组指向的内容可以被修改，类似于 c++ 中的 int *const p;

```js
const person = {
  name: 'Nick'
};
person.name = 'John';

const person = [];
person.push('John');
```

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

# 面向对象

## trait

## trait object

## 动态类型检测

# 函数式编程

# 泛型

## 迭代器( Iterators )

```js
// map, filter, reduce
const numbers = [0, 1, 2, 3, 4, 5, 6];

const doubledNumbers = numbers.map(n => n * 2); // [0, 2, 4, 6, 8, 10, 12]

const evenNumbers = numbers.filter(n => n % 2 === 0); // [0, 2, 4, 6]

const sum = numbers.reduce((prev, cur) => prev + cur, 0); // 21
```

# 命令行

## 命令行参数

## 用户输入

# 文件处理

## 打开文件

## 新建文件

## 读写

# 环境参数

# 进程

# 时间

```js
let v = new Date(); // 本地当前时间
let v = new Date("2019-09-08T09:08:00"); // 本地时间
let v = new Date("2019-09-08"); // utc时间(只有时期，没有时间)

let a = v.getTime(); // 时间戳,豪秒

v.setTime(v.getTime() + 8*60*60*1000); // 增加8小时
```

# 线程

## 创立线程

## 线程结束

## 线程间通迅

## 线程间同步

# 异步

# 网络

