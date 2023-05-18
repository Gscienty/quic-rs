use std::io::{self, Read, Write};

pub(crate) fn read_varint_u64(r: &mut dyn Read) -> Result<u64, io::Error> {
    let mut first_buf: [u8; 1] = [0; 1];
    r.read_exact(&mut first_buf)?;

    match first_buf[0] & 0b11_000000 {
        0b00_000000 => Ok(first_buf[0] as u64),
        0b01_000000 => {
            let mut remain_buf: [u8; 1] = [0; 1];
            r.read_exact(&mut remain_buf)?;

            Ok((((first_buf[0] & 0b00_111111) as u64) << 8) | (remain_buf[0] as u64))
        }
        0b10_000000 => {
            let mut remain_buf: [u8; 3] = [0; 3];
            r.read_exact(&mut remain_buf)?;

            Ok((((first_buf[0] & 0b00_111111) as u64) << 24)
                | ((remain_buf[0] as u64) << 16)
                | ((remain_buf[1] as u64) << 8)
                | (remain_buf[2] as u64))
        }
        0b11_000000 => {
            let mut remain_buf: [u8; 7] = [0; 7];
            r.read_exact(&mut remain_buf)?;

            Ok((((first_buf[0] & 0b00_111111) as u64) << 56)
                | ((remain_buf[0] as u64) << 48)
                | ((remain_buf[1] as u64) << 40)
                | ((remain_buf[2] as u64) << 32)
                | ((remain_buf[3] as u64) << 24)
                | ((remain_buf[4] as u64) << 16)
                | ((remain_buf[5] as u64) << 8)
                | (remain_buf[6] as u64))
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "unexcepted variable-length 2MSB",
        )),
    }
}

pub(crate) fn write_varint_u64(n: u64, w: &mut dyn Write) -> Result<(), io::Error> {
    match n {
        0..=63 => w.write_all(&[n as u8]),
        64..=16383 => w.write_all(&[0b01_000000 | (((n >> 8) & 0xff) as u8), ((n & 0xff) as u8)]),
        16384..=1073741823 => w.write_all(&[
            0b10_000000 | (((n >> 24) & 0xff) as u8),
            (((n >> 16) & 0xff) as u8),
            (((n >> 8) & 0xff) as u8),
            ((n & 0xff) as u8),
        ]),
        1073741824..=4611686018427387903 => w.write_all(&[
            0b11_000000 | (((n >> 56) & 0xff) as u8),
            (((n >> 48) & 0xff) as u8),
            (((n >> 40) & 0xff) as u8),
            (((n >> 32) & 0xff) as u8),
            (((n >> 24) & 0xff) as u8),
            (((n >> 16) & 0xff) as u8),
            (((n >> 8) & 0xff) as u8),
            ((n & 0xff) as u8),
        ]),
        _ => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "unexcepted variable-length number",
        )),
    }
}
