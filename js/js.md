# 特性

动态语言。

对象的定义与其它语言不同。函数，数组，日期，正则表达式都是对象。

Object
* Function
* Array
* Date
* RegExp

# 注释

```js
// comment
/* comment */
```

# 对内存的表示

## 连续的内存

### 数字( Number )

双精度 64 位，不对数字进行整型的区分。实现时，整形为 32 位。

#### NaN : Not a Number

```js
NaN + 5; // NaN
isNaN(NaN); // true
```

#### Infinity and -Infinity : 无穷

```js
1 / 0;   //  Infinity
-1 / 0;  // -Infinity

// 数字是否有限(非无穷)
isFinite(1 / 0);      // false
isFinite(-Infinity);  // false
isFinite(NaN);        // false
```

### 字符串( String )

utf-16

```js
// 字符长度
"abc".length

// 模板字符串
let val = 1;
`abc ${val} xxx
ddd`

// 查找
indexOf()
lastIndexOf()

// 子字符串
'hello'.charAt(0); // "h"

const t = "abcd";
t.slice(1,2);  // "b"
t.slice(0,-2); // "ab"
t.slice(2);  // "cd"

// 替换
'hello, world'.replace('world', 'mars'); // "hello, mars"

// 大小写
'hello'.toUpperCase(); // "HELLO"
"AAA".toLowerCase(); // aaa
```

### 空类型

```js
null
undefined
```

### Symbol

### Boolean

```js
false:  0, empty strings (""), NaN, null, undefined
All other values become true.
```

### 数组( Array )

```js
let a = new Array();
a[0] = 'dog';
a[1] = 'cat';
a[2] = 'hen';
a.length; // 3

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

## 内存数据的读写

### 直接读写( let )

let 的作用域为块级的。

### 只读( const )

### shadwing

### 连续空间部分内容的引用( Slice )

#### String Slice

#### Other Slice

## 类型转换

```js
// 字符串转为数字
parseInt('11', 2); // 3
parseInt('123', 10); // 123
parseInt('010', 10); // 10
parseInt('hello', 10); // NaN
parseInt('3hello', 10); // 3
parseFloat("3.4"); // always uses base 10.

// 数字转为字符串
"" + 123; // "123"
"abc" + 123; // "abc123"

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

// 删除item
delete t.a;
delete t["b"];

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

# 对象

```js
let obj = new Object();
let obj = {};

let obj = {
  name: 'Carrot',
  details: {
    color: 'orange',
    size: 12
  }
};

obj.details.color; // orange
obj['details']['size']; // 12
```

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

```js
var a = 1;
var b = 2;

(function() {
  var b = 3;
  a += b;
})();

a; // 4
b; // 2
```

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

```js
if () {
}
else if () {
}
else {
}
```

### ? :

```js
let a = i > 3 ? 4 : 9;
```

### || &&

```js
let a = str.length || 2;
let a = str && str.length;
```

### switch

```js
switch (action) {
  case 'draw':
    drawIt();
    break;
  case 'eat':
    eatIt();
    break;
  default:
    doNothing();
}

//////////////////
switch (a) {
  case 1: // fallthrough
  case 2:
    eatIt();
    break;
  default:
    doNothing();
}

////////////////////////
switch (1 + 3) {
  case 2 + 2:
    yay();
    break;
  default:
    neverhappens();
}
```

## 循环

### for

```js
/////////////////////////////////////////
// for ( let i = 0; i < 10; i++ )
for (let i = 0; i < 10; i++) {
    console.log(i);
}

/////////////////////////////////////////
// for ( let v of array )
const num = [4, 5, 6];
for (let i of num) {
    console.log(i);
}
// 4
// 5
// 6

/////////////////////////////////////////
// for ( let v in object )
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

## package

## Separating Modules into Different Files

# 面向对象

## 动态类型检测

```js
typeof a[90];
```

# 函数式编程

## 迭代器( Iterators )

```js
// map, filter, reduce
const numbers = [0, 1, 2, 3, 4, 5, 6];

const doubledNumbers = numbers.map(n => n * 2); // [0, 2, 4, 6, 8, 10, 12]

const evenNumbers = numbers.filter(n => n % 2 === 0); // [0, 2, 4, 6]

const sum = numbers.reduce((prev, cur) => prev + cur, 0); // 21
```

# 文件处理

## 打开文件

## 新建文件

## 读写

# 环境参数

# 时间

```js
let v = new Date(); // 本地当前时间
let v = new Date("2019-09-08T09:08:00"); // 本地时间
let v = new Date("2019-09-08"); // utc时间(只有时期，没有时间)

let a = v.getTime(); // 时间戳,豪秒

v.setTime(v.getTime() + 8*60*60*1000); // 增加8小时
```

# 网络
