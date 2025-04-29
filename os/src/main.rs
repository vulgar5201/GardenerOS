#![no_std]
#![no_main]
#![feature(panic_info_message)]  // 用于增强panic信息

use core::panic::PanicInfo;
use core::arch::asm;
use core::fmt::{self, Write};

// 系统调用编号
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

// 通用系统调用接口
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!("ecall",
            in("x10") args[0],
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
            lateout("x10") ret
        );
    }
    ret
}

// 封装系统调用
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

// 实现标准输出
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

// 打印接口
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

// 打印宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

// Panic处理
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap_or(&format_args!(""))
        );
    } else {
        println!("Panicked: {}", info.message().unwrap_or(&format_args!("")));
    }
    sys_exit(-1);
    loop {}
}

// 程序入口
#[no_mangle]
extern "C" fn _start() {
    println!("Hello, RISC-V OS World!");
    println!("System call demo:");
    
    // 测试格式化输出
    println!("Formatted output: 0x{:x}", 42);
    
    sys_exit(0);  // 正常退出
}