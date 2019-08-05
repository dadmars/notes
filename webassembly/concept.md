# key concepts

Module: Represents a WebAssembly binary that has been compiled by the browser into executable machine code.  A Module is stateless and thus, like a Blob, can be explicitly shared between windows and workers (via postMessage()).  A Module declares imports and exports just like an ES2015 module.

Memory: A resizable ArrayBuffer that contains the linear array of bytes read and written by WebAssembly’s low-level memory access instructions.

Table: A resizable typed array of references (e.g. to functions) that could not otherwise be stored as raw bytes in Memory (for safety and portability reasons).

Instance: A Module paired with all the state it uses at runtime including a Memory, Table, and set of imported values.  An Instance is like an ES2015 module that has been loaded into a particular global with a particular set of imports.
