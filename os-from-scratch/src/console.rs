use crate::uart::Uart;
use core::fmt::{self, Write};

struct Stdio;

static mut CONSOLE_UART: Uart = Uart {
    base_addr: 0x1000_0000,
};

// fn get_console_uart() -> Uart {
//     Uart::new(0x1000_0000)
// }

pub fn init() {
    unsafe { CONSOLE_UART.init() }
}

#[allow(unused)]
pub fn read_byte() -> Option<u8> {
    unsafe { CONSOLE_UART.get_byte() }
}

impl Write for Stdio {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { CONSOLE_UART.write_str(s) }
    }
}

pub fn print(args: fmt::Arguments) {
    Stdio.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => {{
//         $crate::console::print(format_args!($($arg)*));
//     }};
// }

// #[macro_export]
// macro_rules! println {
//     () => {
//         $crate::print!("\n")
//     };
//     ($($arg:tt)*) => {{
//         $crate::console::print(format_args!($($arg)*));
//         $crate::console::print(format_args!("\r\n"));
//     }};
// }
