#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("asm/boot.S"));

#[no_mangle]
pub extern "C" fn kmain(hart_id: usize, dtb: usize) -> ! {
    rustv_os_lab::kernel_main(hart_id, dtb)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustv_os_lab::println!("[panic] {info}");
    loop {
        rustv_os_lab::riscv::wfi();
    }
}

