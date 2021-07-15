mod riscv;
use log::debug;
use env_logger;

fn main() {
    env_logger::init();
    let bus = Box::new(riscv::SystemBus::new());
    let mut core = riscv::Rv32ICore::new(bus);
    core.reset();
}
