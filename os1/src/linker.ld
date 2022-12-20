OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

/* 代码段只有 .text 一个段，存放程序的所有汇编代码
   .rodata 存放只读全局数据，.data 存放可修改的全局数据
   .bss 存放未初始化的全局啊数据，程序的加载者代为进行零初始化，逐字节清 0
   heap 运行时动态分配的数据
   stack 函数调用上下文的保存和恢复，以及作用域内的局部变量
 */

/* 每一行的格式都是 <ObjectFile>(SelectionName) */
SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;
    /* 将 .text.entry 放在 .text 段文件的开头  */
    /* 开始地址 */
    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    /* 结束地址 */
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