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

npm link

npm init wasm-app www
cd wasm-game-of-life/www
npm install
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


Preston Richey

    blogprojectsabout

Up and Running with React + Rust + Wasm

I’m super excited about WebAssembly! It’s fast, (can be) small, and extremely portable. In fact, I wouldn’t be surprised if in the near future most web developers write code that eventually gets compiled to Wasm.

But currently there’s not a huge selection of resources showing how to get started with WebAssembly, and I couldn’t find any tutorials that worked with create-react-app. Most focus on writing and compiling a module, but rush over the details of actually using Wasm code. What follows is a basic setup for a React app using WebAssembly that should serve as a good foundation for more complex applications.
Setup

This post assumes basic familiarity with the command line, React, and the basics of Rust tooling. If you get stuck, the Rust Book and Intro to React are great resources.

First, go through the setup docs here in the very helpful Rust Wasm book. Install the Rust toolchain, wasm-pack, cargo-generate, and make sure npm is installed and up to date.

With a few substantial differences, I’m roughly following the directory structure shown here, but I’ll go through the process step by step. I also have the code up on my Github, here.

Our project will have 2 main directories inside of the root, lib, which will house our Rust code, and app, which will house our React app. For local development, we’ll use npm link to get around needing to deploy our Wasm code as a module, so these two directories really don’t need to be next to each other; I’m just doing so for convenience.
Creating our Rust module

First, make a directory and move into it.

mkdir hello-wasm && cd hello-wasm

Next, let’s add our Rust code. I’m putting it in hello-wasm/lib.

mkdir lib && cd lib

Let’s add our Cargo.toml, which will act as a manifest for our Rust library. (More info here.)

[package]
name = "hello-wasm"
version = "0.1.0"
authors = ["Your Name <you@email.com>"]

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "=0.2.34"

Note: we need this specific wasm-bindgen version because of reasons.

I’m omitting some niceties included in the Rust book setup doc, like allocation optimizations and an improved error handling. (More here.) You should probably include those if you plan on going further than this tutorial, but for the sake of simplicity I’ll leave that as an exercise for the reader. (That’s you!)

Next, we’ll add our library code which will be compiled to Wasm.

mkdir src && touch src/lib.rs

Make lib.rs look like this:

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("👋 from Wasm");
}

From inside the pkg directory, run

npm link

npm install react-app-rewired wasm-loader -D
config-overrides.js

const path = require('path');

module.exports = function override(config, env) {
  const wasmExtensionRegExp = /\.wasm$/;

  config.resolve.extensions.push('.wasm');

  config.module.rules.forEach(rule => {
    (rule.oneOf || []).forEach(oneOf => {
      if (oneOf.loader && oneOf.loader.indexOf('file-loader') >= 0) {
        // Make file-loader ignore WASM files
        oneOf.exclude.push(wasmExtensionRegExp);
      }
    });
  });

  // Add a dedicated loader for WASM
  config.module.rules.push({
    test: wasmExtensionRegExp,
    include: path.resolve(__dirname, 'src'),
    use: [{ loader: require.resolve('wasm-loader'), options: {} }]
  });

  return config;
};

"scripts": {
  "start": "react-app-rewired start",
  "build": "react-app-rewired build",
  "test": "react-app-rewired test",
}

From inside our app directory, run the following:

npm link hello-wasm

App.js 

import React, { useState } from 'react';
import './App.css';

const Loaded = ({ wasm }) => <button onClick={wasm.greet}>Click me</button>;

const Unloaded = ({ loading, loadWasm }) => {
  return loading ? (
    <div>Loading...</div>
  ) : (
    <button onClick={loadWasm}>Load library</button>
  );
};

const App = () => {
  const [loading, setLoading] = useState(false);
  const [wasm, setWasm] = useState(null);

  const loadWasm = async () => {
    try {
      setLoading(true);
      const wasm = await import('hello-wasm');
      setWasm(wasm);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        {wasm ? (
          <Loaded wasm={wasm} />
        ) : (
          <Unloaded loading={loading} loadWasm={loadWasm} />
        )}
      </header>
    </div>
  );
};

First, inside our App component, we have a function loadWasm:

const loadWasm = async () => {
  try {
    setLoading(true);
    const wasm = await import('hello-wasm');
    setWasm(wasm);
  } finally {
    setLoading(false);
  }
};

Notably, this function is asynchronous. You might have expected to import our Wasm module at the top of the file, like you would most other modules (import {wasm} from 'hello-wasm', e.g.). We do this for a few reasons. First, the browser gives the following error when attempting to import the wasm module synchronously:

WebAssembly module is included in initial chunk.
This is not allowed, because WebAssembly download and compilation must happen asynchronous.