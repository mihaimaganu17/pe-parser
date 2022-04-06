use crate::{
    error::Result,
    parsing::*
};
/// MS-DOS Legacy header
#[derive(Debug)]
pub struct DosHeader {
    /// Magic number
    pub e_magic:        u16,
    /// Bytes on last page of file
    pub e_cblp:         u16,
    /// Pages in file
    pub e_cp:           u16,
    /// Relocations
    pub e_crlc:         u16,
    /// Size of header in paragraphs
    pub e_cparhdr:      u16,
    /// Minimum extra paragraphs needed
    pub e_minalloc:     u16,
    /// Maximum extra paragraphs needed
    pub e_maxalloc:     u16,
    /// Initial (relative) SS value
    pub e_ss:           u16,
    /// Initial SP value
    pub e_sp:           u16,
    /// Checksum
    pub e_csum:         u16,
    /// Initial IP value
    pub e_ip:           u16,
    /// Initial (relative) CS value
    pub e_cs:           u16,
    /// File address of relocation table
    pub e_lfarlc:       u16,
    /// Overlay number
    pub e_ovno:         u16,
    /// Reserved words (array length is 4)
    pub e_res:          Vec<u16>,
    /// OEM identifier (for e_oeminfo)
    pub e_oemid:        u16,
    /// OEM information; e_oemid specific
    pub e_oeminfo:      u16,
    /// Reserved words (array length is 10)
    pub e_res2:         Vec<u16>,
    /// File address of new exe header
    pub e_lfanew:       u32,
}

impl DosHeader {
    pub fn from_bytes<'pe>(bytes: &'pe [u8]) -> Result<(Self, &[u8])> {
        let (e_magic, bytes)     = take_u16(bytes)?;
        let (e_cblp, bytes)      = take_u16(bytes)?;
        let (e_cp, bytes)        = take_u16(bytes)?;
        let (e_crlc, bytes)      = take_u16(bytes)?;
        let (e_cparhdr, bytes)   = take_u16(bytes)?;
        let (e_minalloc, bytes)  = take_u16(bytes)?;
        let (e_maxalloc, bytes)  = take_u16(bytes)?;
        let (e_ss, bytes)        = take_u16(bytes)?;
        let (e_sp, bytes)        = take_u16(bytes)?;
        let (e_csum, bytes)      = take_u16(bytes)?;
        let (e_ip, bytes)        = take_u16(bytes)?;
        let (e_cs, bytes)        = take_u16(bytes)?;
        let (e_lfarlc, bytes)    = take_u16(bytes)?;
        let (e_ovno, bytes)      = take_u16(bytes)?;

        let mut e_res = Vec::with_capacity(4usize);
        for chunk in bytes.chunks(2) {
            let (value, bytes) = take_u16(bytes)?;
            e_res.push(value);
        }

        let (_, bytes) = bytes.split_at(8usize);

        let (e_oemid, bytes)     = take_u16(bytes)?;
        let (e_oeminfo, bytes)   = take_u16(bytes)?;

        let mut e_res2 = Vec::with_capacity(10usize);
        for chunk in bytes.chunks(2) {
            let (value, bytes) = take_u16(bytes)?;
            e_res.push(value);
        }
        let (_, bytes) = bytes.split_at(20usize);

        let (e_lfanew, bytes)    = take_u32(bytes)?;

        Ok((DosHeader {
            e_magic, e_cblp, e_cp, e_crlc, e_cparhdr, e_minalloc, e_maxalloc,
            e_ss, e_sp, e_csum, e_ip, e_cs, e_lfarlc, e_ovno, e_res, e_oemid,
            e_oeminfo, e_res2, e_lfanew
        }, bytes))
    }

    pub fn len() -> usize {
        64usize
    }
}
