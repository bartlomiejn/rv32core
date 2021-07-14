pub trait SystemBus32 {
    fn read(&self, addr: i32) -> i32;
    fn write(&self, addr: i32);
}

pub struct Bus {}

impl SystemBus32 for Bus {
    fn read(&self, addr: i32) -> i32 {
        return 0;
    }

    fn write(&self, addr: i32) {

    }
}
