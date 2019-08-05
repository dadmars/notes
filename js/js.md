# 函数参数列表: arguments

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

# const

对象和数组指向的内容可以被修改，类似于 c++ 中的 int *const p;

```js
const person = {
  name: 'Nick'
};
person.name = 'John';

const person = [];
person.push('John');
```

# 数组

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
```

## for..of

```js
const num = [4, 5, 6];
for (let i of num) {
    console.log(i);
}
// 4
// 5
// 6
```

## map, filter, reduce

```js
const numbers = [0, 1, 2, 3, 4, 5, 6];

const doubledNumbers = numbers.map(n => n * 2); // [0, 2, 4, 6, 8, 10, 12]

const evenNumbers = numbers.filter(n => n % 2 === 0); // [0, 2, 4, 6]

const sum = numbers.reduce((prev, next) => prev + next, 0); // 21
```

## 转成字符串

```js
const t = ["a", "b", "c"];
t.join("-");  // "a-b-c"
t.join();     // "a,b,c"

t.toString()  // "a,b,c"
```

# 数字

```js
NaN + 5; // NaN

isNaN(NaN); // true

1 / 0;   //  Infinity
-1 / 0;  // -Infinity

isFinite(1 / 0);      // false
isFinite(-Infinity);  // false
isFinite(NaN);        // false
```

# 字符串

## 模板字符串

```js
`abc ${val} xxx
ddd`
```

## 转为数字

```js
parseInt('123', 10); // 123
parseInt('010', 10); // 10
parseInt('hello', 10); // NaN
parseFloat("3.4"); // always uses base 10.
```

## 查找

```js
indexOf()
lastIndexOf()
```

## 转成数组

```js
const t = "a,b,c";
t.split(",");  // ["a", "b", "c"]

const t = "a.b.c";
t.split(".");  // ["a", "b", "c"]
```

## 截取子字符串

```js
const t = "abcd";
t.slice(1,2);  // "b"
t.slice(0,-2); // "ab"
t.slice(2);  // "cd"
```

## 替换

```js
const t = "abcd";
t.replace("a", "d");
```

## 大小写

```js
'hello'.toUpperCase(); // "HELLO"
radData.toLowerCase();
```

# 对象

## for..in

```js
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

## 得到keys

```js
const t = {
    a: 1,
    b: 2,
    c: 3
};

Object.keys(t);   // ["a", "b", "c"]
```
