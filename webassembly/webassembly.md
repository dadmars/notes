# 关键概念( wasm )

## Module

可以 export 一些方法，也可以通过 import 引入，和 es6 中的 module 概念相同。

## Memory

线性数组，可以读写其中的内容，内存可以增长，但不可以缩小

## Table

只读线性数组，存放函数等内容

## Instance

运行实例

# Rust

## 安装

```bash
cargo install wasm-pack
```

## Hello World

```bash
cargo new --lib hello-wasm
```

```rust
File: src/lib.rs

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    // call js function alert
    pub fn alert(s: &str);
}

#[wasm_bindgen]
// the function can be called by js
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

File: Cargo.toml

// build a cdylib version lib
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

```bash
wasm-pack build --scope mynpmusername
cd pkg
npm publish --access=public
```

```js
File: package.json

"dependencies": {
    "@mynpmusername/hello-wasm": "^0.1.0",
    "wasm-game-of-life": "file:../pkg", // Add this line!
},

File: xx.js

import * as wasm from "wasm-game-of-life";

wasm.greet();
```
