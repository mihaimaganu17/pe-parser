use crate::{
    error::{Result, PeError}
};

/// Helper function that consumes 1 byte(u8) from `bytes` and returns a u16
/// as result. In case `bytes` buffer is too small it returns an error.
pub fn take_u8(bytes: &[u8]) -> Result<(u8, &[u8])> {
    if bytes.len() < 1 {
        return Err(PeError::BufferTooSmall)
    }

    let (left, right) = bytes.split_at(1usize);

    Ok((u8::from_le_bytes(left.try_into()?), right))
}

/// Helper function that consumes 1 byte(u8) from `bytes` and returns a u16
/// as result. In case `bytes` buffer is too small it returns an error.
pub fn take_u16(bytes: &[u8]) -> Result<(u16, &[u8])> {
    if bytes.len() < 1 {
        return Err(PeError::BufferTooSmall)
    }

    let (left, right) = bytes.split_at(2usize);

    Ok((u16::from_le_bytes(left.try_into()?), right))
}

/// Helper function that consumes 1 byte(u8) from `bytes` and returns a u16
/// as result. In case `bytes` buffer is too small it returns an error.
pub fn take_u32(bytes: &[u8]) -> Result<(u32, &[u8])> {
    if bytes.len() < 4 {
        return Err(PeError::BufferTooSmall)
    }

    let (left, right) = bytes.split_at(4usize);

    Ok((u32::from_le_bytes(left.try_into()?), right))
}

/// Helper function that consumes 1 byte(u8) from `bytes` and returns a u16
/// as result. In case `bytes` buffer is too small it returns an error.
pub fn take_u64(bytes: &[u8]) -> Result<(u64, &[u8])> {
    if bytes.len() < 8 {
        return Err(PeError::BufferTooSmall)
    }

    let (left, right) = bytes.split_at(8usize);

    Ok((u64::from_le_bytes(left.try_into()?), right))
}

