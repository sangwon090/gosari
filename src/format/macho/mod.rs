use std::{fs::File, io::{Read, Seek, SeekFrom}};
use crate::format::IFormat;

pub struct MachO {

}

impl MachO {

}

impl IFormat for MachO {
    fn is_valid(file: &mut File) -> bool {
        let mut buffer: [u8; 0x04] = [0; 0x04];
        file.seek(SeekFrom::Start(0)).unwrap();
        file.read_exact(&mut buffer).unwrap();

        if &buffer[0x00..0x04] == [0xCF, 0xFA, 0xED, 0xFE] {
            return true;
        } else if &buffer[0x00..0x04] == [0xCE, 0xFA, 0xED, 0xFE] {
            return true;
        }

        false
    }
}