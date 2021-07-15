mod riscv;
use i64;
use std::env;
use std::str;
use std::process::exit;
use std::fs;
use std::fs::{File};
use std::io::Read;
use log::{info};
use env_logger;

const VAR_BIN: &str = "RV32I_BIN";
const VAR_OFFSET: &str = "RV32I_OFFSET";
const VAR_LEN: &str = "RV32I_LEN";

fn hexstring_to_i64(string: &String) -> Option<i64> {
    let offset: Option<i64>;

    // Remove 0x 0X prefixes if any.
    let stripped_str = string
        .strip_prefix("0x")
        .or_else(|| string.strip_prefix("0X"));

    // For both no prefixes and prefix removed case convert to an i64.
    match stripped_str {
        Some(str) => offset = i64::from_str_radix(str.trim(), 16).ok(),
        None => offset = i64::from_str_radix(&string.trim(), 16).ok(),
    }

    offset
}

fn load_binary(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename)
        .expect(&format!("No file found at {}.", filename));
    let metadata = fs::metadata(&filename)
        .expect("Unable to read binary file metadata.");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow.");

    buffer
}

fn main() {
    env_logger::init();

    let filename = env::var(VAR_BIN)
        .expect("Supply rv32i binary file to load as RV32I_BIN variable.");
    info!("Binary file to load: {}", filename);

    let offset = env::var(VAR_OFFSET)
        .expect("Supply binary file text section offset as hex as 
            RV32I_OFFSET.");
    info!("Text offset: {}", offset);
    let offset = hexstring_to_i64(&offset)
        .expect("Couldn't parse text section offset as hex.");

    let len = env::var(VAR_LEN)
        .expect("Supply binary file text section length as hex as 
            RV32I_LEN.");
    info!("Text length: {}", len);
    let len = hexstring_to_i64(&len)
        .expect("Couldn't parse text section length as hex.");

    let binary = load_binary(&filename);

    info!("Binary loaded.");

    let bus = Box::new(riscv::SystemBus::new());
    let mut core = riscv::Rv32ICore::new(bus);
    core.reset();
}
