use nom::{IResult, number::complete::{le_u32}};


#[derive(Debug, Default)]
pub struct DataDirectory {
    virtual_address: u32,
    size: u32,
}

impl DataDirectory {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DataDirectory> {
        let (i, virtual_address) = le_u32(input)?;
        let (i, size) = le_u32(i)?;

        Ok((i, DataDirectory {
            virtual_address,
            size,
        }))
    }
}