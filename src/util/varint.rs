use std::io::{self, Read, Write};

use super::byteorder;

#[derive(Debug, PartialEq)]
pub(crate) struct Varint {
    pub(crate) value: u64,
    pub(crate) size: usize,
}

pub(crate) fn read_varint(r: &mut dyn Read) -> Result<Varint, io::Error> {
    let mut bytes = [0; 8];
    r.read_exact(&mut bytes[..1])?;

    let prefix = (bytes[0] & 0b11_000000) >> 6;
    bytes[0] &= 0b00_111111;

    match prefix {
        0b00 => Ok(Varint {
            value: byteorder::from_bigendian_bytes::<1>(&bytes[..1]),
            size: 1,
        }),
        0b01 => {
            r.read_exact(&mut bytes[1..2])?;
            Ok(Varint {
                value: byteorder::from_bigendian_bytes::<2>(&bytes[..2]),
                size: 2,
            })
        }
        0b10 => {
            r.read_exact(&mut bytes[1..4])?;
            Ok(Varint {
                value: byteorder::from_bigendian_bytes::<4>(&bytes[..4]),
                size: 4,
            })
        }
        0b11 => {
            r.read_exact(&mut bytes[1..8])?;
            Ok(Varint {
                value: byteorder::from_bigendian_bytes::<8>(&bytes[..8]),
                size: 8,
            })
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "unexcepted variable-length 2MSB",
        )),
    }
}

pub(crate) fn write_varint(n: u64, w: &mut dyn Write) -> Result<usize, io::Error> {
    match n {
        0..=63 => {
            w.write_all(&byteorder::to_bigendian_bytes::<_, 1>(n))?;

            Ok(1)
        }
        64..=16383 => {
            let mut buf = byteorder::to_bigendian_bytes::<_, 2>(n);
            buf[0] |= 0b01_000000;

            w.write_all(&buf)?;

            Ok(2)
        }
        16384..=1073741823 => {
            let mut buf = byteorder::to_bigendian_bytes::<_, 4>(n);
            buf[0] |= 0b10_000000;

            w.write_all(&buf)?;

            Ok(4)
        }
        1073741824..=4611686018427387903 => {
            let mut buf = byteorder::to_bigendian_bytes::<_, 8>(n);
            buf[0] |= 0b11_000000;

            w.write_all(&buf)?;
            Ok(8)
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "unexcepted variable-length number",
        )),
    }
}
