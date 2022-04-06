use crate::{
    error::{Result, PeError},
    parsing::*
};

pub struct OptionalHeader {
    /// Defines the type of the image file. The most common is 0x10B which is
    /// a normal executable file.
    magic: ImageType,
    /// The linker major version number
    major_linker_version: u8,
    /// The linker minor version number
    minor_linker_version: u8,
    /// The size of code(text) section, or the sum of all code sections if
    /// there are multiple sections.
    size_of_code: u32,
    /// The size of the initialized data section, or the sum of all such
    /// sections if there are multiple data sections
    size_of_initialized_data: u32,
    /// The size of the uninitialized data section(BSS), of the sum of all such
    /// sections if there are multiple BSS sections.
    size_of_uninitialized_data: u32,
    /// The address of the entry point relative to the image base when the exe
    /// file is loaded into memory
    addr_of_entry_point: u32,
    /// The address that is relative to the image base of the beginning-of-code
    /// section when it is loaded into memory.
    base_of_code: u32,
    /// Windows specific fields
    pub win_fields: WindowsSpecific
}

impl OptionalHeader {
    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let (magic, bytes) = take_u16(bytes)?;
        let magic = magic.into();
        let (major_linker_version, bytes) = take_u8(bytes)?;
        let (minor_linker_version, bytes) = take_u8(bytes)?;
        let (size_of_code, bytes) = take_u32(bytes)?;
        let (size_of_initialized_data, bytes) = take_u32(bytes)?;
        let (size_of_uninitialized_data, bytes) = take_u32(bytes)?;
        let (addr_of_entry_point, bytes) = take_u32(bytes)?;
        let (base_of_code, bytes) = take_u32(bytes)?;
        let (win_fields, bytes) = WindowsSpecific::from_bytes(magic, bytes)?;

        Ok(( Self {
            magic,
            major_linker_version,
            minor_linker_version,
            size_of_code,
            size_of_initialized_data,
            size_of_uninitialized_data,
            addr_of_entry_point,
            base_of_code,
            win_fields
        }, bytes))
    }
}

#[derive(Clone, Copy)]
pub enum ImageType {
    Pe32 = 0x010B,
    ROM = 0x107,
    Pe64 = 0x20B,
    Unknown,
}

impl From<u16> for ImageType {
    fn from(value: u16) -> ImageType {
        match value {
            0x10B => ImageType::Pe32,
            0x107 => ImageType::ROM,
            0x20B => ImageType::Pe64,
            _ => ImageType::Unknown,
        }
    }
}

pub enum WindowsSpecific {
    PE32(Pe32),
    PE64(Pe64)
}

impl WindowsSpecific {
    pub fn from_bytes(magic: ImageType, bytes: &[u8])
            -> Result<(Self, &[u8])> {
        match magic {
            ImageType::Pe32 => {
                let (pe32, bytes) = Pe32::from_bytes(bytes)?;
                Ok((Self::PE32(pe32), bytes))
            },
            ImageType::Pe64 => {
                let (pe64, bytes) = Pe64::from_bytes(bytes)?;
                Ok((Self::PE64(pe64), bytes))
            },
            ImageType::ROM => Err(PeError::Unimplemented),
            ImageType::Unknown => Err(PeError::Unimplemented)
        }
    }
}

pub struct Pe32 {
    /// The address that is relative to the image base of the beginning-of-data
    /// section when is is loaded into memory.
    pub base_of_data: u32,
    /// The preferred address of the first byte of image when loaded into
    /// memory; must be a multiple of 64k. DLL default is 0x1000_0000. Default
    /// for windows CE EXEs is 0x0001_0000. And default for Windows NT is
    /// 0x0040_0000
    pub image_base: u32,
    /// The alignment (in bytes) of sections when they are loaded into memory.
    /// It must be greater than or equal to FileAlignment. The default is
    /// the page size for the architecture.
    section_alignment: u32,
    /// The alignment factor(in bytes) that is used to align the raw data of
    /// sections in the image file. The value should be a power of 2, bigger
    /// than 512 and smaller than 64K, inclusive. Default is 512. If the
    /// SectionAlignment is less than the architecture's page size, then
    /// FileAlignment must match SectionAlignment.
    file_alignment: u32,
    /// The major version number of the required operating system.
    major_os_version: u16,
    /// The minor version number of the required operating system.
    minor_os_version: u16,
    /// the major version number of the image.
    major_image_version: u16,
    /// The minor version number of the image.
    minor_image_version: u16,
    /// the major version number of the subsystem.
    major_subsys_version: u16,
    /// The minor version number of the subsystem.
    minor_subsys_version: u16,
    /// Reserved, must be zero.
    win32_version_value: u32,
    /// The size of the image, including all headers, as the image is loaded in
    /// memory. It must be a multiple of `section_alignment`.
    size_of_image: u32,
    /// The combines size of an MS-DOS stub, PE header, and section headers
    /// rounded up to a multiple of `file_alignment`.
    size_of_headers: u32,
    /// The image file checkshum. The algorithm for computing the checksum is
    /// incorporated into IMAGHELP.dll.
    checksum: u32,
    /// The subsytem that is required to run this image.
    subsystem: u16,
    dll_characteristics: u16,
    /// The size of the stack to reserve. Only `size_of_stack_commit` is
    /// commited; the rest is made available one page at a time until the
    /// reserve size if reached.
    size_of_stack_reserve: u32,
    /// The size of the stack to commit.
    size_of_stack_commit: u32,
    /// The size of the heap to reserve. Only `size_of_heap_commit` is
    /// commited; the rest is made available one page at a time until the
    /// reserve size if reached.
    size_of_heap_reserve: u32,
    /// The size of the heap to commit.
    size_of_heap_commit: u32,
    /// Reserved, must be zero.
    loader_flags: u32,
    /// The number of data-directory entries in the remainder of the optional
    /// header. Each describes a location and size.
    number_of_rva_and_sizes: u32,
}

impl Pe32 {
    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let (base_of_data, bytes) = take_u32(bytes)?;
        let (image_base, bytes) = take_u32(bytes)?;
        let (section_alignment, bytes) = take_u32(bytes)?;
        let (file_alignment, bytes) = take_u32(bytes)?;
        let (major_os_version, bytes) = take_u16(bytes)?;
        let (minor_os_version, bytes) = take_u16(bytes)?;
        let (major_image_version, bytes) = take_u16(bytes)?;
        let (minor_image_version, bytes) = take_u16(bytes)?;
        let (major_subsys_version, bytes) = take_u16(bytes)?;
        let (minor_subsys_version, bytes) = take_u16(bytes)?;
        let (win32_version_value, bytes) = take_u32(bytes)?;
        let (size_of_image, bytes) = take_u32(bytes)?;
        let (size_of_headers, bytes) = take_u32(bytes)?;
        let (checksum, bytes) = take_u32(bytes)?;
        let (subsystem, bytes) = take_u16(bytes)?;
        let (dll_characteristics, bytes) = take_u16(bytes)?;
        let (size_of_stack_reserve, bytes) = take_u32(bytes)?;
        let (size_of_stack_commit, bytes) = take_u32(bytes)?;
        let (size_of_heap_reserve, bytes) = take_u32(bytes)?;
        let (size_of_heap_commit, bytes) = take_u32(bytes)?;
        let (loader_flags, bytes) = take_u32(bytes)?;
        let (number_of_rva_and_sizes, bytes) = take_u32(bytes)?;

        Ok(( Self {
            base_of_data, image_base, section_alignment, file_alignment,
            major_os_version, minor_os_version,
            major_image_version, minor_image_version,
            major_subsys_version, minor_subsys_version,
            win32_version_value, size_of_image, size_of_headers,
            checksum, subsystem, dll_characteristics,
            size_of_stack_reserve, size_of_stack_commit,
            size_of_heap_reserve, size_of_heap_commit,
            loader_flags, number_of_rva_and_sizes
        }, bytes))
    }
}

pub struct Pe64 {
    /// The preferred address of the first byte of image when loaded into
    /// memory; must be a multiple of 64k. DLL default is 0x1000_0000. Default
    /// for windows CE EXEs is 0x0001_0000. And default for Windows NT is
    /// 0x0040_0000
    pub image_base: u64,
    /// The alignment (in bytes) of sections when they are loaded into memory.
    /// It must be greater than or equal to FileAlignment. The default is
    /// the page size for the architecture.
    section_alignment: u32,
    /// The alignment factor(in bytes) that is used to align the raw data of
    /// sections in the image file. The value should be a power of 2, bigger
    /// than 512 and smaller than 64K, inclusive. Default is 512. If the
    /// SectionAlignment is less than the architecture's page size, then
    /// FileAlignment must match SectionAlignment.
    file_alignment: u32,
    /// The major version number of the required operating system.
    major_os_version: u16,
    /// The minor version number of the required operating system.
    minor_os_version: u16,
    /// the major version number of the image.
    major_image_version: u16,
    /// The minor version number of the image.
    minor_image_version: u16,
    /// the major version number of the subsystem.
    major_subsys_version: u16,
    /// The minor version number of the subsystem.
    minor_subsys_version: u16,
    /// Reserved, must be zero.
    win32_version_value: u32,
    /// The size of the image, including all headers, as the image is loaded in
    /// memory. It must be a multiple of `section_alignment`.
    size_of_image: u32,
    /// The combines size of an MS-DOS stub, PE header, and section headers
    /// rounded up to a multiple of `file_alignment`.
    size_of_headers: u32,
    /// The image file checkshum. The algorithm for computing the checksum is
    /// incorporated into IMAGHELP.dll.
    checksum: u32,
    /// The subsytem that is required to run this image.
    subsystem: u16,
    dll_characteristics: u16,
    /// The size of the stack to reserve. Only `size_of_stack_commit` is
    /// commited; the rest is made available one page at a time until the
    /// reserve size if reached.
    size_of_stack_reserve: u64,
    /// The size of the stack to commit.
    size_of_stack_commit: u64,
    /// The size of the heap to reserve. Only `size_of_heap_commit` is
    /// commited; the rest is made available one page at a time until the
    /// reserve size if reached.
    size_of_heap_reserve: u64,
    /// The size of the heap to commit.
    size_of_heap_commit: u64,
    /// Reserved, must be zero.
    loader_flags: u32,
    /// The number of data-directory entries in the remainder of the optional
    /// header. Each describes a location and size.
    number_of_rva_and_sizes: u32,
}

impl Pe64 {
    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let (image_base, bytes) = take_u64(bytes)?;
        let (section_alignment, bytes) = take_u32(bytes)?;
        let (file_alignment, bytes) = take_u32(bytes)?;
        let (major_os_version, bytes) = take_u16(bytes)?;
        let (minor_os_version, bytes) = take_u16(bytes)?;
        let (major_image_version, bytes) = take_u16(bytes)?;
        let (minor_image_version, bytes) = take_u16(bytes)?;
        let (major_subsys_version, bytes) = take_u16(bytes)?;
        let (minor_subsys_version, bytes) = take_u16(bytes)?;
        let (win32_version_value, bytes) = take_u32(bytes)?;
        let (size_of_image, bytes) = take_u32(bytes)?;
        let (size_of_headers, bytes) = take_u32(bytes)?;
        let (checksum, bytes) = take_u32(bytes)?;
        let (subsystem, bytes) = take_u16(bytes)?;
        let (dll_characteristics, bytes) = take_u16(bytes)?;
        let (size_of_stack_reserve, bytes) = take_u64(bytes)?;
        let (size_of_stack_commit, bytes) = take_u64(bytes)?;
        let (size_of_heap_reserve, bytes) = take_u64(bytes)?;
        let (size_of_heap_commit, bytes) = take_u64(bytes)?;
        let (loader_flags, bytes) = take_u32(bytes)?;
        let (number_of_rva_and_sizes, bytes) = take_u32(bytes)?;

        Ok(( Self {
            image_base, section_alignment, file_alignment,
            major_os_version, minor_os_version,
            major_image_version, minor_image_version,
            major_subsys_version, minor_subsys_version,
            win32_version_value, size_of_image, size_of_headers,
            checksum, subsystem, dll_characteristics,
            size_of_stack_reserve, size_of_stack_commit,
            size_of_heap_reserve, size_of_heap_commit,
            loader_flags, number_of_rva_and_sizes
        }, bytes))
    }
}
