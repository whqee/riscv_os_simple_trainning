use core::fmt::{Write, self};

use crate::sbi::sbi_debug_console_write_byte_fast;

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c_char in s.as_bytes() {
            sbi_debug_console_write_byte_fast(*c_char)
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {{
        use core::fmt::Write;
        let _ = crate::console::Console.write_fmt(format_args!($fmt $(, $($arg)+)?));
    }}
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {{
        use crate::console::Console;
        use core::fmt::Write;
        let _ = Console.write_fmt(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }}
}
