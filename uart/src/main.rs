#![no_main]
#![no_std]
mod entry;
mod uart;

fn main() -> ! {
    entry::zero_bss();
    uart::Uart::new(0x1000_0000).init();
    println!("Hello, world!");
    loop {
        if let Some(c_ascii) = uart::Uart::new(0x1000_0000).get_byte() {
            println!("read a ascii: {}", c_ascii);
        }
    }
}


#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {{
        use core::fmt::Write;
        let _ = (0x1000_0000 as uart::Uart).write_fmt(format_args!($fmt $(, $($arg)+)?));
    }}
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {{
        use core::fmt::Write;
        use crate::uart::Uart;
        let _ = Uart::new(0x1000_0000).write_fmt(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }}
}
