/// PATH_CHALLENGE 帧
///
/// 用于检查与通信对方的可达性, 并在连接迁移期间进行路径验证.
///
/// 帧结构如下:
/// PATH_CHALLENGE Frame {
///     Type (i) = 0x1a,
///     Data (64),
/// }
pub struct PathChallengeFrame {}
