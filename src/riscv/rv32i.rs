use super::eei::EEI;
use u32;
use std::error;
use std::fmt;
use log::{error, debug, trace};

// Opcode
const LOAD: u8 = 0x3;
const STORE: u8 = 0x23;
const OPIMM: u8 = 0x13;
const OP: u8 = 0x33;
const JAL: u8 = 0x6f;
const JALR: u8 = 0x67;
const BRANCH: u8 = 0x63;
const LUI: u8 = 0x37;
const AUIPC: u8 = 0x17;
const FENCE: u8 = 0xf;
const SYSTEM: u8 = 0x73;

// Funct3
// LOAD
const LB: u8 = 0x0;
const LH: u8 = 0x1;
const LW: u8 = 0x2; 
const LBU: u8 = 0x3;
const LHU: u8 = 0x4;

// STORE
const SB: u8 = 0x0;
const SH: u8 = 0x1;
const SW: u8 = 0x2; 

// OPIMM
const ADDI: u8 = 0x0;
const SLLI: u8 = 0x1;
const SLTI: u8 = 0x2;
const SLTIU: u8 = 0x3;
const XORI: u8 = 0x4;
const SRLI_SRAI: u8 = 0x5;
const ORI: u8 = 0x6;
const ANDI: u8 = 0x7;

// OP
const ADD: u8 = 0x0;
const SUB: u8 = 0x0;
const SLL: u8 = 0x1;
const SLT: u8 = 0x2;
const SLTU: u8 = 0x3;
const XOR: u8 = 0x4;
const SRL: u8 = 0x5;
const SRA: u8 = 0x5;
const OR: u8 = 0x6;
const AND: u8 = 0x7;

// BRANCH
const BEQ: u8 = 0x0;
const BNE: u8 = 0x1;
const BLT: u8 = 0x4;
const BGE: u8 = 0x5;
const BLTU: u8 = 0x6;
const BGEU: u8 = 0x7;

// SYSTEM
const PRIV: u8 = 0x0;

// Funct12
const ECALL: u16 = 0x0;
const EBREAK: u16 = 0x1;

#[derive(Debug, Clone)]
enum Error {
    InvalidOpcode(u8),
    InvalidFunct3(u8),
    InvalidFunct12(u16),
    InstrAddrMisaligned(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidOpcode(opcode) =>
                write!(f, "Invalid opcode {}", opcode),
            Error::InvalidFunct3(funct3) =>
                write!(f, "Invalid funct3 {}", funct3),
            Error::InvalidFunct12(funct12) =>
                write!(f, "Invalid funct12 {}", funct12),
            Error::InstrAddrMisaligned(addr) =>
                write!(f, "Instruction address misaligned 0x{:08x}", addr),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self { _ => None, }
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
        self.pc = 0;
        self.x = [0; 32];
    }

    pub fn set_pc(&mut self, pc: u32) {
        debug!("Set pc: {}", pc);
        self.pc = pc;
    }

    pub fn step(&mut self) {
        let _ = self.eei
            .read32(self.pc)
            .and_then(|instr| self.decode_and_execute(instr))
            .or_else(|err| -> Result<(), Box<dyn error::Error>> {
                error!("Error: {}", err);
                Ok(())
            });
        self.pc += 4;
        trace!("State {:?}", &self);
    }

    fn decode_and_execute(&mut self, instr: u32)
    -> Result<(), Box<dyn error::Error>> {
        let opcode: u8 = self.bits(instr, 6, 0) as u8;
        let funct3: u8 = self.bits(instr, 14, 12) as u8;
        let funct7: u8 = self.bits(instr, 31, 25) as u8;

        let rd: u8 = self.bits(instr, 11, 7) as u8; 
        let rs1: u8 = self.bits(instr, 19, 15) as u8;
        let rs2: u8 = self.bits(instr, 24, 20) as u8;

        // TODO: Verify immediate values with riscv-spec
        let imm_i: u16 = self.bits(instr, 31, 20) as u16; // 12
        // TODO: Is S immediate correct?
        let imm_s: u16 = 
            ((self.bits(instr, 31, 25) as u16) << 5)
            | self.bits(instr, 11, 7) as u16;
        let imm_u: u32 = self.bits(instr, 31, 12) << 12;
        let imm_b: u32 = 
            self.bits(instr, 11, 8) << 1 
            | self.bits(instr, 30, 25) << 5
            | self.bits(instr, 7, 7) << 11
            | self.bits(instr, 31, 31) << 12; // 13
        let imm_j: u32 = 
            self.bits(instr, 30, 21) << 1
            | self.bits(instr, 20, 20) << 11
            | self.bits(instr, 19, 12) << 12
            | self.bits(instr, 31, 31) << 20; // 21

        trace!(
            "Decode instruction 0x{:08x} opcode: 0x{:02x} funct3: 0x{:02x} 
            funct7: 0x{:02x} rd: x{} rs1: x{} rs2: x{}", instr, opcode, funct3, 
            funct7, rd, rs1, rs2);

        // https://github.com/riscv/riscv-opcodes/blob/master/opcodes-rv32i
        match opcode {
            LOAD => return self.load(funct3, rd, rs1, imm_i),
            STORE => return self.store(funct3, rs1, rs2, imm_s),
            OPIMM => return self.op_imm(funct3, rd, rs1, imm_i),
            LUI => return self.lui(rd, imm_u),
            AUIPC => return self.auipc(rd, imm_u),
            OP => return self.op(funct3, rd, rs1, rs2, funct7),
            JAL => return self.jal(rd, imm_j),
            JALR => return self.jalr(rd, rs1, imm_i),
            BRANCH => return self.branch(funct3, rs1, rs2, imm_b),
            FENCE => return self.fence(funct3, rd, rs1, instr),
            SYSTEM => return self.system(funct3, imm_i, rd, rs1),
            _ => return Err(Box::new(Error::InvalidOpcode(opcode))),
        }
    }

    fn load(&mut self, funct3: u8, rd: u8, rs1: u8, imm: u16) 
    -> Result<(), Box<dyn error::Error>> {
        let addr = self.imm_addr(rs1, imm);
        let temp: u32;

        trace!("load funct3: {} rd: x{} rs1: x{} imm: {}", funct3, rd, rs1, 
            imm);

        match self.eei.read32(addr) {
            Ok(val) => temp = val,
            Err(err) => return Err(err),
        }
        match funct3 {
            LB => self.x[rd as usize] = temp as u8 as i8 as i32 as u32,
            LH => self.x[rd as usize] = temp as u16 as i16 as i32 as u32,
            LW => self.x[rd as usize] = temp,
            LBU => self.x[rd as usize] = temp & 0xff,
            LHU => self.x[rd as usize] = temp & 0xffff,
            _ => return Err(Box::new(Error::InvalidFunct3(funct3))),
        }

        Ok(())
    }    

    fn store(&mut self, funct3: u8, rs1: u8, rs2: u8, imm: u16)
    -> Result<(), Box<dyn error::Error>> {
        let addr = self.imm_addr(rs1, imm);
        match funct3 {
            SB => 
                return self.eei.write8(self.x[rs2 as usize] as u8, addr),
            SH => 
                return self.eei.write16(self.x[rs2 as usize] as u16, addr),
            SW => return self.eei.write32(self.x[rs2 as usize], addr),
            _ => return Err(Box::new(Error::InvalidFunct3(funct3))),
        }
    }

    fn op_imm(&mut self, funct3: u8, rd: u8, rs1: u8, imm: u16)
    -> Result<(), Box<dyn error::Error>> {
        let rs1 = self.x[rs1 as usize];
        let immi = imm as i16 as i32;
        let rd: &mut u32 = &mut self.x[rd as usize];
        match funct3 {
            ADDI => *rd = rs1.wrapping_add((imm as i16) as u32),
            SLLI => *rd = rs1 << (imm as u32 & 0x1f),
            SLTI => if (rs1 as i32) < immi { *rd = 1; } else { *rd = 0; },
            SLTIU => if rs1 < (immi as u32) { *rd = 1; } else { *rd = 0; },
            XORI => *rd = rs1 ^ (immi as u32),
            SRLI_SRAI if imm & 0x400 == 0 => // SRLI
                *rd = rs1 >> (imm as u32 & 0x1f),
            SRLI_SRAI if imm & 0x400 != 0 => // SRAI
                *rd = (rs1 as i32 >> (imm & 0x1f)) as u32,
            ORI => *rd = rs1 | (immi as u32),
            ANDI => *rd = rs1 & (immi as u32),
            _ => return Err(Box::new(Error::InvalidFunct3(funct3))),
        }
        Ok(())
    }

    fn lui(&mut self, rd: u8, imm: u32) -> Result<(), Box<dyn error::Error>> {
        self.x[rd as usize] = imm << 12;
        Ok(())
    }

    fn auipc(&mut self, rd: u8, imm: u32) -> Result<(), Box<dyn error::Error>> {
        self.x[rd as usize] = self.pc + (imm << 12);
        Ok(())
    }

    fn op(&mut self, funct3: u8, rd: u8, rs1: u8, rs2: u8, funct7: u8)
    -> Result<(), Box<dyn error::Error>> {
        let rs1 = self.x[rs1 as usize];
        let rs2 = self.x[rs2 as usize];
        let rd = &mut self.x[rd as usize];
        match (funct3, funct7) {
            (ADD, 0x0) => *rd = rs1.wrapping_add(rs2),
            (SUB, 0x32) => *rd = rs1.wrapping_sub(rs2),
            (SLL, 0x0) => *rd = rs1 << (rs2 & 0x1f),
            (SLT, 0x0) => 
                if (rs1 as i32) < (rs2 as i32) { *rd = 1; }
                else { *rd = 0; },
            (SLTU, 0x0) => 
                if rs1 < rs2 { *rd = 1; } 
                else { *rd = 0; },
                // TODO: check whether SLTU rd, x0, rs2 case works correctly
            (XOR, 0x0) => *rd = rs1 ^ rs2,
            (SRL, 0x0) => *rd = rs1 >> (rs2 & 0x1f),
            (SRA, 0x32) => *rd = (rs1 as i32 >> (rs2 & 0x1f)) as u32,
            (OR, 0x0) => *rd = rs1 | rs2,
            (AND, 0x0) => *rd = rs1 & rs2,
            (_, _) => return Err(Box::new(Error::InvalidFunct3(funct3))),
        }
        Ok(())
    }

    fn jal(&mut self, rd: u8, imm: u32) -> Result<(), Box<dyn error::Error>> {
        let target = self.pc.wrapping_add(self.sext(imm, 21));
        if target % 4 != 0 {  
            return Err(Box::new(Error::InstrAddrMisaligned(target)))
        }
        self.x[rd as usize] = self.pc + 4;
        self.pc = target;
        Ok(())
    }

    fn jalr(&mut self, rd: u8, rs1: u8, imm: u16)
    -> Result<(), Box<dyn error::Error>> {
        let target = self
            .x[rs1 as usize]
            .wrapping_add(self.sext(imm as u32, 12)) 
            & 0xfffffffe;
        if target % 4 != 0 {
            return Err(Box::new(Error::InstrAddrMisaligned(target)))
        }
        self.x[rd as usize] = self.pc + 4;
        self.pc = target;
        Ok(())
    }

    fn branch(&mut self, funct3: u8, rs1: u8, rs2: u8, imm: u32) 
    -> Result<(), Box<dyn error::Error>> {
        let target = self.pc.wrapping_add(self.sext(imm, 12));
        match funct3 {
            BEQ => if self.x[rs1 as usize] == self.x[rs2 as usize] { 
                self.pc = target; 
            },
            BNE => if self.x[rs1 as usize] != self.x[rs2 as usize] { 
                self.pc = target; 
            },
            BLT => 
                if (self.x[rs1 as usize] as i32) 
                <  (self.x[rs2 as usize] as i32) { 
                    self.pc = target; 
                },
            BGE => 
                if (self.x[rs1 as usize] as i32) 
                >= (self.x[rs2 as usize] as i32) { 
                    self.pc = target; 
                },
            BLTU => if self.x[rs1 as usize] < self.x[rs2 as usize] { 
                self.pc = target; 
            },
            BGEU => if self.x[rs1 as usize] >= self.x[rs2 as usize] { 
                self.pc = target; 
            },
            _ => return Err(Box::new(Error::InvalidFunct3(funct3))),
        }
        Ok(())
    }

    fn fence(&mut self, funct3: u8, rd: u8, rs1: u8, instr: u32)
    -> Result<(), Box<dyn error::Error>> {
        trace!("FENCE opcode called");
        Ok(())
    }

    fn system(&mut self, funct3: u8, funct12: u16, rd: u8, rs1: u8)
    -> Result<(), Box<dyn error::Error>> {
        match funct12 {
            ECALL => self.eei.ecall(),
            EBREAK => self.eei.ebreak(),
            _ => return Err(Box::new(Error::InvalidFunct12(funct12))),
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

    fn sext(&self, val: u32, width: u8) -> u32 {
        let sign = val >> (width - 1);
        let mut res = 0x0u32;
        if sign == 0 {
            res |= val;
        } else {
            let mut mask = 0x0u32;
            for _ in 0..width {
                mask = (mask << 1) | 1
            }
            res = (!mask) | val;
        }
        res
    }
}
