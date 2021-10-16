use crate::format::pe::header::{DOSHeader, NTHeader, SectionHeader};
use nom::{IResult, bytes::complete::take};

#[derive(Debug, Default)]
pub struct PEHeader {
    pub dos_header: DOSHeader,
    pub dos_stub: Vec<u8>,
    pub nt_header: NTHeader,
    pub section_headers: Vec<SectionHeader>,
}

impl PEHeader {
    pub fn parse(input: &[u8]) -> IResult<&[u8], PEHeader> {
        let (i, dos_header) = DOSHeader::parse(input)?;
        let (i, dos_stub) = take(dos_header.lfanew - 64)(i)?;
        let (i, nt_header) = NTHeader::parse(i)?;
        let mut section_headers: Vec<SectionHeader> = Vec::new();
        let mut i: &[u8] = i;

        for _ in 0..nt_header.file_header.number_of_sections {
            let (_i, section_header) = SectionHeader::parse(i)?;
            section_headers.push(section_header);
            i = _i;
        }

        Ok((i, PEHeader {
            dos_header,
            dos_stub: dos_stub.to_vec(),
            nt_header,
            section_headers,
        }))
    }
}