## wat 转 wasm 后查看十六进制内容

wasm 是一种不同类型的代码，比 js 快得多，快如原生
可以用任何语言给它支持 wasm。
rust 转 wasm

并能在浏览器加载与它们的 js 相同的上述可调整指令
浏览器会执行这些指令，而且这些指令非常快

we need compile Rust to WebAssembly code
WebAssembly can be executed in the browser
it runs at natie speed
3d apps, cad, virtual reality, system application, games

必须做很多事情，它需要大量资源，并且即使使用它也必须具有高性能。

wasm 只有 4 种类型：i32, i64, f32, f64。

---

这是：

模块里声明函数，有两个参数 `a` 和 `b`，函数体内读这两个参数，并返回二者之和。

```wat
(module
  (func $sum (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add)
  (export "$sum" (func $sum))
)
```

各部分：

模块、导出、函数、参数声明和返回类型声明各自被括号包围。

关键字直接写，函数名、参数名前有 "$"。

被双引号包围的导出成员名能被 js 用就行。

[https://webassembly.github.io/wabt/demo/wat2wasm/](https://webassembly.github.io/wabt/demo/wat2wasm/) 用于试验 wasm 代码。

调试好 wasm 代码就下载，下载了的 .wasm 文件若要查看十六进制内容。

- linux
  ```bash
  xxd -g1 sum.wasm
  ```

- win10
  ```bash
  Format-Hex -Path sum.wasm
  ```

在终端显示。

```

           00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F

00000000   00 61 73 6D 01 00 00 00 01 07 01 60 02 7F 7F 01  .asm.......`..
00000010   7F 03 02 01 00 07 08 01 04 24 73 75 6D 00 00 0A  ........$sum...
00000020   09 01 07 00 20 00 20 01 6A 0B 00 18 04 6E 61 6D  .... . .j....nam
00000030   65 01 06 01 00 03 73 75 6D 02 09 01 00 02 00 01  e.....sum.......
00000040   61 01 01 62                                      a..b
```

共 67 条指令，右侧的是咬合的纹理格式。

## 获取 wasm 调其模块成员

```js
fetch('sum.wasm')
  .then(response => response.arrayBuffer())
  .then(WebAssembly.instantiate)
  .then(wasm => {
    const wasmMod = wasm.instance.exports;
    wasmMod.sum(10, 20)
  });
```

## js 给 wasm 传入模块成员

声明 `WebAssembly.instantiate` 的第二参数 `importObject`。

```js
const importObject = {
  console: {
    log: () => {
      console.log('log');
    },
    error: () => {
      console.error('error');
    },
  },
};
```

在 wat 引入 js 的 `importObject`：

```wat
(module
  (import "console" "log" (func $log))
  (import "console" "error" (func $error))
  (func $sum (param $a i32) (param $b i32) (result i32)
    call $log
    call $error
    local.get $a
    local.get $b
    i32.add)
  (export "sum" (func $sum))
)
```

在函数体调用引入模块成员。

传入 `importObject` 如下所示：

```js
fetch('/import_object.wasm')
.then(response => response.arrayBuffer())
.then(buffer => WebAssembly.instantiate(buffer, importObject))
.then(wasm => {
  console.log(wasm.instance.exports.sum(10, 20))
});
```

## 写入内存供 js 读

```wat
(module
  (import "console" "log" (func $log))
  (import "console" "error" (func $error))
  (memory 1)
  (data (i32.const 0) "Hi")
  (func $sum (param $a i32) (param $b i32) (result i32)
    call $log
    call $error
    local.get $a
    local.get $b
    i32.add)
  (export "mem" (memory 0))
  (export "sum" (func $sum))
)
```

`memory 1` 代表 1 页内存, 每页约 64KB。

`(export "mem" (memory 0))` 访问创建的第一个内存。

`(memory 1)` 也可以写成 `(memory $mem 1)` 用于给内存起别名

之后读内存

```js
fetch('/mem_000.wasm')
  .then(response => response.arrayBuffer())
  .then(buffer => WebAssembly.instantiate(buffer, importObject))
  .then(wasm => {
    const wasmMod = wasm.instance.exports;
    console.log(wasmMod.sum(10, 20));
    
    const uint8Array = new Uint8Array(wasmMod.mem.buffer, 0, 2);
    const txt = new TextDecoder().decode(uint8Array)
  });
```

## js 写内存传入 rust

```js
const memory = new WebAssembly.Memory({ initial: 1 });

const importObject = {
  js: {
    mem: memory,
  },
  console: {
    log: () => {
      console.log('log');
    },
    error: () => {
      console.error('error');
    },
  },
};
```

```wat
(module
  (import "console" "log" (func $log))
  (import "console" "error" (func $error))
  (memory (import "js" "mem") 1)
  (data (i32.const 0) "Hi")
  (func $sum (param $a i32) (param $b i32) (result i32)
    call $log
    call $error
    local.get $a
    local.get $b
    i32.add)
  (export "sum" (func $sum))
)
```

不再导出内存，而是导入 js 申请来的内存，在里面写入数据 "Hi"
