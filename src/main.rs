mod bus;
mod cpu;
mod instructions;
mod gpu;

//use sdl2;

fn main() {
    let mut bus: bus::Bus = bus::Bus::new_bus(&String::from("Tetris.GB"));
    let mut cpu = cpu::CPU::new_cpu();
    let mut gpu = gpu::GPU::new_gpu();

    /*let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("GB Emulator", 160, 144)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();*/

    let debugging = false;
    let advanced_debug = true;
    cpu.set_breakpoint(0x27a3);

    loop {
        if debugging && cpu.pc <= 0x213 && cpu.pc >= 0x20b{
            println!("{:#x}", cpu.pc);
            let op = cpu.fetch_byte(&mut bus, cpu.pc);
            let instruction = &instructions::Instruction::SET[op as usize];
            println!("[{:#x}] {} {:#x} {:#x}", cpu.fetch_byte(&mut bus, cpu.pc), instruction.disassembly, cpu.fetch_byte(&mut bus, cpu.pc + 1), cpu.fetch_byte(&mut bus, cpu.pc + 2));
            if op == 0xf0 {
                println!("0xff44 = {:#x}", bus.fetch_byte(0xff44));
            }
            println!("Y coord = {}", gpu.get_y_coord());
        }
        gpu.tick(&mut bus);
        cpu.tick(&mut bus, advanced_debug);
        /*let tile = (0..=16).map(|i| cpu.fetch_byte(&mut bus, 0x8000 + i));
        for hex in tile {
            print!("{:#x} ", hex);
        }
        println!("");*/
    }
}
