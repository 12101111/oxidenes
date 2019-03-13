use log::{LevelFilter, Log, Metadata, Record};
use oxidenes::NES;
use std::{env, fs::File, io::Read};

static LOGGER: Logger = Logger;

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        println!("{:<5} {}", record.level().to_string(), record.args());
    }
    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
}

fn main() {
    init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE: oxidenes <PATH TO NES FILE>");
        return;
    }
    let rom_path = args[1].clone();
    let mut rom = File::open(&rom_path).unwrap();
    let mut buffer = Vec::with_capacity(24 * 1024);
    rom.read_to_end(&mut buffer).expect("Read File error");
    let mut nes = NES::new(&buffer);
    nes.set_pc(0xC000);
    loop {
        nes.step();
        if nes.get_cycles() > 26554 {
            break;
        }
    }
}