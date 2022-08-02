### 简单 fun

```js
(module
  (func (export "sum") (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add))

```

### 导入 module & 导出 memory

```js
const importObj = {
  console: {
    log: () => {
      console.log("This is ...");
    },
    error: () => {
      console.log("Some error");
    },
  },
};
```

```js
//wasm 接受了import对象
//wasm 初始化了memory 大小 1页等于 64kb
//wasm 定义了一个data数据，从内存0开始，存入了字符“Hi”
//wasm 定义了一个方法$sum
//wasm 导出内存变量$mem作为mem
(module
    (import "console" "log" (func $log))
    (import "console" "error" (func $error))
    (memory $mem 1)
    (data (i32.const 0) "Hi")
    (func $sum (param i32 i32) (result i32)
        call $log
        call $error
        local.get 0
        local.get 1
        i32.add
    )
    (export "mem" (memory $mem))
    (export "sum" (func $sum))
)
```

### js 导入 memory

```js
const memory = new WebAssembly.Memory({initial: 1});
const importObj = {
  js: {
    mem: memory,
  },
  console: {
    log: () => {
      console.log("This is ...");
    },
    error: () => {
      console.log("Some error");
    },
  },
};
```

```js
(module
    (import "console" "log" (func $log))
    (import "console" "error" (func $error))
    (memory (import "js" "mem") 1)
    (data (i32.const 0) "Hi")
    (func $sum (param i32 i32) (result i32)
        call $log
        call $error
        local.get 0
        local.get 1
        i32.add
    )
    (export "sum" (func $sum))
)
```
