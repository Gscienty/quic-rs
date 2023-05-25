use crate::{
    attr::{Deserializer, Serializer},
    util,
};

use super::types::FrameType;

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
pub(crate) struct ConnectionCloseFrame {
    /// 是否是系统级（QUIC层级）报错
    sys_err: bool,

    /// 错误代码
    error_code: u64,

    /// 触发错误的帧类型；当帧类型未知时，使用值为0.
    frame_type: u64,

    /// 错误原因
    reason: String,
}

impl ConnectionCloseFrame {
    /// 构造一个 CONNECTION_CLOSE 帧
    ///
    /// # Returns
    /// 返回一个 CONNECTION_CLOSE 帧
    pub(crate) fn new(sys_err: bool) -> Self {
        Self {
            sys_err,
            error_code: 0,
            frame_type: 0,
            reason: String::new(),
        }
    }

    /// 获取是否是系统级（QUIC 层级）报错
    ///
    /// # Returns
    /// 返回是否是系统级（QUIC层级）报错
    #[inline(always)]
    pub(crate) const fn get_sys_err(&self) -> bool {
        self.sys_err
    }

    /// 设置是否是系统级（QUIC 层级）报错
    ///
    /// # Arguments
    /// `sys_err` - 是否是系统级（QUIC层级）报错
    #[inline(always)]
    pub(crate) fn set_sys_err(&mut self, sys_err: bool) {
        self.sys_err = sys_err
    }

    /// 获取错误代码
    ///
    /// # Returns
    /// 返回错误代码
    #[inline(always)]
    pub(crate) const fn get_error_code(&self) -> u64 {
        self.error_code
    }

    /// 设置错误代码
    ///
    /// # Arguments
    /// `error_code` - 错误代码
    #[inline(always)]
    pub(crate) fn set_error_code(&mut self, error_code: u64) {
        self.error_code = error_code
    }

    /// 获取触发错误的帧类型
    ///
    /// # Returns
    /// 返回帧类型
    #[inline(always)]
    pub(crate) const fn get_frame_type(&self) -> u64 {
        self.frame_type
    }

    /// 设置触发错误的帧类型
    ///
    /// # Arguments
    /// `frame_type` - 帧类型
    #[inline(always)]
    pub(crate) fn set_frame_type(&mut self, frame_type: u64) {
        self.frame_type = frame_type
    }

    /// 获取错误原因
    ///
    /// # Returns
    /// 返回错误原因
    #[inline(always)]
    pub(crate) fn get_reason(&self) -> &str {
        &self.reason
    }

    /// 设置错误原因
    ///
    /// # Arguments
    /// `reason` - 错误原因
    #[inline(always)]
    pub(crate) fn set_reason(&mut self, reason: &str) {
        self.reason = String::from(reason)
    }
}

impl Serializer for ConnectionCloseFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::ConnectionClose {
            sys_err: self.sys_err,
        }
        .into()])?;

        payload_size += util::write_varint(self.error_code, w)?;

        if self.sys_err {
            payload_size += util::write_varint(self.frame_type, w)?;
        }

        payload_size += util::write_varint(self.reason.len() as u64, w)?;

        w.write_all(self.reason.as_bytes())?;
        payload_size += self.reason.len();

        Ok(payload_size)
    }
}

impl Deserializer for ConnectionCloseFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let error_code = util::read_varint(r)?;
        self.error_code = error_code.value;
        payload_size += error_code.size;

        if self.sys_err {
            let frame_type = util::read_varint(r)?;
            self.frame_type = frame_type.value;
            payload_size += frame_type.size;
        }

        let len = util::read_varint(r)?;
        let mut reason = Vec::<u8>::with_capacity(len.value as usize);
        reason.resize(len.value as usize, 0);
        payload_size += len.size;

        r.read_exact(&mut reason)?;
        self.reason = String::from_utf8(reason).or(Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid input",
        )))?;
        payload_size += self.reason.len();

        Ok(payload_size)
    }
}
