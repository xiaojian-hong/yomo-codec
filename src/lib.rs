// pub mod codec;
pub mod tag;
pub mod varint;

#[cfg(test)]
mod tests {
  use super::*;
  use tag;
  use varint;

  #[test]
  fn tlv() {
    // 假设用TLV编码方式表达JSON内容的 { a:-1 } 的话：
    //  Tag = 0x01 (Integer)
    //  Length = 0x02 (means 1 byte length)
    //  Value = 0x01 (-1)
    //
    // 1) read `Tag`, the first byte。Tag只用1个Byte描述，Tag描述了Value的类型
    let v = vec![0x01, 0x02, 0x01];
    let mut t = tag::Tag::default();
    t.read_type(v[0]);
    assert_eq!(t.get_type(), 0x01);
    // 2) read `Length`，以确定向后寻找多少个字节来确定Value的值
    let mut vlen = varint::Varint::new();
    let next_pos = vlen.read(v, 1);
    assert_eq!(next_pos, 1);
    let len = vlen.to_i64();
    assert_eq!(len, 1);
    // 3) read `Value`，Value的值
    let mut vval = varint::Varint::new();
    vval.read(vec![0x01, 0x02, 0x01], (len as usize) + next_pos);
    assert_eq!(vval.to_i64(), -1);
  }
}