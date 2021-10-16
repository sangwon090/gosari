use std::env;
use std::fs::File;
use std::io::Read;
use gosari::format::{Format, PE};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file: File = File::open(&args[1]).unwrap();
    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data).unwrap();

    match Format::detect(&data) {
        Format::ELF => todo!(),
        Format::MachO => todo!(),
        Format::PE => {
            let (i, pe) = PE::parse(&data).unwrap();
        },
        Format::Unknown => todo!(),
    }
}