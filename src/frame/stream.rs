use crate::{
    attr::{
        Deserializer, Serializer, StreamDataGetter, StreamDataSetter, StreamID, StreamIDGetter,
        StreamIDSetter,
    },
    util,
};

use super::types::FrameType;

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

    stream_id: StreamID,
    offset: usize,
    data: Vec<u8>,
}

impl StreamFrame {
    /// 构造一个 STREAM 帧
    ///
    /// # Arguments
    /// `off_flag` - 标识 STREAM 帧中是否包含数据偏移量字段
    /// `len_flag` - 标识 STREAM 帧中是否包含数据长度字段
    /// `fin_flag` - 标识 STREAM 帧是否为结束
    ///
    /// # Returns
    /// 返回一个 STREAM 帧
    pub(crate) fn new(off_flag: bool, len_flag: bool, fin_flag: bool) -> Self {
        Self {
            off_flag,
            len_flag,
            fin_flag,

            stream_id: 0,
            offset: 0,
            data: Vec::new(),
        }
    }

    /// 获取 FIN 标识
    ///
    /// # Returns
    /// 返回 FIN 标识
    pub(crate) const fn get_fin_flag(&self) -> bool {
        self.fin_flag
    }

    /// 设置 FIN 标识
    ///
    /// # Arguments
    /// `fin_flag` - FIN 标识
    pub(crate) fn set_fin_flag(&mut self, fin_flag: bool) {
        self.fin_flag = fin_flag
    }

    /// 获取 LEN 标识
    ///
    /// # Returns
    /// 返回 LEN 标识
    pub(crate) const fn get_len_flag(&self) -> bool {
        self.len_flag
    }

    /// 设置 LEN 标识
    ///
    /// # Arguments
    /// `len_flag` - LEN 标识
    pub(crate) fn set_len_flag(&mut self, len_flag: bool) {
        self.len_flag = len_flag
    }

    /// 获取 OFF 标识
    ///
    /// # Returns
    /// 返回 OFF 标识
    pub(crate) const fn get_off_flag(&self) -> bool {
        self.off_flag
    }

    /// 设置 OFF 标识
    ///
    /// # Arguments
    /// `off_flag` - OFF 标识
    pub(crate) fn set_off_flag(&mut self, off_flag: bool) {
        self.off_flag = off_flag
    }
}

impl StreamIDGetter for StreamFrame {
    fn get_stream_id(&self) -> StreamID {
        self.stream_id
    }
}

impl StreamIDSetter for StreamFrame {
    fn set_stream_id(&mut self, stream_id: StreamID) {
        self.stream_id = stream_id
    }
}

impl StreamDataGetter for StreamFrame {
    fn get_data(&self) -> (usize, &[u8]) {
        (self.offset, &self.data)
    }
}

impl StreamDataSetter for StreamFrame {
    fn set_data(&mut self, offset: usize, data: &[u8]) {
        self.offset = offset;
        self.data.extend_from_slice(data);
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
