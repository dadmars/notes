# 关键概念( wasm )

WebAssembly( wasm ), 有两种格式:

* .wat  webassembly text
* .wasm 二进制, 运行在wasm 虚拟机中

## Module

可以 export 一些方法，也可以通过 import 引入，和 es6 中的 module 概念相同。

## Memory

线性数组，可以读写其中的内容，内存可以增长，但不可以缩小．js的内存管理对此区域不超作用，只能读取或写入基本数据，如u8, i32..　wasm　函数也只能读取或写入　scale 数据.通过此方式，js与wasm进行交互.

## Table

只读线性数组，存放函数等内容

## Instance

运行实例

# Rust

## 安装

```bash
npm install react-app-rewired wasm-loader -D

cargo install cargo-generate
cargo install wasm-pack

cargo generate --git https://github.com/rustwasm/wasm-pack-template
wasm-pack build

npm init wasm-app www
cd wasm-game-of-life/www
npm install
npm run start
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

## Shrinking .wasm Size

setting opt-level = "z", and running wasm-opt -Oz

compress it with gzip (which nearly every HTTP server does) 

$ gzip -9 < pkg/wasm_game_of_life_bg.wasm | wc -c

Use the wasm-snip tool to remove the panicking infrastructure functions 

[features]
default = ["wee_alloc"]