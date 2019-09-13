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

