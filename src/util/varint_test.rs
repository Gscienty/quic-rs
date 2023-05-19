use std::{
    assert_eq,
    io::{BufReader, BufWriter, Cursor},
};

use super::varint;

#[test]
fn test_varint() {
    let verify = |n: usize| {
        let mut buf = [0; 16];
        {
            let mut writer = BufWriter::new(Cursor::new(&mut buf[..]));
            assert!(varint::write_varint(n, &mut writer).is_ok());
        }

        let mut reader = BufReader::new(Cursor::new(&buf));
        varint::read_varint::<u64>(&mut reader).unwrap()
    };

    assert_eq!(verify(52), 52);
    assert_eq!(verify(12138), 12138);
    assert_eq!(verify(973741823), 973741823);
    assert_eq!(verify(223344556677), 223344556677)
}
