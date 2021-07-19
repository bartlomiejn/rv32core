use super::eei::EEI;
use u32;
use std::error;
use std::fmt;
use std::convert::TryFrom;
use log::{error, debug, trace};

#[derive(Debug, Clone)]
enum Error {
    LoadStoreException(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::LoadStoreException(funct3) =>
                write!(f, "Invalid load/store funct3 {}", funct3),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self { _ => None, }
    }
}

enum Opcode {
    Load = 0x3,
    Store = 0x23,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::Load as u8 => Ok(Self::Load),
            x if x == Self::Store as u8 => Ok(Self::Store),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Rv32I {
    eei: Box<dyn EEI>,
    pc: u32,
    x: [u32; 32],
}

impl Rv32I {
    pub fn new(eei: Box<dyn EEI>) -> Self {
        Self { eei, pc: 0, x: [0; 32], }
    }

    pub fn reset(&mut self) {
        debug!("Reset");
    }

    pub fn set_pc(&mut self, pc: u32) {
        debug!("Set pc: {}", pc);
        self.pc = pc;
    }

    pub fn step(&mut self) {
        let instr = self.eei.read32(self.pc);
        match instr {
            Ok(instr) => self.decode_and_execute(instr),
            Err(err) => error!("EEI read error {}", err),
        }
        trace!("State {:?}", &self);
    }

    fn decode_and_execute(&mut self, instr: u32) {
        let opcode: u8 = self.bits(instr, 6, 0) as u8;
        let funct3: u8 = self.bits(instr, 14, 12) as u8;
        let funct7: u8 = self.bits(instr, 31, 25) as u8;

        let rd: u8 = self.bits(instr, 11, 7) as u8; 
        let rs1: u8 = self.bits(instr, 19, 15) as u8;
        let rs2: u8 = self.bits(instr, 24, 20) as u8;

        let imm_i: u16 = self.bits(instr, 31, 20) as u16;
        let imm_s: u16 = 
            ((self.bits(instr, 31, 25) as u16) << 5)
            | self.bits(instr, 11, 7) as u16;
        let imm_u: u32 = self.bits(instr, 31, 12);
        let imm_b: u32 = 
            self.bits(instr, 11, 8) << 1 
            | self.bits(instr, 30, 25) << 5
            | self.bits(instr, 7, 7) << 11
            | self.bits(instr, 31, 31) << 12;
        let imm_j: u32 = 
            self.bits(instr, 30, 21) << 1
            | self.bits(instr, 20, 20) << 11
            | self.bits(instr, 19, 12) << 12
            | self.bits(instr, 31, 31) << 20;

        trace!(
            "Decode instruction 0x{:08x} opcode: 0x{:02x} funct3: 0x{:02x} 
            funct7: 0x{:02x} rd: x{} rs1: x{} rs2: x{}", instr, opcode, funct3, 
            funct7, rd, rs1, rs2);

        // https://github.com/riscv/riscv-opcodes/blob/master/opcodes-rv32i

        let result: Result<(), Box<dyn error::Error>>;
        match Opcode::try_from(opcode) {
            Ok(Opcode::Load) => result = self.load(funct3, rd, rs1, imm_i),
            Ok(Opcode::Store) => result = self.store(funct3, rs1, rs2, imm_s),
            _ => error!("Unhandled/invalid instruction"),
        }

        // TODO: Exception handling
    }

    fn load(&mut self, funct3: u8, rd: u8, rs1: u8, imm: u16) 
    -> Result<(), Box<dyn error::Error>> {
        let base = self.x[rs1 as usize];
        let offset = imm as i16;
        let addr = base.wrapping_add(offset as u32);
        let temp: u32;

        trace!("load funct3: {} rd: x{} rs1: x{} imm: {}", funct3, rd, rs1, 
            imm);

        match self.eei.read32(addr) {
            Ok(val) => temp = val,
            Err(err) => {
                error!("Memory read error addr: 0x{:08x}", addr);
                return Err(err)
            },
        }

        match funct3 {
            0x0 => self.x[rd as usize] = temp as u8 as i8 as i32 as u32,
            0x1 => self.x[rd as usize] = temp as u16 as i16 as i32 as u32,
            0x2 => self.x[rd as usize] = temp,
            0x3 => self.x[rd as usize] = temp & 0xff,
            0x4 => self.x[rd as usize] = temp & 0xffff,
            _ => {
                error!("Invalid funct3");
                return Err(Box::new(Error::LoadStoreException(funct3)))
            },
        }

        Ok(())
    }

    fn store(&mut self, funct3: u8, rs1: u8, rs2: u8, imm: u16)
    -> Result<(), Box<dyn error::Error>> {
        let base = self.x[rs1 as usize];
        let offset = imm as i16;
        let addr = base.wrapping_add(offset as u32);

        match funct3 {
            0x0 => return self.eei.write8(self.x[rs2 as usize] as u8, addr),
            0x1 => return self.eei.write16(self.x[rs2 as usize] as u16, addr),
            0x2 => return self.eei.write32(self.x[rs2 as usize], addr),
            _ => {
                error!("Invalid funct3");
                return Err(Box::new(Error::LoadStoreException(funct3)))
            },
        }
    }

    fn bits(&self, val: u32, end: u8, start: u8) -> u32 {
        let mut mask: u32 = 0x1;
        for _ in 0..(end - start) {
            mask = (mask << 1) | 1;
        }
        (val >> start) & mask
    }
}
