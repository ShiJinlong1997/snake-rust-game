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

wasm 只有 4 种类型：i32, i64, f32, f64

---

这是：

模块里声明函数，有两个参数 `a` 和 `b`，函数体内读这两个参数，并返回二者之和

```wasm
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

[https://webassembly.github.io/wabt/demo/wat2wasm/](https://webassembly.github.io/wabt/demo/wat2wasm/) 用于试验 wasm 代码

调试好 wasm 代码就下载，下载了的 .wasm 文件若要查看十六进制内容

- linux
  ```bash
  xxd -g1 sum.wasm
  ```

- win10
  ```bash
  Format-Hex -Path sum.wasm
  ```

在终端显示

```

           00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F

00000000   00 61 73 6D 01 00 00 00 01 07 01 60 02 7F 7F 01  .asm.......`..
00000010   7F 03 02 01 00 07 08 01 04 24 73 75 6D 00 00 0A  ........$sum...
00000020   09 01 07 00 20 00 20 01 6A 0B 00 18 04 6E 61 6D  .... . .j....nam
00000030   65 01 06 01 00 03 73 75 6D 02 09 01 00 02 00 01  e.....sum.......
00000040   61 01 01 62                                      a..b
```

共 67 条指令，右侧的是咬合的纹理格式