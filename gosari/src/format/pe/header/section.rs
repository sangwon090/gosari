use nom::{IResult, bytes::complete::{take}, number::complete::{le_u16, le_u32}};

#[derive(Debug)]
pub struct SectionHeader {
    pub name: String,
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

impl SectionHeader {
    pub fn parse(input: &[u8]) -> IResult<&[u8], SectionHeader> {
        let (i, name) = take(8u8)(input)?;
        let (i, virtual_size) = le_u32(i)?;
        let (i, virtual_address) = le_u32(i)?;
        let (i, size_of_raw_data) = le_u32(i)?;
        let (i, pointer_to_raw_data) = le_u32(i)?;
        let (i, pointer_to_relocations) = le_u32(i)?;
        let (i, pointer_to_linenumbers) = le_u32(i)?;
        let (i, number_of_relocations) = le_u16(i)?;
        let (i, number_of_linenumbers) = le_u16(i)?;
        let (i, characteristics) = le_u32(i)?;

        let name = String::from_utf8(name.to_vec()).unwrap().replace("\0", "");

        Ok((i, SectionHeader {
            name,
            virtual_size,
            virtual_address,
            size_of_raw_data,
            pointer_to_raw_data,
            pointer_to_relocations,
            pointer_to_linenumbers,
            number_of_relocations,
            number_of_linenumbers,
            characteristics,
        }))
    }
}