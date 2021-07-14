use super::bus;

pub struct Rv32ICore {
    bus: Box<dyn bus::SystemBus32>,
    pc: i32,
    x0: i32, x1: i32, x2: i32, x3: i32, x4: i32, x5: i32, x6: i32, x7: i32,
    x8: i32, x9: i32, x10: i32, x11: i32, x12: i32, x13: i32, x14: i32, 
    x15: i32, x16: i32, x17: i32, x18: i32, x19: i32, x20: i32, x21: i32,
    x22: i32, x23: i32, x24: i32, x25: i32, x26: i32, x27: i32, x28: i32,
    x29: i32, x30: i32, x31: i32
}

impl Rv32ICore {
    pub fn new(bus: Box<dyn bus::SystemBus32>) -> Rv32ICore {
        Rv32ICore {
            bus: bus,
            pc: 0,
            x0: 0, x1: 0, x2: 0, x3: 0, x4: 0, x5: 0, x6: 0, x7: 0, x8: 0, 
            x9: 0, x10: 0, x11: 0, x12: 0, x13: 0, x14: 0, x15: 0, x16: 0, 
            x17: 0, x18: 0, x19: 0, x20: 0, x21: 0, x22: 0, x23: 0, x24: 0, 
            x25: 0, x26: 0, x27: 0, x28: 0, x29: 0, x30: 0, x31: 0
        }
    }

    pub fn reset(&self) {

    }

    pub fn step(&self) {

    }
}
