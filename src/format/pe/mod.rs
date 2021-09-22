use std::{fs::File, io::{Read, Seek, SeekFrom}};
use crate::format::IFormat;

pub struct PE {

}

impl PE {

}

impl IFormat for PE {
    fn is_valid(file: &mut File) -> bool {
        let mut buffer: [u8; 0x02] = [0; 0x02];
        file.seek(SeekFrom::Start(0)).unwrap();
        file.read_exact(&mut buffer).unwrap();

        if &buffer[0x00..0x02] == [0x4D, 0x5A] {
            return true;
        }

        false
    }    
}