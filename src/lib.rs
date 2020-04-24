// pub mod codec;
pub mod tag;
pub mod varint;

use tag::*;
use varint::Varint;

#[derive(Debug, Default)]
pub struct TLV {
  /// Tag 类型 1 Byte长度
  tag: Tag,
  /// Value 的长度
  len: i64,
  /// Value 的 raw bytes
  buf: Vec<u8>,
  /// 如果是读取一连串的数组时，pos返回的是下一次读取的位置，方便streaming read
  pos: usize,
}

impl TLV {
  /// Static method, build TLV block
  pub fn from_data(data: &[u8], pos:usize) -> TLV {
    let mut tlv = TLV { 
      tag: Tag::new(0x00), 
      len: 0,
      buf: [].to_vec(),
      pos: 0,
    };
    tlv.read(data, pos);
    tlv
  }

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
    self.buf = tmp.to_vec();
    self.pos += self.len as usize;
  }

  /// 如果TLV的`Value`是`Varint`类型，则转换为i64类型
  pub fn to_varint(&mut self) -> i64 {
    let mut v = Varint::new();
    v.read(&self.buf, 0);
    v.to_i64()
  }

  /// 如果TLV的`Value`是`Binary`类型，则转换为Vec<u8>类型
  pub fn to_binary(&mut self) -> &Vec<u8> {
    &self.buf
  }

  /// 如果TLV的`Value`是`Float`类型，则转换为IEEE754类型  
  pub fn to_float(&mut self) -> f64 {
    panic!("TODO: confirm rust f32 vs IEEE754, not implementation")
  }

  /// 如果TLV的`Value`是`String`类型，则转换为String类型
  pub fn to_string(&mut self) -> String {
    String::from_utf8(self.buf.to_vec()).unwrap()
  }

  pub fn to_uuid(&mut self) {
    panic!("TODO: we need implement an uuid format with distributed unique ID generator, also, the UUID should contain data sender identify and timestamp, not implementation")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tag;
  use varint;

  #[test]
  fn read_data() {
    let v = [0x01, 0x02, 0x01];
    let mut tlv = TLV::from_data(&v, 0);
    assert_eq!(tlv.to_binary().as_slice(), [0x01]);
    assert_eq!(tlv.pos, 3);
  }

  #[test]
  fn tlv() {
    let v = [0x01, 0x02, 0x01];
    let mut tlv = TLV::default();
    tlv.read(&v, 0);
    assert_eq!(tlv.to_varint(), -1);
    assert_eq!(tlv.to_binary().as_slice(), [1]);
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

  #[test]
  #[should_panic]
  fn panic_to_float() {
    let v = [0x01, 0x02, 0x01];
    let mut tlv = TLV::default();
    tlv.read(&v, 0);
    tlv.to_float();
  }
  
  #[test]
  fn to_string() {
    let v = [0x00, 0x02, 0x43];
    let mut tlv = TLV::default();
    tlv.read(&v, 0);
    assert_eq!(tlv.to_string(), "C");
  }
  
  #[test]
  fn to_string2() {
    let v = [0x00, 0x08, 240, 159, 146, 150];
    let mut tlv = TLV::default();
    tlv.read(&v, 0);
    assert_eq!(tlv.to_binary().as_slice(), [240, 159, 146, 150]);
    assert_eq!(tlv.to_string(), "💖");
  }
  
  #[test]
  #[should_panic]
  fn panic_to_uuid() {
    let v = [0x01, 0x02, 0x01];
    let mut tlv = TLV::default();
    tlv.read(&v, 0);
    tlv.to_uuid();
  }
}