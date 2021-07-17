use super::eei::ExecutionEnvironmentInterface;
use log::{error, warn, debug, trace};

#[derive(Debug)]
pub struct Rv32I {
    eei: Box<dyn ExecutionEnvironmentInterface>,
    pc: u32,
    x: [u32; 32],
}

impl Rv32I {
    pub fn new(eei: Box<dyn ExecutionEnvironmentInterface>) -> Self {
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
        trace!("After step 0x{:?}", &self);
    }

    fn decode_and_execute(&mut self, instr: u32) {
        trace!("Decode instruction 0x{:x}", instr);

        // 32-0
        let opcode: u8 = instr as u8 & 0b1111111; // 6-0
        let rd: u8 = (instr >> 7) as u8 & 0b11111; // 11-7
        let funct3: u8 = (instr >> 12) as u8 & 0b111; // 14-12
        let rs1: u8 = (instr >> 15) as u8 & 0b11111; // 19-15
        let rs2: u8 = (instr >> 20) as u8 & 0b11111; // 24-20
        let imm_i: u16 = (instr >> 20) as u16 & 0b11111111111 // 31-20
;
        trace!(
            "opcode: 0x{:x} rd: 0x{:x} funct3: 0x{:x}", opcode, rd, funct3);

        // https://github.com/riscv/riscv-opcodes/blob/master/opcodes-rv32i
        match (opcode, funct3) {
            // Load
            (0x3, 0x0) => self.LB(rd, rs1, imm_i),
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

    // Load and store instructions transfer a value between the registers and memory. Loads are encoded in the I-type format and stores are S-type. The effective address is obtained by adding register rs1 to the sign-extended 12-bit offset. Loads copy a value from memory to register rd. Stores copy the value in register rs2 to memory.
    fn LB(&mut self, rd: u8, rs1: u8, imm: u16) {
        trace!("LB rd: x{} rs1: x{} imm: {}", rd, rs1, imm);
        let base = self.x[rs1 as usize];
        let offset = (imm as i16) as i32;
        // self.eei.read32(addr);
    }
}
