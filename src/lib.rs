use std::{fs};

pub mod headers;
pub mod error;

use crate::{
    headers::{dos::DosHeader},
};
use error::Result;

/// Object representing a Portable Executable file(or PE) as described by
/// Microsoft's documentation
pub struct PE {
    /// A vector of bytes representing all the byte in the file
    data: Vec<u8>,
    /// MS-DOS Header
    dos_header: DosHeader,
    /// MS-DOS Stub -> Not parsed
    dos_stub: Vec<u8>,
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
        // Consume the MS-DOS stub (or everything until the PE header)
        let dos_stub = data.drain(..usize::try_from(dos_header.e_lfanew)?)
            .collect();

        Ok(Self {
            data,
            dos_header,
            dos_stub
        })
    }

    /// Return how much data has not been parsed
    pub fn remaining_bytes(&self) -> usize {
        self.data.len()
    }
}


#[cfg(test)]
mod tests {
    use super::PE;

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
    }
}
