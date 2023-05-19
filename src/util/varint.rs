use std::{
    io::{self, Read, Write},
    ops,
};

pub(crate) fn read_varint<T>(r: &mut dyn Read) -> Result<T, io::Error>
where
    T: ops::BitOr<Output = T> + From<u8> + ops::Shl<Output = T>,
{
    let mut first_byte: [u8; 1] = [0; 1];
    r.read_exact(&mut first_byte)?;

    let first_part = T::from(first_byte[0] & 0b00_111111);
    match first_byte[0] & 0b11_000000 {
        0b00_000000 => Ok(T::from(first_byte[0])),
        0b01_000000 => {
            let mut remain_bytes: [u8; 1] = [0; 1];
            r.read_exact(&mut remain_bytes)?;

            Ok(first_part.shl(8.into()) | T::from(remain_bytes[0]).into())
        }
        0b10_000000 => {
            let mut remain_bytes: [u8; 3] = [0; 3];
            r.read_exact(&mut remain_bytes)?;

            Ok(first_part.shl(24.into())
                | T::from(remain_bytes[0]).shl(16.into())
                | T::from(remain_bytes[1]).shl(8.into())
                | T::from(remain_bytes[2]))
        }
        0b11_000000 => {
            let mut remain_bytes: [u8; 7] = [0; 7];
            r.read_exact(&mut remain_bytes)?;

            Ok(first_part.shl(56.into())
                | T::from(remain_bytes[0]).shl(48.into())
                | T::from(remain_bytes[1]).shl(40.into())
                | T::from(remain_bytes[2]).shl(32.into())
                | T::from(remain_bytes[3]).shl(24.into())
                | T::from(remain_bytes[4]).shl(16.into())
                | T::from(remain_bytes[5]).shl(8.into())
                | T::from(remain_bytes[6]))
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "unexcepted variable-length 2MSB",
        )),
    }
}

pub(crate) fn write_varint<T>(n: T, w: &mut dyn Write) -> Result<(), io::Error>
where
    T: ops::Shr<Output = T> + Into<usize> + Copy + PartialOrd,
{
    let n = n.into();
    match n {
        0..=63 => w.write_all(&[n as u8]),
        64..=16383 => w.write_all(&[(n >> 8) as u8 | 0b01_000000, n as u8]),
        16384..=1073741823 => w.write_all(&[
            (n >> 24) as u8 | 0b10_000000,
            (n >> 16) as u8,
            (n >> 8) as u8,
            n as u8,
        ]),
        1073741824..=4611686018427387903 => w.write_all(&[
            (n >> 56) as u8 | 0b11_000000,
            (n >> 48) as u8,
            (n >> 40) as u8,
            (n >> 32) as u8,
            (n >> 24) as u8,
            (n >> 16) as u8,
            (n >> 8) as u8,
            n as u8,
        ]),
        _ => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "unexcepted variable-length number",
        )),
    }
}
