use super::eei::EEI;
use u32;
use std::error;
use std::fmt;
use std::convert::TryFrom;
use std::ops::Add;
use log::{error, debug, trace};

const LB: u8 = 0x0;
const LH: u8 = 0x1;
const LW: u8 = 0x2; 
const LBU: u8 = 0x3;
const LHU: u8 = 0x4;
const SB: u8 = 0x0;
const SH: u8 = 0x1;
const SW: u8 = 0x2; 
const ADDI: u8 = 0x0;
const SLTI: u8 = 0x2;
const SLTIU: u8 = 0x3;
const XORI: u8 = 0x4;
const ORI: u8 = 0x6;
const ANDI: u8 = 0x7;

#[derive(Debug, Clone)]
enum Error {
    Funct3Exception(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Funct3Exception(funct3) =>
                write!(f, "Invalid funct3 {}", funct3),
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
    OpImm = 0x13,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::Load as u8 => Ok(Self::Load),
            x if x == Self::Store as u8 => Ok(Self::Store),
            x if x == Self::OpImm as u8 => Ok(Self::OpImm),
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
            Ok(Opcode::OpImm) => result = self.op_imm(funct3, rd, rs1, imm_i),
            _ => error!("Unhandled/invalid instruction"),
        }
        // TODO: Exception handling
    }

    fn load(&mut self, funct3: u8, rd: u8, rs1: u8, imm: u16) 
    -> Result<(), Box<dyn error::Error>> {
        let addr = self.imm_addr(rs1, imm);
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
            LB => self.x[rd as usize] = temp as u8 as i8 as i32 as u32,
            LH => self.x[rd as usize] = temp as u16 as i16 as i32 as u32,
            LW => self.x[rd as usize] = temp,
            LBU => self.x[rd as usize] = temp & 0xff,
            LHU => self.x[rd as usize] = temp & 0xffff,
            _ => {
                error!("Invalid funct3");
                return Err(Box::new(Error::Funct3Exception(funct3)))
            },
        }

        Ok(())
    }    

    fn store(&mut self, funct3: u8, rs1: u8, rs2: u8, imm: u16)
    -> Result<(), Box<dyn error::Error>> {
        let addr = self.imm_addr(rs1, imm);
        match funct3 {
            SB => return self.eei.write8(self.x[rs2 as usize] as u8, addr),
            SH => return self.eei.write16(self.x[rs2 as usize] as u16, addr),
            SW => return self.eei.write32(self.x[rs2 as usize], addr),
            _ => return Err(Box::new(Error::Funct3Exception(funct3))),
        }
    }

    fn op_imm(&mut self, funct3: u8, rd: u8, rs1: u8, imm: u16)
    -> Result<(), Box<dyn error::Error>> {
        let val = self.x[rs1 as usize];
        let immi = imm as i16 as i32;
        let rd: &mut u32 = &mut self.x[rd as usize];
        match funct3 {
            ADDI => *rd = val.wrapping_add((imm as i16) as u32),
            SLTI => if (val as i32) < immi { *rd = 1; } else { *rd = 0; },
            SLTIU => if val < (immi as u32) { *rd = 1; } else { *rd = 0; },
            XORI => *rd = val ^ (immi as u32),
            ORI => *rd = val | (immi as u32),
            ANDI => *rd = val & (immi as u32),
            _ => return Err(Box::new(Error::Funct3Exception(funct3))),
        }
        Ok(())
    }

    fn imm_addr(&self, rs1: u8, imm: u16) -> u32 {
        self.x[rs1 as usize].wrapping_add(imm as i16 as u32)
        
    }

    fn bits(&self, val: u32, end: u8, start: u8) -> u32 {
        let mut mask: u32 = 0x1;
        for _ in 0..(end - start) {
            mask = (mask << 1) | 1;
        }
        (val >> start) & mask
    }
}
