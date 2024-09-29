use clap::Parser;

#[derive(Parser)]
pub struct Chip8Options {
    #[arg(long = "scale", default_value = "10")]
    pub scale_factor: u32,
    #[arg(long = "file", value_hint = clap::ValueHint::FilePath)]
    pub file: String,
    pub rom: Vec<u8>,
}

impl Chip8Options {
    pub fn build(&mut self) {
        use std::fs::File;
        use std::io::Read;
        let mut file = File::open(&self.file).unwrap();
        file.read_to_end(&mut self.rom).unwrap();
    }
}
