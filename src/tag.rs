#[repr(u8)]
/// WireType defines data type of a Tag
pub enum WireType {
  /// UTF-8 string
  String = 0x00,
  /// Big-endian encoding. numberic type
  Integer = 0x01,
  /// float, IEEE754 Big-endian (32bit/64bit/128bit)
  Float = 0x02,
  /// Guid
  UUID = 0x03,
  /// Scode
  SCode = 0x04,
  /// Binary
  Binary = 0x80,
}

#[derive(Debug, Default)]
/// 1 Byte Represent the `Tag` of `Tag-Length-Value`, defines the data type
pub struct Tag {
  code: u8,
}

impl Tag {
  pub fn set_type(&mut self, wire_type: WireType) {
    self.code = wire_type as u8;
  }

  pub fn get_type(&self) -> u8 {
    return self.code;
  }

  pub fn print(&self) {
    println!("[Tag={:#010b}]", self.code);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ctor_default() {
    let t = Tag::default();
    let code = t.get_type();
    t.print();
    assert_eq!(t.get_type(), 0);
    assert_eq!(code, 0);
  }

  #[test]
  fn test_1() {
    let mut t = Tag::default();
    t.set_type(WireType::Integer);
    t.print();
    assert_eq!(t.get_type(), 1);
  }

  // #[test]
  // #[should_panic]
  // fn panic_when_filed_number_over_range() {
  //   let mut t = Tag::default();
  //   t.set_field_number(8);
  // }
}
