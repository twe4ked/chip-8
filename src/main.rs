use chip_8::opcode;
use chip_8::vm::{Key, Vm, HEIGHT, WIDTH};
use clap::{App, Arg};
use minifb::{Scale, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let matches = App::new("LC-3 VM")
        .arg(
            Arg::with_name("PROGRAM")
                .help("The program to run.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let mut rom = Vec::new();
    File::open(matches.value_of("PROGRAM").unwrap())
        .expect("unable to open ROM")
        .read_to_end(&mut rom)
        .expect("unable to read ROM");

    let mut vm = Vm::new();
    vm.load_rom(&rom);

    let window_options = WindowOptions {
        scale: Scale::X8,
        ..WindowOptions::default()
    };
    let mut window =
        Window::new("CHIP-8", WIDTH, HEIGHT, window_options).expect("could open window");

    let mut last_instant = Instant::now();
    while window.is_open() {
        sleep(Duration::from_micros(1660) - last_instant.elapsed());

        window.get_keys().map(|keys| {
            vm.key = None;

            for k in keys {
                let key = Key::from(k);
                match key {
                    Some(_) => {
                        vm.key = key;
                        break;
                    }
                    None => (),
                }
            }
        });

        let instruction = vm.fetch();
        let opcode = opcode::decode(instruction);
        vm.execute(opcode);

        window
            .update_with_buffer(&vm.frame_buffer.buffer())
            .expect("could not update buffer");

        last_instant = Instant::now();
    }
}
