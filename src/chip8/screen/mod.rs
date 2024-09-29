use std::collections::HashSet;

use sdl2::{
    event::EventPollIterator, keyboard::Keycode, pixels::Color, rect, render::Canvas,
    video::Window, EventPump,
};

pub struct Screen {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    scale_factor: u32,
    width: u32,
    height: u32,
    pixel_buffer: Vec<u8>,
}

impl Screen {
    pub fn new(scale_factor: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let width = 64;
        let height = 32;

        let window = video_subsystem
            .window("CHIP 8", width * scale_factor, height * scale_factor)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();
        let pixel_buffer = vec![0; (width * height) as usize];

        Self {
            canvas,
            event_pump,
            scale_factor,
            width,
            height,
            pixel_buffer,
        }
    }

    pub fn start_screen(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn get_events(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
        self.pixel_buffer.fill(0);
    }

    pub fn is_key_pressed(&mut self, key: u8) -> bool {
        let sdl_key = match key {
            0x0 => Keycode::X,
            0x1 => Keycode::Num1,
            0x2 => Keycode::Num2,
            0x3 => Keycode::Num3,
            0x4 => Keycode::Q,
            0x5 => Keycode::W,
            0x6 => Keycode::E,
            0x7 => Keycode::A,
            0x8 => Keycode::S,
            0x9 => Keycode::D,
            0xA => Keycode::Z,
            0xB => Keycode::C,
            0xC => Keycode::Num4,
            0xD => Keycode::R,
            0xE => Keycode::F,
            0xF => Keycode::V,
            _ => return false,
        };

        let keys: HashSet<Keycode> = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        keys.contains(&sdl_key)
    }

    pub fn draw(&mut self, x_start: u8, y_start: u8, sprite: &[u8]) -> bool {
        let mut collision = false;

        for (y_offset, byte) in sprite.iter().enumerate() {
            for x_offset in 0..8 {
                if (byte & (0x80 >> x_offset)) != 0 {
                    let x = (x_start.wrapping_add(x_offset as u8) % self.width as u8) as usize;
                    let y = (y_start.wrapping_add(y_offset as u8) % self.height as u8) as usize;
                    let index = x + y * self.width as usize;

                    // Check for collision
                    if self.pixel_buffer[index] == 1 {
                        collision = true;
                    }

                    // XOR drawing
                    self.pixel_buffer[index] ^= 1;
                }
            }
        }

        self.update_canvas();
        collision
    }

    fn update_canvas(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        for (i, &pixel) in self.pixel_buffer.iter().enumerate() {
            let x = (i % self.width as usize) as i32;
            let y = (i / self.width as usize) as i32;
            if pixel == 1 {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(
                x * self.scale_factor as i32,
                y * self.scale_factor as i32,
                self.scale_factor,
                self.scale_factor,
            ));
        }

        self.canvas.present();
    }
}
