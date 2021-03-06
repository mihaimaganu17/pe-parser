use crate::error::{PeError};

pub enum MachineType {
    /// The content of this field is assumed to be applicable to any machine
    Unknown     = 0x0,
    /// Matsushita AM33
    Am33        = 0x1d3,
    /// x64
    Amd64       = 0x8664,
    /// Arm little endian
    Arm         = 0x1c0,
    /// Arm64 little endian
    Arm64       = 0xaa64,
    /// Arm Thumb-2 little endian
    ArmNt       = 0x1c4,
    /// EFI byte code
    Ebc         = 0xebc,
    /// Intel 386 or later processors and compatible processors
    I386        = 0x14c,
    /// Intel Itanium processor family
    Ia64        = 0x200,
    /// LoongArch 32-bit processor family
    LoongArch32 = 0x6232,
    /// LoongArch 64-bit processor family
    LoongArch64 = 0x6264,
    /// Mitsubishi M32R little endian
    M32R        = 0x9041,
    /// Mips16
    Mips16      = 0x266,
    /// Mips with FPU
    MipsFpu     = 0x366,
    /// Mips16 with FPU
    MipsFpu16   = 0x466,
    /// Power PC little endian
    PowerPc     = 0x1f0,
    /// Power Pc with floating point support,
    PowerPcFp   = 0x1f1,
    /// Mips little endian
    R4000       = 0x166,
    /// Risc-V 32-bit address space
    RiscV32     = 0x5032,
    /// Risc-V 64-bit address space
    RiscV64     = 0x5064,
    /// Risc-V 128-bit address space
    RiscV128    = 0x5128,
    /// Hitachi SH3
    Sh3         = 0x1a2,
    /// Hitachi SH3 DSP
    Sh3Dsp      = 0x1a3,
    /// Hitachi SH4
    Sh4         = 0x1a6,
    /// Hitachi SH5
    Sh5         = 0x1a8,
    /// Thumb
    Thumb       = 0x1c2,
    /// Mips little-endian WCE v2
    WceMipsV2   = 0x169,
    /// Invalid Machine Type
    Invalid     = 0x1,
}

impl TryFrom<u16> for MachineType {
    type Error = PeError;
    fn try_from(value: u16) -> Result<MachineType, Self::Error> {
        match value {
            0x0     => Ok(Self::Unknown),
            0x1d3   => Ok(Self::Am33),
            0x8664  => Ok(Self::Amd64),
            0x1c0   => Ok(Self::Arm),
            0xaa64  => Ok(Self::Arm64),
            0x1c4   => Ok(Self::ArmNt),
            0xebc   => Ok(Self::Ebc),
            0x14c   => Ok(Self::I386),
            0x200   => Ok(Self::Ia64),
            0x6232  => Ok(Self::LoongArch32),
            0x6264  => Ok(Self::LoongArch64),
            0x9041  => Ok(Self::M32R),
            0x266   => Ok(Self::Mips16),
            0x366   => Ok(Self::MipsFpu),
            0x466   => Ok(Self::MipsFpu16),
            0x1f0   => Ok(Self::PowerPc),
            0x1f1   => Ok(Self::PowerPcFp),
            0x166   => Ok(Self::R4000),
            0x5032  => Ok(Self::RiscV32),
            0x5064  => Ok(Self::RiscV64),
            0x5128  => Ok(Self::RiscV128),
            0x1a2   => Ok(Self::Sh3),
            0x1a3   => Ok(Self::Sh3Dsp),
            0x1a6   => Ok(Self::Sh4),
            0x1a8   => Ok(Self::Sh5),
            0x1c2   => Ok(Self::Thumb),
            0x169   => Ok(Self::WceMipsV2),
            _       => Err(PeError::InvalidMachineType(value)),
        }
    }
}
