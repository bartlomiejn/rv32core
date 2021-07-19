mod riscv;
use u64;
use std::env;
use std::str;
use std::fs;
use std::io::Read;
use log::info;
use env_logger;

const VAR_BIN: &str = "RV32I_BIN";
const VAR_OFFSET: &str = "RV32I_OFFSET";
const VAR_LEN: &str = "RV32I_LEN";

fn hexstring_to_u32(string: &String) -> Option<u32> {
    let offset: Option<u32>;
    let stripped_str = string
        .strip_prefix("0x")
        .or_else(|| string.strip_prefix("0X"));
    match stripped_str {
        Some(str) => offset = u32::from_str_radix(str.trim(), 16).ok(),
        None => offset = u32::from_str_radix(&string.trim(), 16).ok(),
    }
    offset
}

fn load_binary(filename: &String) -> Vec<u8> {
    let mut f = fs::File::open(&filename)
        .expect(&format!("No file found at {}", filename));
    let metadata = fs::metadata(&filename)
        .expect("Unable to read binary file metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Other error");
    buffer
}

fn main() {
    env_logger::init();

    let filename = env::var(VAR_BIN)
        .expect("Supply rv32i binary file to load as RV32I_BIN variable");
    info!("Binary file to load: {}", filename);

    let offset = env::var(VAR_OFFSET)
        .expect("Supply binary file text section offset as hex as 
            RV32I_OFFSET");
    let offset = hexstring_to_u32(&offset)
        .expect("Couldn't parse text section offset as hex");
    info!("Text offset: 0x{:x}", offset);

    let len = env::var(VAR_LEN)
        .expect("Supply binary file text section length as hex as 
            RV32I_LEN");
    let len = hexstring_to_u32(&len)
        .expect("Couldn't parse text section length as hex");
    info!("Text length: 0x{:x}", len);

    let binary = load_binary(&filename);

    let mut eei = Box::new(riscv::SoftwareInterface::new());
    eei.load(&binary, 0x0);

    info!("Binary read & loaded");

    let mut core = riscv::Rv32I::new(eei);

    info!("RV32I core created");

    core.set_pc(offset as u32);
    core.step();

}
