/// STREAM_DATA_BLOCKED 帧
///
/// 用于流量控制算法的调整输入，作用在具体的流上.
/// 当发送方由于流级流量控制而无法发送数据时，应发送该帧.
///
/// 帧结构如下:
/// STREAM_DATA_BLOCKED Frame {
///     Type (i) = 0x15,
///     Stream ID (i),
///     Maximum Stream Data (i),
/// }
pub struct StreamDataBlockedFrame {}
