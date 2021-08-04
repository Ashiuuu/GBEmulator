mod bus;
mod cpu;
mod instructions;
mod instructions2;
mod gpu;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let x_size: u32 = 160;
    let y_size: u32 = 144;
    let scale: f32 = 2.0;

    let mut bus: bus::Bus = bus::Bus::new_bus(&String::from("Tetris.GB"));
    //let mut bus: bus::Bus = bus::Bus::new_bus(&String::from("cpu_instrs.gb"));
    let mut cpu = cpu::CPU::new_cpu();
    let mut gpu = gpu::GPU::new_gpu(x_size, y_size);

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

    let debugging = false;
    let advanced_debug_mode = 0;
    cpu.set_breakpoint(0x27c3);

    'main_loop: loop {
        if debugging {
            println!("{:#x}", cpu.pc);
            let op = cpu.fetch_byte(&mut bus, cpu.pc);
            let instruction = &instructions::Instruction::SET[op as usize];
            println!("[{:#x}] {} {:#x} {:#x}", cpu.fetch_byte(&mut bus, cpu.pc), instruction.disassembly, cpu.fetch_byte(&mut bus, cpu.pc + 1), cpu.fetch_byte(&mut bus, cpu.pc + 2));
            if op == 0xf0 {
                println!("0xff44 = {:#x}", bus.fetch_byte(0xff44));
            }
            println!("Y coord = {}", gpu.get_y_coord());
        }

        /*if cpu.pc == 0x282a {
            for i in 0..16 {
                print!("{:02x} ", bus.fetch_byte(0x8000 + i));
            }
            println!("");
            break 'main_loop;
        }*/

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
                _ => (),
            };
        }

        gpu.tick(&mut bus, &mut canvas);
        cpu.tick(&mut bus, advanced_debug_mode);
    }
}
