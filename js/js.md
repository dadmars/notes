# 数组和字符串

## 数组转成字符串

```c
const t = ["a", "b", "c"];
t.join("-");  // "a-b-c"
t.join();     // "a,b,c"
```

## 截取字符串

```c
const t = "abcd";
t.slice(1,2);
t.slice(0,-2);
t.slice(2);
```

## 替换

```c
const t = "abcd";
t.replace("a", "d");
```

# 对象

## 得到keys

```c
const t = {
    a: 1,
    b: 2,
    c: 3
};

Object.keys(t);   // ["a", "b", "c"]
```
