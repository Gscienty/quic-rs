mod byteorder;
mod varint;

pub(crate) use byteorder::{from_bigendian_bytes, to_bigendian_bytes};
pub(crate) use varint::{read_varint, write_varint};

#[cfg(test)]
mod varint_test;
