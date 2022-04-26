### 内存布局

![../_images/MemoryLayout.png](https://s2.loli.net/2022/04/26/pC75ZzlIU6Mdxot.png)

`.text`是代码段，存放的是汇编指令

`.rodata`已经初始化的只读全局数据，通常为常数或常量字符串

`.data`可以修改的已初始化的全局数据

`.bss`未初始化全局数据，通常有程序加载者进行零初始化。

`heap`堆，用来存放程序运行时动态分配的数据，有低地址到高地址

`stack`栈，函数调用上下文的保存与恢复，每个函数作用域内的本地变量也存放于其栈帧内，从高地址到低地址增长

> 上图的内存布局中`text`段全是汇编指令就保证了CPU在平滑的控制流中知道下一条指令的位置。而链接器（编译中有一步叫链接）确保了可执行程序的所有指令都被重排进代码段。

### 第一段汇编

说明：其实说第一行并不严谨，因为在rcore中，直接省略了引导程序的书写，而是采用rustsbi，rustsbi会进行硬件初始化后，跳转到一个实现约定好的地址`0x80200000`继续执行，在自定义链接脚本的帮助下，硬件初始化后就会跳转下文中的地方继续执行。

```asm
    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:
```
其实上文的汇编只是做了一些初始化，并且定义了64KB的栈空间（没有进行零初始化，因为栈会自动增长，不需要初始化，不会有访问为定义内存的情况）

链接器如下：主要是链接不同数据到不同区域，并定义了入口

```
OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
```



