mod bus;
mod cpu;
mod instructions;
mod instructions2;
mod gpu;
mod debugger;

//use std::time::Duration;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Keys {
    row_1: u8,
    row_2: u8,
}

impl Keys {
    pub fn new_keys() -> Keys {
        Keys { row_1: 0xF, row_2: 0xF }
    }

    fn key_down(&mut self, key: Keycode) {
        match key {
            Keycode::Down => self.row_1 &= 0b11110111,
            Keycode::Up => self.row_1 &= 0b11111011,
            Keycode::Left => self.row_1 &= 0b11111101,
            Keycode::Right => self.row_1 &= 0b11111110,
            Keycode::Return => self.row_2 &= 0b11110111, // Start Button
            Keycode::Space => self.row_2 &= 0b11111011, // Select Button
            Keycode::B => self.row_2 &= 0b11111101,
            Keycode::A => self.row_2 &= 0b11111110,
            _ => (),
        };
    }

    fn key_up(&mut self, key: Keycode) {
        match key {
            Keycode::Down => self.row_1 |= 0b1000,
            Keycode::Up => self.row_1 |= 0b100,
            Keycode::Left => self.row_1 |= 0b10,
            Keycode::Right => self.row_1 |= 0b1,
            Keycode::Return => self.row_2 |= 0b1000, // Start Button
            Keycode::Space => self.row_2 |= 0b100, // Select Button
            Keycode::B => self.row_2 |= 0b10,
            Keycode::A => self.row_2 |= 0b1,
            _ => (),
        };
    }

    pub fn update_keys(&mut self, event: sdl2::event::Event) {
        match event {
            Event::KeyDown { keycode: Some(val), .. } => self.key_down(val),
            Event::KeyUp { keycode: Some(val), .. } => self.key_up(val),
            _ => (),
        };
    }

    pub fn update_register(&self, bus: &mut bus::Bus) {
        let row = (bus.fetch_byte(0xFF00) & 0b110000) >> 4;
        if row & 1 == 0 { // 4 upper bits are set to 1 to keep from reading more values
            bus.set_byte(0xFF00, (self.row_1 & 0xF) | 0b11110000); // update register with direction keys values
        } else if row & 0b10 == 0 {
            bus.set_byte(0xFF00, (self.row_2 & 0xF) | 0b11110000);
        }
    }
}

fn main() {
    let x_size: u32 = 160;
    let y_size: u32 = 144;
    let scale: f32 = 2.0;

    let mut bus: bus::Bus = bus::Bus::new_bus(&String::from("roms/Tetris.GB"));
    //let mut bus: bus::Bus = bus::Bus::new_bus(&String::from("roms/11-op a,(hl).gb"));
    let mut cpu = cpu::CPU::new_cpu();
    let mut gpu = gpu::GPU::new_gpu();
    let mut keys = Keys::new_keys();

    let mut debugger = debugger::Debugger::new_debugger();
    //debugger.set_paused(true);
    let debug = false;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().expect("Failed to generate event pump !");

    let window = video_subsystem.window("GB Emulator", (scale as u32) * x_size, (scale as u32) * y_size)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    canvas.set_scale(scale, scale).unwrap();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::F1), .. } => debugger.set_paused(true),
                _ => keys.update_keys(event),
            };
        }
        if debug == true {
            debugger.tick(&mut cpu, &mut bus, &mut gpu, &mut keys, &mut canvas);
        } else {
            keys.update_register(&mut bus);
            gpu.tick(&mut bus, &mut canvas);
            cpu.tick(&mut bus);
        }
    }
}
