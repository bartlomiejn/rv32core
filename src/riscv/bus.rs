use log::trace;
use core::fmt;

pub trait BusInterface {
    fn read(&self, addr: u32) -> u32;
    fn write(&self, val: u32, addr: u32);
}

impl fmt::Debug for dyn BusInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = self as *const Self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

pub struct SystemBus {
    ram: [u8; 1024 * 1024],
}

impl SystemBus {
    pub fn new() -> Self {
        Self { ram: [0; 1024 * 1024], }
    }
}

impl BusInterface for SystemBus {
    fn read(&self, addr: u32) -> u32 {
        trace!("read addr: {}", addr);
        return 0;
    }

    fn write(&self, val: u32, addr: u32) {
        trace!("write value: {} addr: {}", val, addr);
    }
}
