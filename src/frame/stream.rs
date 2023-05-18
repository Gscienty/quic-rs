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
pub struct StreamFrame {}
