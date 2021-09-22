mod elf;
mod macho;
mod pe;

pub use elf::ELF;
pub use macho::MachO;
pub use pe::PE;

use std::fs::File;

#[derive(Debug)]
pub enum Format {
    ELF,
    MachO,
    PE,
    Unknown,
}

pub trait IFormat {
    fn is_valid(file: &mut File) -> bool;
}

impl Format {
    pub fn detect(file: &mut File) -> Format {
        if ELF::is_valid(file) {
            return Format::ELF;
        } else if MachO::is_valid(file) {
            return Format::MachO;
        } else if PE::is_valid(file) {
            return Format::PE;
        } else {
            return Format::Unknown;
        }
    }
}