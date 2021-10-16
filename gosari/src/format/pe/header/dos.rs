use nom::{IResult, number::complete::{le_u16, le_u32}, multi::count};
use std::convert::TryInto;

#[derive(Debug, Default)]
pub struct DOSHeader {
    pub magic: u16,
    pub cblp: u16,
    pub cp: u16,
    pub crlc: u16,
    pub cparhdr: u16,
    pub minalloc: u16,
    pub maxalloc: u16,
    pub ss: u16,
    pub sp: u16,
    pub csum: u16,
    pub ip: u16,
    pub cs: u16,
    pub lfarlc: u16,
    pub ovno: u16,
    pub res: [u16; 4],
    pub oemid: u16,
    pub oeminfo: u16,
    pub res2: [u16; 10],
    pub lfanew: u32,
}

impl DOSHeader {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DOSHeader> {
        let (i, magic) = le_u16(input)?;
        let (i, cblp) = le_u16(i)?;
        let (i, cp) = le_u16(i)?;
        let (i, crlc) = le_u16(i)?;
        let (i, cparhdr) = le_u16(i)?;
        let (i, minalloc) = le_u16(i)?;
        let (i, maxalloc) = le_u16(i)?;
        let (i, ss) = le_u16(i)?;
        let (i, sp) = le_u16(i)?;
        let (i, csum) = le_u16(i)?;
        let (i, ip) = le_u16(i)?;
        let (i, cs) = le_u16(i)?;
        let (i, lfarlc) = le_u16(i)?;
        let (i, ovno) = le_u16(i)?;
        let (i, res) = count(le_u16, 4)(i)?;
        let (i, oemid) = le_u16(i)?;
        let (i, oeminfo) = le_u16(i)?;
        let (i, res2) = count(le_u16, 10)(i)?;
        let (i, lfanew) = le_u32(i)?;

        Ok((i, DOSHeader {
            magic,
            cblp,
            cp,
            crlc,
            cparhdr,
            minalloc,
            maxalloc,
            ss,
            sp,
            csum,
            ip,
            cs,
            lfarlc,
            ovno,
            res: res.try_into().unwrap(),
            oemid,
            oeminfo,
            res2: res2.try_into().unwrap(),
            lfanew,
        }))
    }
}