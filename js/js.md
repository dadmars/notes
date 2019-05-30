# 数组和字符串

## 数组转成字符串

```c
const t = ["a", "b", "c"];
t.join("-");  // "a-b-c"
t.join();     // "a,b,c"
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