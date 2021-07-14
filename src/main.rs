mod riscv;

fn main() {
    let bus = Box::new(riscv::Bus{});
    let core = riscv::Rv32ICore::new(bus);
    core.reset();
}
