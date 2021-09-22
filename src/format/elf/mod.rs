use std::{fs::File, io::{Read, Seek, SeekFrom}};
use crate::format::IFormat;

pub struct ELF {

}

impl ELF {

}

impl IFormat for ELF {
    fn is_valid(file: &mut File) -> bool {
        let mut buffer: [u8; 0x04] = [0; 0x04];
        file.seek(SeekFrom::Start(0)).unwrap();
        file.read_exact(&mut buffer).unwrap();

        if &buffer[0x00..0x04] == [0x7F, 0x45, 0x4C, 0x46] {
            return true;
        }

        false
    }
}