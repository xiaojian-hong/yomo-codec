#[derive(Debug, Default)]
/// 描述变长类型（little-endian）
/// 
/// # 规则
/// 
/// 一个Byte是一个单元，最高位用作msb：
/// + 当最高位为1时，表示后面一个Byte也是该值的一部分
/// + 当最高位为0时，表示这是最后一个值的表述部分
/// 所以表示值的有效位为 7 个bits。例如：`0000 0100`，表示该Byte就是整个值，如果该值类型为数值类型，则是`0x04`，十进制是4；而`1000 0010 0000 0001`在解析编码的时候，会先取出第一个Byte `1000 0010`，首先发现msb位为1，则表示下一个Byte也是该值的一部分，则继续读取下一个值`0000 0001`，发现该Byte的msb为0，则完成该Varint值的读取，得到的两个描述值的Byte分别为`0000 0010`和`0000 0001`, 按照little-endian规则，该值是`0000 0001 0000 0010`，如果该值是数值类型，则是`0x12`, 十进制是258
pub struct VarintElement {
  code: u8,
}

impl VarintElement {
  pub fn set_value(&mut self, data: u8) {
    self.code = data;
  }

  pub fn has_next(&self) -> bool {
    return self.code | 0x80 == 0x80;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn msb_is_1() {
    let mut v = VarintElement::default();
    v.set_value(0x01);
    // t.print();
    assert_eq!(v.has_next(), false);
  }

  #[test]
  fn msb_is_0() {
    let mut v = VarintElement::default();
    v.set_value(0x80);
    // t.print();
    assert_eq!(v.has_next(), true);
  }
}