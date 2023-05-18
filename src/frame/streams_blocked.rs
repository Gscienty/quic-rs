/// STREAM_BLOCKED 帧
///
/// 当发送方由于其对等方设置的最大流限制而无法打开流时，发送该帧.
///
/// 帧结构如下:
/// STREAM_BLOCKED Frame {
///     Type (i) = 0x16..0x17,
///     Maximum Streams (i),
/// }
pub struct StreamsBlocked {
    maximum_streams: u64,
}
