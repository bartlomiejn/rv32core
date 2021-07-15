use log::{debug, trace};
use super::bus::BusInterface;

#[derive(Debug)]
pub struct Rv32ICore {
    bus: Box<dyn BusInterface>,
    pc: u32,
    x0: u32, x1: u32, x2: u32, x3: u32, x4: u32, x5: u32, x6: u32, x7: u32,
    x8: u32, x9: u32, x10: u32, x11: u32, x12: u32, x13: u32, x14: u32, 
    x15: u32, x16: u32, x17: u32, x18: u32, x19: u32, x20: u32, x21: u32,
    x22: u32, x23: u32, x24: u32, x25: u32, x26: u32, x27: u32, x28: u32,
    x29: u32, x30: u32, x31: u32
}

impl Rv32ICore {
    pub fn new(bus: Box<dyn BusInterface>) -> Self {
        Self {
            bus: bus,
            pc: 0,
            x0: 0, x1: 0, x2: 0, x3: 0, x4: 0, x5: 0, x6: 0, x7: 0, x8: 0, 
            x9: 0, x10: 0, x11: 0, x12: 0, x13: 0, x14: 0, x15: 0, x16: 0, 
            x17: 0, x18: 0, x19: 0, x20: 0, x21: 0, x22: 0, x23: 0, x24: 0, 
            x25: 0, x26: 0, x27: 0, x28: 0, x29: 0, x30: 0, x31: 0,
        }
    }

    pub fn reset(&mut self) {
        trace!("Reset: {:?}", &self);
    }

    pub fn step(&mut self) {
        let instr = self.bus.read(self.pc);

        self.pc += 1;

        trace!("After step {:?}", &self);
    }

    fn decode(&mut self, instr: u32) {

    }
}
