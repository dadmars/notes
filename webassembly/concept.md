# What is WebAssembly

With the advent of WebAssembly appearing in browsers, the virtual machine that we talked about earlier will now load and run two types of code — JavaScript AND WebAssembly.

# key concepts

Module: Represents a WebAssembly binary that has been compiled by the browser into executable machine code.  A Module is stateless and thus, like a Blob, can be explicitly shared between windows and workers (via postMessage()).  A Module declares imports and exports just like an ES2015 module.

Memory: A resizable ArrayBuffer that contains the linear array of bytes read and written by WebAssembly’s low-level memory access instructions.

Table: A resizable typed array of references (e.g. to functions) that could not otherwise be stored as raw bytes in Memory (for safety and portability reasons).

Instance: A Module paired with all the state it uses at runtime including a Memory, Table, and set of imported values.  An Instance is like an ES2015 module that has been loaded into a particular global with a particular set of imports.

# How do use WebAssembly

* Porting a C/C++ application with Emscripten.
* Writing or generating WebAssembly directly at the assembly level.
* Writing a Rust application and targetting WebAssembly as its output.

## Porting from C/C++

Two of the many options for creating WASM code are an online wasm assembler or Emscripten. There are a number of online WASM assembler choices, such as:

    WasmFiddle
    WasmFiddle++
    WasmExplorer

The Emscripten tool is able to take just about any C/C++ source code and compile it into a .wasm module, plus the necessary JavaScript "glue" code for loading and running the module, and an HTML document to display the results of the code.

In a nutshell, the process works as follows:

Emscripten first feeds the C/C++ into clang+LLVM — a mature open-source C/C++ compiler toolchain, shipped as part of XCode on OSX for example.

Emscripten transforms the compiled result of clang+LLVM into a .wasm binary.

By itself, WebAssembly cannot currently directly access the DOM; it can only call JavaScript, passing in integer and floating point primitive data types. Thus, to access any Web API, WebAssembly needs to call out to JavaScript, which then makes the Web API call. Emscripten therefore creates the HTML and JavaScript glue code needed to achieve this. 

The JavaScript glue code is not as simple as you might imagine. For a start, Emscripten implements popular C/C++ libraries like SDL, OpenGL, OpenAL, and parts of POSIX. These libraries are implemented in terms of Web APIs and thus each one requires some JavaScript glue code to connect WebAssembly to the underlying Web API.

So part of the glue code is implementing the functionality of each respective library used by the C/C++ code. The glue code also contains the logic for calling the above-mentioned WebAssembly JavaScript APIs to fetch, load and run the .wasm file.

The generated HTML document loads the JavaScript glue file and writes stdout to a <textarea>. If the application uses OpenGL, the HTML also contains a <canvas> element that is used as the rendering target. It’s very easy to modify the Emscripten output and turn it into whatever web app you require.

## Writing WebAssembly directly

In the same fashion as physical assembly languages, the WebAssembly binary format has a text representation — the two have a 1:1 correspondence. You can write or generate this format by hand and then convert it into the binary format with any of several WebAssemby text-to-binary tools.

## Writing Rust Targetting WebAssembly

It is also possible to write Rust code and compile over to WebAssembly, thanks to the tireless work of the Rust WebAssembly Working Group. You can get started with installing the necessary toolchain, compiling a sample Rust program to a WebAssembly npm package, and using that in a sample web app, over at our Compiling from Rust to WebAssembly article.