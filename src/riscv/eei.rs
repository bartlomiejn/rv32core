use std::mem::transmute;
use std::error;
use core::fmt;
use log::{debug, trace};

pub trait EEI {
    fn read32(&self, addr: u32) -> Result<u32, Box<dyn error::Error>>;
    fn write32(&mut self, val: u32, addr: u32) 
    -> Result<(), Box<dyn error::Error>>;
    fn write16(&mut self, val: u16, addr: u32)
    -> Result<(), Box<dyn error::Error>>;
    fn write8(&mut self, val: u8, addr: u32)
    -> Result<(), Box<dyn error::Error>>;
    fn ebreak(&self);
    fn ecall(&self);
}

impl fmt::Debug for dyn EEI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = self as *const Self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

pub struct SoftwareInterface {
    ram: [u8; 1024 * 1024],
}

impl SoftwareInterface {
    pub fn new() -> Self {
        Self { ram: [0; 1024 * 1024], }
    }

    pub fn load(&mut self, data: &Vec<u8>, addr: u32) {
        debug!("Load byte array to addr: 0x{:x}", addr);
        for i in 0..data.len() {
            self.ram[addr as usize + i] = data[i];
        }
    }
}

impl EEI for SoftwareInterface {
    fn read32(&self, addr: u32) -> Result<u32, Box<dyn error::Error>> {
        let val = u32::from_be_bytes([
            self.ram[addr as usize],
            self.ram[addr as usize + 1],
            self.ram[addr as usize + 2],
            self.ram[addr as usize + 3]
        ]);
        trace!("Read32 addr: 0x{:x} value: 0x{:x}", addr, val);
        Ok(val)
    }

    fn write32(&mut self, val: u32, addr: u32) 
    -> Result<(), Box<dyn error::Error>> {
        let bytes: [u8; 4] = unsafe { transmute(val.to_be()) };
        trace!(
            "Write32 value: 0x{:x} addr: 0x{:x} bytes: {:?}", val, addr, bytes);
        for i in 0..bytes.len() {
            self.ram[addr as usize + i] = bytes[i];    
        }
        Ok(())
    }

    fn write16(&mut self, val: u16, addr: u32) 
    -> Result<(), Box<dyn error::Error>> {
        let bytes: [u8; 2] = unsafe { transmute(val.to_be()) };
        trace!(
            "Write16 value: 0x{:x} addr: 0x{:x} bytes: {:?}", val, addr, bytes);
        for i in 0..bytes.len() {
            self.ram[addr as usize + i] = bytes[i];    
        }
        Ok(())
    }

    fn write8(&mut self, val: u8, addr: u32) 
    -> Result<(), Box<dyn error::Error>> {
        trace!(
            "Write8 value: 0x{:x} addr: 0x{:x}", val, addr);
        self.ram[addr as usize] = val;
        Ok(())
    }

    fn ebreak(&self) {
        trace!("ebreak called");
    }

    fn ecall(&self) {
        trace!("ecall called");
    }
}
