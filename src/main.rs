mod chip8;

use clap::Parser;

fn main() {
    let mut options = chip8::options::Chip8Options::parse();
    options.build();
    let mut cpu = chip8::Chip8::new_with_rom(options);
    while cpu.running {
        cpu.cycle();
        // std::thread::sleep(Duration::from_millis(1));
    }
}
