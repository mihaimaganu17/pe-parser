use crate::{
    headers::pe::{
        machine::MachineType,
    },
    parsing::*,
    error::Result,
};

/// Also known as COFF header
pub struct FileHeader {
    /// PE Magic
    pub magic:                   u32,
    /// Identifies the type of target machine.
    pub machine:                 MachineType,
    /// Indicated the size of the section table
    pub number_of_sections:      u16,
    /// The low 32 bits of the number of seconds since Epoch
    pub time_date_stamp:         u32,
    /// The file offset of the COFF symbol table, or zero if no COFF symbol
    /// table is present
    pub pointer_to_symbol_table: u32,
    /// The number of entries in the symbol table. This data can be used to
    /// locate the string table, which immediately follows the symbol table.
    /// This value should be zero for an image because COFF debugging
    /// information is deprecated.
    pub number_of_symbols:       u32,
    /// Required for executable files but not for object files.
    pub size_of_optional_header: u16,
    /// Characteristics flags
    pub characteristics:         u16,
}


impl FileHeader {
    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let (magic, bytes)                   = take_u32(bytes)?;
        let (machine, bytes)                 = take_u16(bytes)?;
        let (number_of_sections, bytes)      = take_u16(bytes)?;
        let (time_date_stamp, bytes)         = take_u32(bytes)?;
        let (pointer_to_symbol_table, bytes) = take_u32(bytes)?;
        let (number_of_symbols, bytes)       = take_u32(bytes)?;
        let (size_of_optional_header, bytes) = take_u16(bytes)?;
        let (characteristics, bytes)         = take_u16(bytes)?;

        // Convert to MachineType
        let machine = machine.try_into()?;

        Ok((Self {
            magic, machine, number_of_sections, time_date_stamp,
            pointer_to_symbol_table, number_of_symbols,
            size_of_optional_header, characteristics
        }, bytes))
    }

    pub fn len() -> usize {
        24usize
    }
}
