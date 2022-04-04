#[derive(Debug, PartialEq)]
pub enum CharacteristicsFlag {
    /// This indicates that the file does not contain base relocations and must
    /// therefore be loaded at its preferred base address. If the base address
    /// is not available, the loader reports and error. The default behavior of
    /// the linke is to strip base relocations from executable (EXE) files.
    RelocsStripped = 0x0001,
    /// Image only. This indicates that the image file is valid and can be run.
    /// If this flag is not set, it indicates a linker error.
    ExecutableImage = 0x0002,
    /// COFF line numbers have been removed. This flag is deprecated and should
    /// be zero.
    LineNumsStripped = 0x0004,
    /// COFF symbol table entries for local symbols have been removed. This
    /// flag is deprecated and should be zero.
    LocalSymsStripped = 0x0008,
    /// Obsolete. Aggressively trim working set. This flag is deprecated for
    /// Windows 2000 and later and must be zero.
    AggressiveWsTrim = 0x0010,
    /// Application can handle > 2-GB addresses.
    LargeAddressAware = 0x0020,
    /// This flag is reserved for future use.
    Reserved = 0x0040,
    /// Indicates little endian
    BytesReversedLo = 0x0080,
    /// Machine is based on a 32-bit-word architecture
    Machine32Bit = 0x0100,
    /// Debugging information is removed from the image file.
    DebugStripped = 0x0200,
    /// If the image is on removable media, fully load it and copy it to the
    /// swap file.
    RemovableRunFromSwap = 0x0400,
    /// If the image is on network media, fully load it and copy it to the
    /// swap file.
    NetRunFromSwap = 0x0800,
    /// The image file is a system file, not a user program.
    System = 0x1000,
    /// The image file is a dynamic-link library(DLL)
    Dll = 0x2000,
    /// The file should be run only on a uniprocessor machine.
    UpSystemOnly = 0x4000,
    /// Big endian. This flag is deprecated and should be zero
    BytesReversedHi = 0x8000,
    /// Invalid
    Invalid = 0x0,
}

impl CharacteristicsFlag {
    pub fn to_vec(value: u16) -> Vec<Self> {
        let mut characteristics = Vec::new();

        // Cycle through every bit flag
        for i in 0..16 {
            let check_flag = 1 << i;
            // Check if flag is present
            let new_flag = match check_flag & value {
                0x0001 => Self::RelocsStripped,
                0x0002 => Self::ExecutableImage,
                0x0004 => Self::LineNumsStripped,
                0x0008 => Self::LocalSymsStripped,
                0x0010 => Self::AggressiveWsTrim,
                0x0020 => Self::LargeAddressAware,
                0x0040 => Self::Reserved,
                0x0080 => Self::BytesReversedLo,
                0x0100 => Self::Machine32Bit,
                0x0200 => Self::DebugStripped,
                0x0400 => Self::RemovableRunFromSwap,
                0x0800 => Self::NetRunFromSwap,
                0x1000 => Self::System,
                0x2000 => Self::Dll,
                0x4000 => Self::UpSystemOnly,
                0x8000 => Self::BytesReversedHi,
                // This option is only meant to satisfy Rust's pattern
                // exhaustive matching
                _ => Self::Invalid,
            };
            // If the flag is not invalid we add it to our list
            if new_flag != Self::Invalid {
                characteristics.push(new_flag);
            }
        }
        characteristics
    }
}

