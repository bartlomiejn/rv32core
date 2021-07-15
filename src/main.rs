mod riscv;
use i64;
use std::env;
use std::str;
use std::process::exit;
use std::num::ParseIntError;
use log::{info, error};
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

// fn load_binary(file: String) -> Vec<u8> {
//     let mut buffer: Vec<u8> = Vec::new();
//     buffer
// }

fn main() {
    env_logger::init();

    let filename = env::var(VAR_BIN);
    if filename.is_err() {
        error!("Supply rv32i binary file to load as RV32I_BIN variable.");
        exit(1);
    }
    let filename = filename.unwrap();
    info!("Binary file to load: {}", filename);

    let offset = env::var(VAR_OFFSET);
    if offset.is_err() {
        error!("Supply binary file text section offset as hex as RV32I_OFFSET.");
        exit(1);
    }
    let offset =  offset.unwrap();
    info!("Binary data offset: {}", offset);
    let offset = hexstring_to_i64(&offset);
    if offset.is_none() {
        error!("Couldn't parse text section offset as hex.");
        exit(1);
    }
    let offset = offset.unwrap();

    let len = env::var(VAR_LEN);
    if len.is_err() {
        error!("Supply binary file text section length as hex as RV32I_LEN.");
        exit(1);
    }
    let len = len.unwrap();
    info!("Binary data offset: {}", offset);
    let len = hexstring_to_i64(&len);
    if len.is_none() {
        error!("Couldn't parse text section length as hex.");
        exit(1);
    }
    let len = len.unwrap();

    let bus = Box::new(riscv::SystemBus::new());
    let mut core = riscv::Rv32ICore::new(bus);
    core.reset();
}
