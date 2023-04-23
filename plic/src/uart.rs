use core::fmt::{self, Write};

// QEMU RISC-V UART base addr: 0x10000000, and 16550 ref: https://www.lammertbies.nl/comm/info/serial-uart
#[derive(Debug)]
pub struct Uart {
    pub(crate) base_addr: usize,
}

impl Uart {
    #[allow(unused)]
    pub fn new(base_addr: usize) -> Self {
        Self { base_addr }
    }

    pub fn init(&self) {
        let ptr = self.base_addr as *mut u8;
        unsafe {
            // Write FCR reg to enable FIFO's and clear FIFO
            ptr.add(2).write_volatile(0x07);

            // Write LCR to turn on DLAB
            ptr.add(3).write_volatile(0x80);

            // Set Baudrate 115200
            ptr.add(0).write_volatile((115200 / 9600) as u8);
            ptr.add(1).write_volatile(0);

            // Write LCR to turn off DLAB and set WLEN8
            ptr.add(3).write_volatile(0x03);
            ptr.add(4).write_volatile(0x0);
        }
    }

    pub fn put_byte(&self, c: u8) {
        let ptr = self.base_addr as *mut u8;
        unsafe {
            // check if ready
            while ptr.add(5).read_volatile() & 0x20 == 0 {}
            ptr.add(0).write_volatile(c);
        }
    }

    pub fn get_byte(&self) -> Option<u8> {
        let ptr = self.base_addr as *mut u8;
        unsafe {
            // check if there's data to be read
            if ptr.add(5).read_volatile() & 1 != 0 {
                Some(ptr.add(0).read_volatile())
            } else {
                None
            }
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c_char in s.as_bytes() {
            self.put_byte(*c_char)
        }
        Ok(())
    }
}

// impl Drop for Uart {
//     fn drop(&mut self) {
//         crate::println!("Drop for Uart");
//     }
// }
