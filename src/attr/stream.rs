pub(crate) type StreamID = u64;

pub(crate) trait StreamIDGetter {
    /// 获取设置帧的 Stream 标识
    ///
    /// # Returns
    /// 返回 Stream 标识
    fn get_stream_id(&self) -> StreamID;
}

pub(crate) trait StreamIDSetter {
    /// 设置帧的 Stream 标识
    ///
    /// # Arguments
    /// `stream_id` - Stream 标识
    fn set_stream_id(&mut self, stream_id: StreamID);
}

pub(crate) trait StreamDataGetter {
    /// 获取数据流中承载的数据
    ///
    /// # Returns
    /// `0` - 数据流帧中承载数据的偏移量
    /// `1` - 数据流帧中的数据
    fn get_data(&self) -> (usize, &[u8]);
}

pub(crate) trait StreamDataSetter {
    /// 设置数据流帧承载的数据
    ///
    /// # Arguments
    /// `offset` - 数据流帧中数据的偏移量
    /// `data` - 数据流帧中本次承载的数据
    fn set_data(&mut self, offset: usize, data: &[u8]);
}
