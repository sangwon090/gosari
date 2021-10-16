use crate::format::pe::header::{PEHeader};
use std::collections::HashMap;
use nom::IResult;

mod header;
mod characteristic;
mod subsystem;

pub use header::*;
pub use characteristic::DllCharacteristics;
pub use subsystem::Subsystem;

#[derive(Debug, Default)]
pub struct PE {
    pub header: PEHeader,
    pub sections: HashMap<String, Vec<u8>>,
}

impl PE {
    pub fn parse(input: &[u8]) -> IResult<&[u8], PE> {
        let (i, header) = PEHeader::parse(input)?;
        let mut sections: HashMap<String, Vec<u8>> = HashMap::new();

        for section_header in &header.section_headers {
            let offset = section_header.pointer_to_raw_data as usize;
            let size = section_header.size_of_raw_data as usize;
            let section = &input[offset as usize..(offset + size) as usize];
            sections.insert(section_header.name.to_string(), section.to_vec());
        }

        Ok((i, PE {
            header,
            sections,
        }))
    }

    pub fn is_valid(input: &[u8]) -> bool {
        if &input[0x00..0x02] == [0x4D, 0x5A] {
            return true;
        }

        false
    }
}