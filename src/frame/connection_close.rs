/// CONNECTION_CLOSE 帧
///
/// 用于关闭连接.
///
/// 如果存在未明确关闭的打开流, 则在关闭连接时关闭它们.
///
/// 帧结构如下:
/// CONNECTION_CLOSE Frame {
///     Type (i) = 0x1c..0x1d,
///     Error Code (i),
///     [Frame Type (i)],
///     Reason Phrase Length (i),
///     Reason Phrase (..),
/// }
pub struct ConnectionCloseFrame {}
