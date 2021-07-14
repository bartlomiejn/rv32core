mod riscv;
use log::debug;
use env_logger;

fn main() {
    env_logger::init();
    let bus = Box::new(riscv::Bus{});
    let core = riscv::Rv32ICore::new(bus);
    core.reset();
}
