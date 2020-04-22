use std::vec::Vec;

/// 描述变长类型（little-endian）
/// 
/// # 规则
/// 
/// 一个Byte是一个单元，最高位用作msb：
/// 
/// + 当最高位为1时，表示后面一个Byte也是该值的一部分
/// + 当最高位为0时，表示这是最后一个值的表述部分
/// 
/// 所以表示值的有效位为 7 个bits。例如：`0100 0001`，表示该Byte就是整个值，如果该值类型为数值类型，则是`0x41`（十进制是101），如果该值类型是字符（不是Rust的char，它是4bytes的），表示'A'；而`1000 0010 0000 0001`在解析编码的时候，会先取出第一个Byte `1000 0010`，首先发现msb位为1，则表示下一个Byte也是该值的一部分，则继续读取下一个值`0000 0001`，发现该Byte的msb为0，则完成该Varint值的读取，得到的两个描述值的Byte分别为`0000 0010`和`0000 0001`, 按照little-endian规则，该值是`0000 0001 0000 0010`，如果该值是数值类型，则是`0x12`, 十进制是258.
#[derive(Debug, Default)]
pub struct Varint {
  vec: Vec<u8>,
}

impl Varint {
  /// Create an empty Varint
  pub const fn new() -> Varint {
    Varint { vec: Vec::new() }
  }

  /// Get the raw 'value bytes'
  pub fn into_bytes(mut self) -> Vec<u8> {
    self.vec.reverse();
    return self.vec;
  }

  /// 给定的bytes，给定起始位置，读取一个完整的`Varint`值，并返回截止字节位置
  pub fn read(&mut self, val: Vec<u8>, at: usize) -> usize {
    let mut curr = at;
    let pos = loop {
      // push `val[curr]`'s 7 valid bits `0111 1111` to vec
      self.vec.push(val[curr] & 0x7F);
      // if `val[curr]`'s msb is `1000 0000`, continue reading next one
      if val[curr] & 0x80 == 0 {
        break curr;
      }
      curr += 1;
    };

    return pos;
  }

  /// Get capacity
  pub fn capacity(&self) -> usize {
    self.vec.capacity()
  }

  /// Return the length of this `Varint`, in bytes.
  pub fn len(&self) -> usize {
    self.vec.len()
  }

  /// Convert to i32 type value
  pub fn to_u32(&self) -> u32 {
    let len = self.vec.len();
    let mut result: u32 = 0;
    for i in 0..len {
      let tmp: u32 = (self.vec[i] as u32) << ((len - i - 1) * 8);
      result = result + tmp;
    }
    return result;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_1byte_varint_from_0() {
    let v = vec![0x01, 0x02];
    let mut reader = Varint::new();
    let at = reader.read(v, 0);
    assert_eq!(reader.into_bytes(), &[1]);
    assert_eq!(at, 0);
  }

  #[test]
  fn read_1byte_varint_from_1() {
    let v = vec![0x01, 0x01, 0x02];
    let mut reader = Varint::new();
    let at = reader.read(v, 1);
    assert_eq!(reader.into_bytes(), &[1]);
    assert_eq!(at, 1);
  }

  #[test]
  fn read_multi_bytes_varint_from_0() {
    // 1. valid bits are [0,1,2] -> 0x81, 0x82, 0x03
    // 2. remove msb: 0x01, 0x02, 0x03
    // 3. little-endian: 0x03, 0x02, 0x01
    let v = vec![0x81, 0x82, 0x03, 0x01, 0x01];
    let mut reader = Varint::new();
    let at = reader.read(v, 0);
    assert_eq!(reader.into_bytes(), &[0x03, 0x02, 0x01]);
    assert_eq!(at, 2);
  }

  #[test]
  fn to_u32() {
    // 1. valid bits are [0,1,2] -> 0x81, 0x82, 0x03
    // 2. remove msb: 0x01, 0x02, 0x03
    // 3. little-endian: 0x03, 0x02, 0x01
    // 4. to i32: 0x00010203 = 66051
    let v = vec![0x81, 0x82, 0x03, 0x01, 0x01];
    let mut reader = Varint::new();
    let at = reader.read(v, 0);
    // assert_eq!(reader.into_bytes(), &[0x03, 0x02, 0x01]);
    assert_eq!(at, 2);
    assert_eq!(reader.to_u32(), 0x010203)
  }
}