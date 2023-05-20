use std::{
    assert_eq,
    io::{BufReader, BufWriter, Cursor},
};

use super::varint;

#[test]
fn test_varint() {
    let verify = |n: u64| {
        let mut buf = [0; 16];
        {
            let mut writer = BufWriter::new(Cursor::new(&mut buf[..]));
            assert!(varint::write_varint(n, &mut writer).is_ok());
        }

        let mut reader = BufReader::new(Cursor::new(&buf));

        varint::read_varint(&mut reader).unwrap()
    };

    assert_eq!(verify(52), varint::Varint { value: 52, size: 1 });
    assert_eq!(
        verify(12138),
        varint::Varint {
            value: 12138,
            size: 2
        }
    );
    assert_eq!(
        verify(973741823),
        varint::Varint {
            value: 973741823,
            size: 4
        }
    );
    assert_eq!(
        verify(223344556677),
        varint::Varint {
            value: 223344556677,
            size: 8
        }
    );
}
