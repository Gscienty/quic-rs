/// ACK 帧
///
/// 数据接收方会发送 ACK 帧给数据发送方，用于告知数据发送方已经有多少数据包
/// 已经被接收并处理.
/// 一个 ACK 帧至少包含一个 ACK Range；用于标识已经被接收并处理的数据包.
///
/// 在 QUIC 中，确认是不可被撤销的，一旦确认，数据包就会保持确认状态.
///
/// 在不同的数据包编号空间的数据包可以使用相同的数值进行标识.
/// 因此对于数据包的确认，需要同时指示数据包编号和数据包编号空间. ACK 帧仅
/// 确认与 ACK 帧所在的数据包相同编号空间中的数据包编号.
///
/// [Notice] 版本协商和重试数据包无法被确认
///
/// ACK 帧可以使用 0x02 或 0x03 两种帧类型字节表示.
/// 0x02: 不包含 ECM Counts 字段;
/// 0x03: 需包含 ECM Counts 字段.
/// ECM Counts 字段, 即 ACK 帧需包含到目前为止
/// 在连接上接收到的具有关联 ECN 标记的 QUIC 数据包的累积计数.
///
/// 帧结构如下:
/// ACK Frame {
///     Type (i) = 0x02..0x03,
///     Largest Acknowledged (i),
///     ACK Delay (i),
///     ACK Range Count (i),
///     First ACK Range (i),
///     ACK Range (..) ...,
///     [ECN Counts (...)]
/// }
pub struct ACKFrame {
    largest: u64,
    delay: u64,
    first_range: u64,
    ranges: Vec<ACKRange>,
    ecn: Option<ECNCounts>,
}

pub struct ACKRange {
    gap: u64,
    length: u64,
}

pub struct ECNCounts {
    ect0: u64,
    ect1: u64,
    ecn_ce: u64,
}
