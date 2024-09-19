use core::fmt;

use crate::drivers::uart::{Uart, UART0_BASE};
use crate::sync::SpinLock;

static UART: SpinLock<Uart> = SpinLock::new(Uart::new(UART0_BASE));

pub fn init() {
    UART.lock().init();
}

pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    let _ = UART.lock().write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*)
    };
}

