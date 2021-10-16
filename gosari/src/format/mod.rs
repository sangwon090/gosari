pub mod coff;
pub mod elf;
pub mod macho;
pub mod pe;

pub use coff::FileHeader;
pub use elf::ELF;
pub use macho::MachO;
pub use pe::PE;

#[derive(Debug)]
pub enum Format {
    ELF,
    MachO,
    PE,
    Unknown,
}

impl Format {
    pub fn detect(input: &[u8]) -> Format {
        if ELF::is_valid(input) {
            return Format::ELF;
        } else if MachO::is_valid(input) {
            return Format::MachO;
        } else if PE::is_valid(input) {
            return Format::PE;
        } else {
            return Format::Unknown;
        }
    }
}