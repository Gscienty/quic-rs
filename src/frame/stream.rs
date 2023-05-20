use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// STREAM 帧
///
/// 创建一个流，并携带流数据.
///
/// 帧结构如下:
/// STREAM Frame {
///     Type (i) = 0x08..0x0f,
///     Stream ID (i),
///     [Offset (i)],
///     [Length (i)],
///     Stream Data (..),
/// }
pub struct StreamFrame {
    off_flag: bool,
    len_flag: bool,
    fin_flag: bool,

    stream_id: u64,
    offset: usize,
    data: Vec<u8>,
}

impl StreamFrame {
    pub fn new(off_flag: bool, len_flag: bool, fin_flag: bool) -> Self {
        Self {
            off_flag,
            len_flag,
            fin_flag,

            stream_id: 0,
            offset: 0,
            data: Vec::new(),
        }
    }
}

impl Serializer for StreamFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::Stream {
            off_flag: self.off_flag,
            len_flag: self.len_flag,
            fin_flag: self.fin_flag,
        }
        .into()])?;

        payload_size += util::write_varint(self.stream_id, w)?;
        if self.off_flag {
            payload_size += util::write_varint(self.offset as u64, w)?;
        }
        if self.len_flag {
            payload_size += util::write_varint(self.data.len() as u64, w)?;
        }

        w.write_all(&self.data)?;
        payload_size += self.data.len();

        Ok(payload_size)
    }
}

impl Deserializer for StreamFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let stream_id = util::read_varint(r)?;
        self.stream_id = stream_id.value;
        payload_size += stream_id.size;

        if self.off_flag {
            let offset = util::read_varint(r)?;
            self.offset = offset.value as usize;
            payload_size += offset.size;
        }

        // 如果 `len_flag` == false, 则该 stream 帧的 data 部分应该读取到数据包的结尾.
        if self.len_flag {
            let data_len = util::read_varint(r)?;
            self.data.resize(data_len.value as usize, 0);
            payload_size += data_len.size;

            r.read(&mut self.data)?;
            payload_size += self.data.len();
        } else {
            payload_size += r.read_to_end(&mut self.data)?;
        }

        Ok(payload_size)
    }
}
