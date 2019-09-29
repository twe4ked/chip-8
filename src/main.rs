use chip_8::opcode;
use chip_8::vm::{Key, Vm, HEIGHT, WIDTH};
use clap::{App, Arg};
use minifb::{Scale, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::thread::{self, sleep};
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

    let (tx_buf, rx_buf) = mpsc::channel::<Vec<u32>>();
    let (tx_key, rx_key) = mpsc::channel::<Option<Key>>();

    thread::spawn(move || {
        let mut vm = Vm::new();
        vm.load_rom(&rom);

        let mut last_instant = Instant::now();
        loop {
            sleep(Duration::from_micros(1660) - last_instant.elapsed());

            vm.key = rx_key.try_iter().last().unwrap_or(None);

            let instruction = vm.fetch();
            let opcode = opcode::decode(instruction);
            vm.execute(opcode);

            tx_buf
                .send(vm.frame_buffer.buffer().clone())
                .expect("unable to send buffer");

            last_instant = Instant::now();
        }
    });

    let window_options = WindowOptions {
        scale: Scale::X8,
        ..WindowOptions::default()
    };
    let mut window =
        Window::new("CHIP-8", WIDTH, HEIGHT, window_options).expect("could open window");

    while window.is_open() {
        window.get_keys().map(|keys| {
            tx_key
                .send(keys.iter().find_map(|k| Key::from(*k)))
                .expect("key send failed")
        });

        match rx_buf.try_iter().last() {
            Some(buffer) => window
                .update_with_buffer(&buffer)
                .expect("could not update buffer"),
            None => window.update(),
        }
    }
}
