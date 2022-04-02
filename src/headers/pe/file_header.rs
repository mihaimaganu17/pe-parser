enum MachineType {
    /// The content of this field is assumed to be applicable to any machine
    Unknown = 0x0,
}


pub struct FileHeader {
    /// PE Magic
    magic:                   u32,
    /// Identifies the type of target machine.
    machine:                 MachineType,
    /// Indicated the size of the section table
    number_of_sections:      u16,
    /// The low 32 bits of the number of seconds since Epoch
    time_date_stamp:         u32,
    /// The file offset of the COFF symbol table, or zero if no COFF symbol
    /// table is present
    pointer_to_symbol_table: u32,
    /// The number of entries in the symbol table. This data can be used to
    /// locate the string table, which immediately follows the symbol table.
    /// This value should be zero for an image because COFF debugging
    /// information is deprecated.
    number_of_symbold:       u32,
    /// Required for executable files but not for object files.
    size_of_optional_header: u16,
    /// Characteristics flags
    characteristics:         u16

}
