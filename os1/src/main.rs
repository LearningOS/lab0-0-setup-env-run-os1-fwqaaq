#![no_std]
#![no_main]
#![feature(panic_info_message)]

// 使用 use 只能在当前作用域可见，一般用于 mod 之间相互引用
use log::*;

// 使用 extern crate 可以在整个 crate 中可用，它导入的范围是整个包
// 对于 bin 文件使用当前目录的 lib 库，我们应该使用库名，像这样导入，也可以使用 use + 库名
// #[macro_use]
// extern crate log;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack_lower();
        fn boot_stack_top();
    }
    clear_bss();
    logging::init();
    println!("Hello, world!");
    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);

    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(
        "boot_stack_lower [{:#x}, {:#x})",
        boot_stack_lower as usize, boot_stack_top as usize
    );
    error!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    panic!("Shutdown machine!");
}
