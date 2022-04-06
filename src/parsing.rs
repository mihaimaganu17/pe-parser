use crate::{
    error::{Result, PeError}
};

/// Helper function that consumes 1 byte(u8) from `bytes` and returns a u16
/// as result. In case `bytes` buffer is too small it returns an error.
pub fn take_u8(bytes: &mut Vec<u8>) -> Result<u8> {
    if bytes.len() < 1 {
        return Err(PeError::BufferTooSmall)
    }

    Ok(u8::from_le_bytes(
        bytes.drain(..1).as_slice().try_into()?))
}

/// Helper function that consumes 2 bytes(u8) from `bytes` and returns a u16
/// as result. In case `bytes` buffer is too small it returns an error.
pub fn take_u16(bytes: &mut Vec<u8>) -> Result<u16> {
    if bytes.len() < 2 {
        return Err(PeError::BufferTooSmall)
    }

    Ok(u16::from_le_bytes(
        bytes.drain(..2).as_slice().try_into()?))
}

/// Helper function that consumes 4 bytes(u8) from `bytes` and returns a u32
/// as result. In case `bytes` buffer is too small it returns an error.
pub fn take_u32(bytes: &mut Vec<u8>) -> Result<u32> {
    if bytes.len() < 4 {
        return Err(PeError::BufferTooSmall)
    }

    Ok(u32::from_le_bytes(
        bytes.drain(..4).as_slice().try_into()?))
}

/// Helper function that consumes 8 bytes(u8) from `bytes` and returns a u64
/// as result. In case `bytes` buffer is too small it returns an error.
pub fn take_u64(bytes: &mut Vec<u8>) -> Result<u64> {
    if bytes.len() < 8 {
        return Err(PeError::BufferTooSmall)
    }

    Ok(u64::from_le_bytes(
        bytes.drain(..8).as_slice().try_into()?))
}
