use sdl2::{event::EventPollIterator, pixels::Color, render::Canvas, video::Window, EventPump};

pub struct Screen {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    scale_factor: u32,
}

impl Screen {
    pub fn new(scale_factor: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("CHIP 8", 64 * scale_factor, 32 * scale_factor)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();
        Screen {
            canvas,
            event_pump,
            scale_factor,
        }
    }

    pub fn start_screen(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn get_events(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}

impl Screen {
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn is_key_pressed(&mut self, _key: u8) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return true,
                sdl2::event::Event::KeyDown { .. } => return true,
                _ => {}
            }
        }
        false
    }

    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let collision = false;
        for (j, &byte) in sprite.iter().enumerate() {
            for i in 0..8 {
                if (byte & (0x80 >> i)) != 0 {
                    let x = (x as u32 + i as u32) * self.scale_factor;
                    let y = (y as u32 + j as u32) * self.scale_factor;
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                    let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(
                        x as i32,
                        y as i32,
                        self.scale_factor,
                        self.scale_factor,
                    ));
                    self.canvas.present()
                }
            }
        }
        collision
    }
}
