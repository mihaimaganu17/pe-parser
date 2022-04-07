use std::{fs};

pub mod headers;
pub mod parsing;
pub mod error;

use crate::{
    headers::{
        dos::DosHeader,
        pe::{
            file_header::FileHeader,
            opt_header::{WindowsSpecific, OptionalHeader}
        },
    },
};
use error::Result;

/// Object representing a Portable Executable file(or PE) as described by
/// Microsoft's documentation
pub struct PE<'pe> {
    /// A slice of bytes representing the content in the file
    bytes: &'pe [u8],
    /// MS-DOS Header
    pub dos_header: DosHeader,
    /// MS-DOS Stub -> Not parsed
    pub dos_stub: Vec<u8>,
    /// PE File Header
    pub file_header: FileHeader,
    /// PE Optional Header
    pub opt_header: OptionalHeader,
}

impl<'pe> PE<'pe> {
    /// Attempts to construct a PE from the given `bytes` slice
    pub fn from_bytes(bytes: &'pe [u8]) -> Result<Self> {
        // Parse the MS-DOS header
        let (dos_header, bytes) = DosHeader::from_bytes(bytes)?;

        // Initialize the file header offset
        let file_header_offset = usize::try_from(dos_header.e_lfanew)?;

        // Consume the MS-DOS stub (or everything until the PE header)
        let (dos_stub, bytes) = bytes
            .split_at(file_header_offset - DosHeader::len());

        // Read the PE File header
        let (file_header, bytes) = FileHeader::from_bytes(bytes)?;

        // Read the PE optional header
        let (opt_header, bytes) = OptionalHeader::from_bytes(bytes)?;

        Ok(Self {
            bytes,
            dos_header,
            dos_stub: dos_stub.to_vec(),
            file_header,
            opt_header
        })
    }

    /// Return how much data has not been parsed
    pub fn remaining_bytes(&self) -> usize {
        self.bytes.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    /// `MZ` Magic used to identify a PE in MS-DOS Header
    const MZ: u16 = 0x5a4d;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    // TODO: Make this tests for all files in the testdata
    fn pe_opt_header() {
        let file_path = "testdata/64bit/notepad.exe";
        let data = fs::read(file_path).unwrap();
        let new = PE::from_bytes(&data).unwrap();

        assert_eq!(MZ, new.dos_header.e_magic);
        assert_eq!(0xf8, new.dos_header.e_lfanew);
        assert_eq!(0x4550, new.file_header.magic);
        assert_eq!(new.opt_header.get_entry_point(), 0x25a30);
    }

    #[test]
    fn pe_data_dir() {
        let file_path = "testdata/64bit/notepad.exe";
        let data = fs::read(file_path).unwrap();
        let pe = PE::from_bytes(&data).unwrap();

        pe.opt_header.data_dirs.iter()
            .for_each(|i| println!("VA: {:08x}, Size: {:08x}", i.virtual_address, i.size));
    }
}
