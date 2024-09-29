mod chip8;
use std::time::Duration;

use chip8::disasm;

fn main() {
    let rom = include_bytes!("../test_opcode.ch8");
    let options = chip8::options::Chip8Options::new(10, rom.to_vec());
    let mut cpu = chip8::Chip8::new_with_rom(options);
    while cpu.running {
        cpu.cycle();
        std::thread::sleep(Duration::from_millis(1));
    }
}
