pub struct Chip8Options {
    pub scale_factor: u32,
    pub rom: Vec<u8>,
}

impl Chip8Options {
    pub fn new(scale_factor: u32, rom: Vec<u8>) -> Self {
        Chip8Options { scale_factor, rom }
    }
}
