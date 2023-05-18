/// DATA_BLOCKED 帧
///
/// 用于流量控制算法的调整输入.
/// 当发送方由于连接级流量控制而无法发送数据时，应发送 DATA_BLOCKED 帧.
///
/// 帧结构如下:
/// DATA_BLOCKED Frame {
///     Type (i) = 0x14,
///     Maximum Data (i),
/// }
pub struct DataBlockedFrame {
    maximum_data: u64,
}
