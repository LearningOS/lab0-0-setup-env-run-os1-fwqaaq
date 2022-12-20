use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use lazy_static::*;

const USER_STACK_SIZE: usize = 4096;
const KERNEL_STACK_SIZE: usize = 4096 * 20;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

// 内核栈
#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

// 用户栈
#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

// 初始化全局的变量
static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

// 取栈顶顶数据（高地址），需要 + USER_STACK_SIZE
impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

// 应用管理器，将会实例化为全局变量，使得任何函数都可访问
struct AppManager {
    num_app: usize,
    current_app: usize, //访问的第几个应用
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        info!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            info!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    // 加载应用程序的二进制镜像到 0x80400000
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!");
        }
        info!("[kernel] Loading app_{}", app_id);
        // clear icache, 清理指令缓存
        core::arch::asm!("fence.i");
        // clear app area，清空一块内存区域，准备加载 app 二进制镜像
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        // 加载 app 镜像
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        // 将找到的 app 区域拷贝到之前的区域
        app_dst.copy_from_slice(app_src);
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

// 初始化 Appmanager
lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app(); // 找 link_app.S 中的 _num_app 入口
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

pub fn init() {
    print_app_info();
}

pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

// 批处理的核心，加载并运行下一个程序
pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    drop(app_manager);
    // before this we have to drop local variables related to resources manually
    // and release the resources
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("Unreachable in batch::run_current_app!");
}
