use std::fmt::Debug;
use std::io::{Read, Write, Result};

use byteorder::{ByteOrder, LittleEndian};

use types::*;

/// This converts an IO encoding error (and logs when in error) into an EncodingResult
pub fn process_encode_io_result(result: Result<usize>) -> EncodingResult<usize> {
    if result.is_err() {
        debug!("Encoding error - {:?}", result.unwrap_err());
        Err(&BAD_ENCODING_ERROR)
    } else {
        Ok(result.unwrap())
    }
}

/// This converts an IO encoding error (and logs when in error) into an EncodingResult
pub fn process_decode_io_result<T>(result: Result<T>) -> EncodingResult<T> where T: Debug {
    if result.is_err() {
        debug!("Decoding error - {:?}", result.unwrap_err());
        Err(&BAD_DECODING_ERROR)
    } else {
        Ok(result.unwrap())
    }
}

/// Calculates the length in bytes of an array of encoded type
pub fn byte_len_array<T: BinaryEncoder<T>>(values: &Option<Vec<T>>) -> usize {
    let mut size = 4;
    if let &Some(ref values) = values {
        for value in values.iter() {
            size += value.byte_len();
        }
    }
    size
}

/// Write an array of the encoded type to stream, preserving distinction between null array and empty array
pub fn write_array<S: Write, T: BinaryEncoder<T>>(stream: &mut S, values: &Option<Vec<T>>) -> EncodingResult<usize> {
    let mut size = 0;
    if let &Some(ref values) = values {
        size += write_i32(stream, values.len() as i32)?;
        for value in values.iter() {
            size += value.encode(stream)?;
        }
    } else {
        size += write_i32(stream, -1)?;
    }
    Ok(size)
}

/// Reads an array of the encoded type from a stream, preserving distinction between null array and empty array
pub fn read_array<S: Read, T: BinaryEncoder<T>>(stream: &mut S) -> EncodingResult<Option<Vec<T>>> {
    let len = read_i32(stream)?;
    if len == -1 {
        Ok(None)
    } else {
        let mut values: Vec<T> = Vec::new();
        for _ in 0..len {
            values.push(T::decode(stream)?);
        }
        Ok(Some(values))
    }
}

pub fn write_u8(stream: &mut Write, value: u8) -> EncodingResult<usize> {
    let buf: [u8; 1] = [value];
    process_encode_io_result(stream.write(&buf))
}

pub fn write_i16(stream: &mut Write, value: i16) -> EncodingResult<usize> {
    let mut buf = [0u8; 2];
    LittleEndian::write_i16(&mut buf, value);
    process_encode_io_result(stream.write(&buf))
}

pub fn write_u16(stream: &mut Write, value: u16) -> EncodingResult<usize> {
    let mut buf = [0u8; 2];
    LittleEndian::write_u16(&mut buf, value);
    process_encode_io_result(stream.write(&buf))
}

pub fn write_i32(stream: &mut Write, value: i32) -> EncodingResult<usize> {
    let mut buf = [0u8; 4];
    LittleEndian::write_i32(&mut buf, value);
    process_encode_io_result(stream.write(&buf))
}

pub fn write_u32(stream: &mut Write, value: u32) -> EncodingResult<usize> {
    let mut buf = [0u8; 4];
    LittleEndian::write_u32(&mut buf, value);
    process_encode_io_result(stream.write(&buf))
}

pub fn write_i64(stream: &mut Write, value: i64) -> EncodingResult<usize> {
    let mut buf = [0u8; 8];
    LittleEndian::write_i64(&mut buf, value);
    process_encode_io_result(stream.write(&buf))
}

pub fn write_u64(stream: &mut Write, value: u64) -> EncodingResult<usize> {
    let mut buf = [0u8; 8];
    LittleEndian::write_u64(&mut buf, value);
    process_encode_io_result(stream.write(&buf))
}

pub fn write_f32(stream: &mut Write, value: f32) -> EncodingResult<usize> {
    let mut buf = [0u8; 4];
    LittleEndian::write_f32(&mut buf, value);
    process_encode_io_result(stream.write(&buf))
}

pub fn write_f64(stream: &mut Write, value: f64) -> EncodingResult<usize> {
    let mut buf = [0u8; 8];
    LittleEndian::write_f64(&mut buf, value);
    process_encode_io_result(stream.write(&buf))
}

pub fn read_bytes(stream: &mut Read, buf: &mut [u8]) -> EncodingResult<usize> {
    let result = stream.read_exact(buf);
    let _ = process_decode_io_result(result)?;
    Ok(buf.len())
}

pub fn read_u8(stream: &mut Read) -> EncodingResult<u8> {
    let mut buf = [0u8];
    let result = stream.read_exact(&mut buf);
    let _ = process_decode_io_result(result)?;
    Ok(buf[0])
}

pub fn read_i16(stream: &mut Read) -> EncodingResult<i16> {
    let mut buf = [0u8; 2];
    let result = stream.read_exact(&mut buf);
    let _ = process_decode_io_result(result)?;
    Ok(LittleEndian::read_i16(&buf))
}

pub fn read_u16(stream: &mut Read) -> EncodingResult<u16> {
    let mut buf = [0u8; 2];
    let result = stream.read_exact(&mut buf);
    let _ = process_decode_io_result(result)?;
    Ok(LittleEndian::read_u16(&buf))
}

pub fn read_i32(stream: &mut Read) -> EncodingResult<i32> {
    let mut buf = [0u8; 4];
    let result = stream.read_exact(&mut buf);
    let _ = process_decode_io_result(result)?;
    Ok(LittleEndian::read_i32(&buf))
}

pub fn read_u32(stream: &mut Read) -> EncodingResult<u32> {
    let mut buf = [0u8; 4];
    let result = stream.read_exact(&mut buf);
    let _ = process_decode_io_result(result)?;
    Ok(LittleEndian::read_u32(&buf))
}

pub fn read_i64(stream: &mut Read) -> EncodingResult<i64> {
    let mut buf = [0u8; 8];
    let result = stream.read_exact(&mut buf);
    let _ = process_decode_io_result(result)?;
    Ok(LittleEndian::read_i64(&buf))
}

pub fn read_u64(stream: &mut Read) -> EncodingResult<u64> {
    let mut buf = [0u8; 8];
    let result = stream.read_exact(&mut buf);
    let _ = process_decode_io_result(result)?;
    Ok(LittleEndian::read_u64(&buf))
}

pub fn read_f32(stream: &mut Read) -> EncodingResult<f32> {
    let mut buf = [0u8; 4];
    let result = stream.read_exact(&mut buf);
    let _ = process_decode_io_result(result)?;
    Ok(LittleEndian::read_f32(&buf))
}

pub fn read_f64(stream: &mut Read) -> EncodingResult<f64> {
    let mut buf = [0u8; 8];
    let result = stream.read_exact(&mut buf);
    let _ = process_decode_io_result(result)?;
    Ok(LittleEndian::read_f64(&buf))
}
