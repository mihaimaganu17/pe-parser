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
    pub fn from_bytes(bytes: &mut Vec<u8>) -> Result<Self> {
        let e_magic     = take_u16(bytes)?;
        let e_cblp      = take_u16(bytes)?;
        let e_cp        = take_u16(bytes)?;
        let e_crlc      = take_u16(bytes)?;
        let e_cparhdr   = take_u16(bytes)?;
        let e_minalloc  = take_u16(bytes)?;
        let e_maxalloc  = take_u16(bytes)?;
        let e_ss        = take_u16(bytes)?;
        let e_sp        = take_u16(bytes)?;
        let e_csum      = take_u16(bytes)?;
        let e_ip        = take_u16(bytes)?;
        let e_cs        = take_u16(bytes)?;
        let e_lfarlc    = take_u16(bytes)?;
        let e_ovno      = take_u16(bytes)?;

        let e_res = bytes.drain(..8).collect::<Vec<u8>>().chunks(2)
            .map(|x| take_u16(&mut x.to_vec()).unwrap()).collect();

        let e_oemid     = take_u16(bytes)?;
        let e_oeminfo   = take_u16(bytes)?;

        let e_res2 = bytes.drain(..20).collect::<Vec<u8>>().chunks(2)
            .map(|x| take_u16(&mut x.to_vec()).unwrap()).collect();
        let e_lfanew    = take_u32(bytes)?;

        Ok( DosHeader {
            e_magic, e_cblp, e_cp, e_crlc, e_cparhdr, e_minalloc, e_maxalloc,
            e_ss, e_sp, e_csum, e_ip, e_cs, e_lfarlc, e_ovno, e_res, e_oemid,
            e_oeminfo, e_res2, e_lfanew
        })
    }

    pub fn len() -> usize {
        64usize
    }
}
