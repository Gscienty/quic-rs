#[cfg(target_endian = "little")]
pub(crate) fn to_bigendian_bytes<T, const S: usize>(n: T) -> [u8; S] {
    let mut ret = [0; S];

    unsafe {
        let n_bytes = &n as *const T as *const u8;
        for i in 0..S {
            ret[i] = *n_bytes.offset((S - 1 - i) as isize);
        }
    }

    ret
}

#[cfg(target_endian = "little")]
pub(crate) fn from_bigendian_bytes<const S: usize>(bytes: &[u8]) -> u64 {
    let mut ret = 0;

    unsafe {
        let ret_bytes = &mut ret as *mut u64 as *mut u8;
        for i in 0..S {
            *ret_bytes.offset(i as isize) = bytes[S - 1 - i];
        }
    }

    ret
}
