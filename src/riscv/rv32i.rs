use super::eei::ExecutionEnvironmentInterface;
use log::{error, warn, debug, trace};

#[derive(Debug)]
pub struct Rv32ICore {
    eei: Box<dyn ExecutionEnvironmentInterface>,
    pc: u32,
    x0: u32, x1: u32, x2: u32, x3: u32, x4: u32, x5: u32, x6: u32, x7: u32,
    x8: u32, x9: u32, x10: u32, x11: u32, x12: u32, x13: u32, x14: u32, 
    x15: u32, x16: u32, x17: u32, x18: u32, x19: u32, x20: u32, x21: u32,
    x22: u32, x23: u32, x24: u32, x25: u32, x26: u32, x27: u32, x28: u32,
    x29: u32, x30: u32, x31: u32
}

impl Rv32ICore {
    pub fn new(eei: Box<dyn ExecutionEnvironmentInterface>) -> Self {
        Self {
            eei,
            pc: 0,
            x0: 0, x1: 0, x2: 0, x3: 0, x4: 0, x5: 0, x6: 0, x7: 0, x8: 0, 
            x9: 0, x10: 0, x11: 0, x12: 0, x13: 0, x14: 0, x15: 0, x16: 0, 
            x17: 0, x18: 0, x19: 0, x20: 0, x21: 0, x22: 0, x23: 0, x24: 0, 
            x25: 0, x26: 0, x27: 0, x28: 0, x29: 0, x30: 0, x31: 0,
        }
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
        trace!("After step 0x{:?}", &self);
    }

    fn decode_and_execute(&mut self, instr: u32) {
        trace!("Decode instruction 0x{:x}", instr);

        // 32-0
        let opcode: u8 = instr as u8 & 0b1111111; // 6-0
        let rd_imm: u8 = (instr >> 7) as u8 & 0b11111; // 11-7
        let funct3: u8 = (instr >> 12) as u8 & 0b111; // 14-12
        let rs1: u8 = (instr >> 15) as u8 & 0b11111; // 19-15
        let rs2: u8 = (instr >> 20) as u8 & 0b11111; // 24-20
        let imm_31_20: u16 = (instr >> 20) as u16 & 0b11111111111 // 31-20
;
        trace!(
            "opcode: 0x{:x} rd: 0x{:x} funct3: 0x{:x}", opcode, rd_imm, funct3);

        // https://github.com/riscv/riscv-opcodes/blob/master/opcodes-rv32i
        match (opcode, funct3) {
            // Load
            (0x3, 0x0) => self.LB(rd_imm, rs1, imm_31_20),
            (0x3, 0x1) => trace!("LH"),
            (0x3, 0x2) => trace!("LW"),
            (0x3, 0x3) => trace!("LBU"),
            (0x3, 0x4) => trace!("LHU"), 
            // Store
            (0x23, 0x0) => trace!("SB"),
            (0x23, 0x1) => trace!("SH"),
            (0x23, 0x2) => trace!("SW"),
            // Other
            (_, _) => warn!("Unhandled/invalid instruction"),
        }
    }

    fn LB(&mut self, rd: u8, rs1: u8, imm: u16) {
        trace!("LB");
    }
}
