use std::{fs};

pub mod headers;
pub mod parsing;
pub mod error;

use crate::{
    headers::{
        dos::DosHeader,
        pe::{
            file_header::FileHeader,
        },
    },
};
use error::Result;

/// Object representing a Portable Executable file(or PE) as described by
/// Microsoft's documentation
pub struct PE {
    /// A vector of bytes representing all the byte in the file
    data: Vec<u8>,
    /// MS-DOS Header
    pub dos_header: DosHeader,
    /// MS-DOS Stub -> Not parsed
    pub dos_stub: Vec<u8>,
    /// PE File Header
    pub file_header: FileHeader,
}

impl PE {
    /// Attempts to parse a PE from the given `file_path`. If the path does not
    /// exist, it returns an error
    pub fn from_path(file_path: &str) -> Result<Self> {
        let data = fs::read(file_path)?;
        Self::from_bytes(&data)
    }

    /// Attempts to construct a PE from the given `bytes` slice
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        // Construct data from bytes received
        let mut data = Vec::from(bytes);

        // Parse the MS-DOS header
        let dos_header = DosHeader::from_bytes(
            &mut data.drain(..DosHeader::len()).collect())?;

        // Initialize the file header offset
        let file_header_offset = usize::try_from(dos_header.e_lfanew)?;

        // Consume the MS-DOS stub (or everything until the PE header)
        let dos_stub = data.drain(..file_header_offset - DosHeader::len())
            .collect();

        // Read the PE File header
        let file_header = FileHeader::from_bytes(
            &mut data.drain(..FileHeader::len()).collect())?;

        Ok(Self {
            data,
            dos_header,
            dos_stub,
            file_header
        })
    }

    /// Return how much data has not been parsed
    pub fn remaining_bytes(&self) -> usize {
        self.data.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// `MZ` Magic used to identify a PE in MS-DOS Header
    const MZ: u16 = 0x5a4d;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    // TODO: Make this tests for all files in the testdata
    fn pe_read_from_path_fails() {
        let new = PE::from_path("testdata/64bit/notepad.exe").unwrap();
        assert_eq!(MZ, new.dos_header.e_magic);
        assert_eq!(0xf8, new.dos_header.e_lfanew);
        assert_eq!(0x4550, new.file_header.magic);
    }
}
