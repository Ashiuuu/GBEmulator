mod bus;
mod canvas;
mod color;
mod cpu;
// mod debugger;
mod buttons;
mod gpu;
mod instructions;
mod registers;

use buttons::Buttons;

use std::time::Duration;

use canvas::Canvas;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

fn main() {
    let x_size: usize = 160;
    let y_size: usize = 144;
    let _scale: f32 = 2.0;

    let mut bus: bus::Bus = bus::Bus::new_bus(&String::from("roms/Tetris.GB"));
    //let mut bus: bus::Bus = bus::Bus::new_bus(&String::from("roms/11-op a,(hl).gb"));
    let mut cpu = cpu::CPU::new_cpu();
    let mut gpu = gpu::GPU::new();
    let mut keys = Buttons::new();

    // let mut debugger = debugger::Debugger::new_debugger();
    //debugger.set_paused(true);
    //let _debug = false;

    let mut canvas = Canvas::new(x_size, y_size);

    let mut window = Window::new(
        "GB Emulator",
        x_size,
        y_size,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X4,
            scale_mode: ScaleMode::Stretch,
            topmost: false,
            transparency: false,
            none: false,
        },
    )
    .unwrap_or_else(|err| {
        panic!("Couldn't create window: {}", err);
    });

    window.limit_update_rate(Some(Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        keys.update_keys(&window);
        keys.update_register(&mut bus);
        gpu.tick(&mut bus, &mut canvas, &mut window);
        cpu.tick(&mut bus);

        window
            .update_with_buffer(&canvas[..], x_size, y_size)
            .unwrap();
    }

    //'main_loop: loop {
    //for event in event_pump.poll_iter() {
    //match event {
    //Event::Quit { .. } => break 'main_loop,
    //Event::KeyDown {
    //keycode: Some(Keycode::Escape),
    //..
    //} => break 'main_loop,
    //Event::KeyDown {
    //keycode: Some(Keycode::F1),
    //..
    //} => debugger.set_paused(true),
    //_ => keys.update_keys(event),
    //};
    //}
    //if debug == true {
    //debugger.tick(&mut cpu, &mut bus, &mut gpu, &mut keys, &mut canvas);
    //} else {
    //keys.update_register(&mut bus);
    //gpu.tick(&mut bus, &mut canvas);
    //cpu.tick(&mut bus);
    //}
    //}
}
