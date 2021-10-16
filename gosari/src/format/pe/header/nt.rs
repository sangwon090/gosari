use crate::format::coff::FileHeader;
use crate::format::pe::header::{OptionalHeader, OptionalHeader32, OptionalHeader64};

use nom::{IResult, number::complete::{le_u16, le_u32}};

#[derive(Debug, Default)]
pub struct NTHeader {
    pub signature: u32,
    pub file_header: FileHeader,
    pub optional_header: OptionalHeader,
}

impl NTHeader {
    pub fn parse(input: &[u8]) -> IResult<&[u8], NTHeader> {
        let (i, signature) = le_u32(input)?;
        let (i, file_header) = FileHeader::parse(i)?;
        let (i, magic) = le_u16(i)?;
        let (i, optional_header) = match magic {
            0x010B => {
                let (i, optional_header_32) = OptionalHeader32::parse(i)?;
                (i, OptionalHeader::PE32(optional_header_32))
            },
            0x020B => {
                let (i, optional_header_64) = OptionalHeader64::parse(i)?;
                (i, OptionalHeader::PE32Plus(optional_header_64))
            },
            _ => {
                (i, OptionalHeader::Unknown)
            },
        };

        Ok((i, NTHeader {
            signature,
            file_header,
            optional_header,
        }))
    }
}