# 这些汇编代码会被内核率先执行, 为了区分 .text, 我们将其写成 .text.entry 
# 并且它之后的所有代码都会放入 .text.entry，我们将其作为入口点
    .section .text.entry
    .globl _start
_start:
# sp： stack pointer
# boot_stack_top: 标识栈顶的位置
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    # boot_stack: 标识栈的下限位置
    .globl boot_stack_lower
boot_stack_lower:
# 预留运行程序的栈空间 64kb
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: