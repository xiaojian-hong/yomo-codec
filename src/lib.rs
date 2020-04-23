// pub mod codec;
pub mod tag;
pub mod varint;

use tag::Tag;
use varint::Varint;

#[derive(Debug, Default)]
pub struct TLV {
  tag: Tag,
  len: i64,
  val: i64,
  pos: usize,
}

impl TLV {
  /// 从一段二进制序列中，给定起始位置，开始读取出一个完整TLV结构的描述
  pub fn read(&mut self, data: &[u8], pos: usize) {
    self.pos = pos;
    // 1）读取`Tag`部分
    self.tag = Tag::new(data[pos]);
    // `Tag`只占1个Byte
    self.pos += 1;
    // 2) 开始读取变长类型的`Length`
    let mut lv = Varint::new();
    // 从指定的位置开始寻找一个完整的LEB128变长数值类型，返回的结果是`Length`所占的字节数
    let delta = lv.read(&data, self.pos);
    // 解码，得出`Length`的值
    self.len = lv.to_i64();
    // 3）读取`Value`部分，这里先假设`Tag`读到的结果为0x01，即为Varint类型（变长数值类型）
    // `Value`的起始位置是`Length`的起始位置 + 其存储长度
    // `Value`的存储长度就是`Length`的值
    self.pos += delta;
    let tmp = &data[self.pos..(self.pos + (self.len as usize))];
    let mut v = Varint::new();
    v.read(&tmp, 0);
    // 得到`Value`的值
    self.val = v.to_i64();
  }

  pub fn to_value(&mut self) -> i64 {
    self.val
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tag;
  use varint;

  #[test]
  fn tlv() {
    let v = [0x01, 0x02, 0x01];
    let mut tlv = TLV::default();
    tlv.read(&v, 0);
    assert_eq!(tlv.to_value(), -1);
  }

  #[test]
  fn raw_tlv() {
    // 假设用TLV编码方式表达JSON内容的 { a:-1 } 的话：
    //
    //  Tag = 0x01 (Integer)
    //  Length = 0x02 (means 1 byte length)
    //  Value = 0x01 (-1)
    //
    // 1) read `Tag`, the first byte。Tag只用1个Byte描述，Tag描述了Value的类型
    let v = [0x01, 0x02, 0x01];
    let mut t = tag::Tag::default();
    t.read_type(v[0]);
    assert_eq!(t.get_type(), 0x01);
    // 2) read `Length`，以确定向后寻找多少个字节来确定Value的值
    let mut vlen = varint::Varint::new();
    let pos_delta = vlen.read(&v, 1);
    assert_eq!(pos_delta, 1);
    let len = vlen.to_i64();
    assert_eq!(len, 1);
    // 3) read `Value`，Value的值
    let mut vval = varint::Varint::new();
    vval.read(&v, (len as usize) + 1);
    assert_eq!(vval.to_i64(), -1);
  }
}