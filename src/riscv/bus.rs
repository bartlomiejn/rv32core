use log::trace;
use core::fmt;

pub trait SystemBus32 {
    fn read(&self, addr: i32) -> i32;
    fn write(&self, val: i32, addr: i32);
}

impl fmt::Debug for dyn SystemBus32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = self as *const Self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

pub struct Bus {}

impl SystemBus32 for Bus {
    fn read(&self, addr: i32) -> i32 {
        trace!("read addr: {}", addr);
        return 0;
    }

    fn write(&self, val: i32, addr: i32) {
        trace!("write value: {} addr: {}", val, addr);
    }
}
