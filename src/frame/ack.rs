use crate::{attr::PacketNumber, util};

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

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
pub(crate) struct ACKFrame {
    /// 对方确认的最大数据包编号
    /// 通常这是对等方在生成ACK帧之前接收到的最大数据包编号
    largest: PacketNumber,

    /// 以微妙为单位的延迟
    delay: u64,

    /// 正在确认的最大确认数据包之前的连续数据包数量
    first_range: usize,

    /// 交替出现未被确认的范围（Gap）和被确认的范围（Ack Range）
    ranges: Vec<ACKRange>,

    /// ECN Counts
    ecn: Option<ECNCounts>,
}

/// ACK 范围
///
/// 每个 ACK 范围由按照递减的数据包编号交替出现 Gap 和 ACK 范围长度值组成.
///
/// 如果最近接收的数据包编号是 1000，那么第一个 ACK 范围的 Gap 值就是 999,
/// ACK 范围长度值表示从 1000 开始的连续数据包数。如果存在多个 ACK 范围,
/// 它们的 Gap 和 ACK 范围长度值都按照同样的方式交替出现.
///
/// 结构如下:
/// ACK Range {
///     Gap (i),
///     ACK Range Length (i),
/// }
#[derive(Clone, Copy)]
pub(crate) struct ACKRange {
    gap: usize,
    length: usize,
}

/// ECN
///
/// 用于指示ECN反馈并报告QUIC数据包的接收情况.
///
/// 结构如下:
/// ECN Counts {
///     ECT0 Count (i),
///     ECT1 Count (i),
///     ECN-CE Count (i),
/// }
#[derive(Clone, Copy)]
pub(crate) struct ECNCounts {
    ect0: u64,
    ect1: u64,
    ecn_ce: u64,
}

impl ACKFrame {
    /// 构造一个 ACK 帧
    ///
    /// # Arguments
    /// `with_ecn` - 是否携带 ECN
    /// # Returns
    /// 返回一个 ACK 帧
    pub(crate) fn new(with_ecn: bool) -> Self {
        Self {
            largest: 0,
            delay: 0,
            first_range: 0,
            ranges: Vec::new(),
            ecn: if with_ecn {
                Some(ECNCounts {
                    ect0: 0,
                    ect1: 0,
                    ecn_ce: 0,
                })
            } else {
                None
            },
        }
    }

    /// 获取最大的确认数据包编号
    ///
    /// # Returns
    /// 返回最大的确认数据包编号
    #[inline(always)]
    pub(crate) const fn get_largest(&self) -> PacketNumber {
        self.largest
    }

    /// 设置最大的确认数据包编号
    ///
    /// # Arguments
    /// `largest` - 最大的确认数据包编号
    #[inline(always)]
    pub(crate) fn set_largest(&mut self, largest: PacketNumber) {
        self.largest = largest
    }

    /// 获取延迟
    ///
    /// # Returns
    /// 返回延迟时间
    #[inline(always)]
    pub(crate) const fn get_delay(&self) -> u64 {
        self.delay
    }

    /// 设置延迟
    ///
    /// # Arguments
    /// `delay` - 延迟时间
    #[inline(always)]
    pub(crate) fn set_delay(&mut self, delay: u64) {
        self.delay = delay
    }

    /// 获取最大确认数据包之前连续数据包数量
    ///
    /// # Returns
    /// 返回数据包数量
    #[inline(always)]
    pub(crate) const fn get_first_range(&self) -> usize {
        self.first_range
    }

    /// 设置最大确认数据包之前连续数据包数量
    ///
    /// # Arguments
    /// `first_range` - 连续数据包数量
    #[inline(always)]
    pub(crate) fn set_first_range(&mut self, first_range: usize) {
        self.first_range = first_range
    }

    /// 获取 ECN Counts
    ///
    /// # Returns
    /// 返回 ECN Counts
    #[inline(always)]
    pub(crate) const fn get_ecn(&self) -> Option<ECNCounts> {
        self.ecn
    }

    /// 设置 ECN Counts
    ///
    /// # Arguments
    /// `ecn_count` - ECN Counts
    #[inline(always)]
    pub(crate) fn set_ecn(&mut self, ecn_count: ECNCounts) {
        self.ecn = Some(ecn_count)
    }

    /// 获取 ACK 范围
    ///
    /// # Returns
    /// 返回 ACK 范围
    #[inline(always)]
    pub(crate) fn get_ranges(&self) -> &[ACKRange] {
        &self.ranges
    }

    /// 设置 ACK 范围
    ///
    /// # Arguments
    /// `ranges` - ACK 范围列表
    #[inline(always)]
    pub(crate) fn set_ranges(&mut self, ranges: &[ACKRange]) {
        self.ranges.extend_from_slice(ranges)
    }
}

impl Serializer for ACKFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::Ack {
            with_ecm: self.ecn.is_some(),
        }
        .into()])?;

        payload_size += util::write_varint(self.largest, w)?;
        payload_size += util::write_varint(self.delay, w)?;
        payload_size += util::write_varint(self.ranges.len() as u64, w)?;
        payload_size += util::write_varint(self.first_range as u64, w)?;

        for range in self.ranges.iter() {
            payload_size += util::write_varint(range.gap as u64, w)?;
            payload_size += util::write_varint(range.length as u64, w)?;
        }

        if let Some(ecn) = self.ecn {
            payload_size += util::write_varint(ecn.ect0, w)?;
            payload_size += util::write_varint(ecn.ect1, w)?;
            payload_size += util::write_varint(ecn.ecn_ce, w)?;
        }

        Ok(payload_size)
    }
}

impl Deserializer for ACKFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let largest = util::read_varint(r)?;
        self.largest = largest.value;
        payload_size += largest.size;

        let delay = util::read_varint(r)?;
        self.delay = delay.value;
        payload_size += delay.size;

        let range_count = util::read_varint(r)?;
        payload_size += range_count.size;
        let range_count = range_count.value as usize - 1;

        let first_range = util::read_varint(r)?;
        self.first_range = first_range.value as usize;
        payload_size += first_range.size;

        for _ in 0..range_count {
            let gap = util::read_varint(r)?;
            payload_size += gap.size;

            let length = util::read_varint(r)?;
            payload_size += length.size;

            self.ranges.push(ACKRange {
                gap: gap.value as usize,
                length: length.value as usize,
            });
        }

        if let Some(ecn) = &mut self.ecn {
            let ect0 = util::read_varint(r)?;
            ecn.ect0 = ect0.value;
            payload_size += ect0.size;

            let ect1 = util::read_varint(r)?;
            ecn.ect1 = ect1.value;
            payload_size += ect1.size;

            let ecn_ce = util::read_varint(r)?;
            ecn.ecn_ce = ecn_ce.value;
            payload_size += ecn_ce.size;
        }

        Ok(payload_size)
    }
}
