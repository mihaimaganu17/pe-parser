use std::{fs, fmt};

pub mod error;
use error::Result;

/// Object representing a Portable Executable file(or PE) as described by
/// Microsoft's documentation
pub struct PE {
    /// A vector of bytes representing all the byte in the file
    data: Vec<u8>,
}

impl PE {
    /// Attempts to parse a PE from the given `file_path`. If the path does not
    /// exist, it returns an error
    pub fn from_path(file_path: &str) -> Result<PE> {
        let data = fs::read(file_path)?;
        Ok( Self { data } )
    }

    /// Attempts to construct a PE from the given `bytes` slice
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let data = Vec::from(bytes);
        Self { data }
    }
}

struct DosHeader {
    /// Magic number
    e_magic:        u16,
    /// Bytes on last page of file
    e_cblp:         u16,
    /// Pages in file
    e_cp:           u16,
    /// Relocations
    e_crlc:         u16,
    /// Size of header in paragraphs
    e_cparhdr:      u16,
    /// Minimum extra paragraphs needed
    e_minalloc:     u16,
    /// Maximum extra paragraphs needed
    e_maxalloc:     u16,
    /// Initial (relative) SS value
    e_ss:           u16,
    /// Initial SP value
    e_sp:           u16,
    /// Checksum
    e_csum:         u16,
    /// Initial IP value
    e_ip:           u16,
    /// Initial (relative) CS value
    e_cs:           u16,
    /// File address of relocation table
    e_lfarlc:       u16,
    /// Overlay number
    e_ovno:         u16,
    /// Reserved words
    e_res:          Vec<u16>,
    /// OEM identifier (for e_oeminfo)
    e_oemid:        u16,
    /// OEM information; e_oemid specific
    e_oeminfo:      u16,
    /// Reserved words
    e_res2:         Vec<u16>,
    /// File address of new exe header
    e_lfanew:       u32,
}

#[cfg(test)]
mod tests {
    use super::PE;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn pe_read_from_path_fails() {
        let new = PE::from_path("new_pe").unwrap();
    }
}
