use nom::{IResult, number::complete::{le_u16, le_u32}};

#[derive(Debug, Default)]
pub struct FileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

impl FileHeader {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FileHeader> {
        let (i, machine) = le_u16(input)?;
        let (i, number_of_sections) = le_u16(i)?;
        let (i, time_date_stamp) = le_u32(i)?;
        let (i, pointer_to_symbol_table) = le_u32(i)?;
        let (i, number_of_symbols) = le_u32(i)?;
        let (i, size_of_optional_header) = le_u16(i)?;
        let (i, characteristics) = le_u16(i)?;

        Ok((i, FileHeader {
            machine,
            number_of_sections,
            time_date_stamp,
            pointer_to_symbol_table,
            number_of_symbols,
            size_of_optional_header,
            characteristics,
        }))
    }
}