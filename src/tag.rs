#[repr(u8)]
/// WireType defines data type of a Tag
pub enum WireType {
  /// Represent varint type (int32, int64, uint32, uint64, sint32, sint64, bool, enum) as `0000 0000`
  Varint = 0,
  /// Represent Fixed64 type (double, long, int64, uint64) as `0000 0001`
  Fixed64 = 1,
  /// Represent Length-delimited type (string, bytes, embedded messages, packed repeated fields) as `0000 0010`
  LengthDelimited = 2,
  /// Represent Fixed32 type (fixed32, sfixed32, float) as `0000 0101`
  Fixed32 = 5,
}

#[derive(Debug, Default)]
/// 1 Byte Represent the `Tag` part of `TLV`
/// 
/// # Structure 
/// 
/// There are two parts of Tag, the lowest 3 bits are `wire type` and the highest 5 bits are `field-number`. 
/// 
/// ## wire-type
/// 
/// `wire type` has 3 valid bits, its value range is from `0000 0000` to `0000 0111`
/// 
/// ## field-number
/// 
/// `field-number` has 5 valid bits, the highest bit is `msb`, the valid value bits is lowest 4 bits. its range is from `0000(msb) 0000` to `0000(msb) 1111`
/// 
/// # Example
/// 
/// e.g. `Tag = 9`, the value represented in bits is `0000 1001`, the lowest 3 bits `001` is `wire-type`, so wire-type's value is 1. the highest 5 bits `0000 1` as a Byte is `0000 0001` (right shift 3 bits), so the filed-number is 1.
pub struct Tag {
  code: u8,
  wire_type: u8,
  field_number: u8,
}

impl Tag {
  pub fn encode(&mut self) {
    self.code = self.field_number << 3 | self.wire_type;
  }

  // pub fn set_wire_type(&mut self, wire_type: i32) {
  //   self.wire_type = wire_type as u8;
  //   self.encode();
  // }

  pub fn set_wire_type(&mut self, wire_type: WireType) {
    self.wire_type = wire_type as u8;
    self.encode()
  }

  pub fn get_wire_type(&self) -> u8 {
    return self.wire_type;
  }

  pub fn set_field_number(&mut self, field_number: u8) {
    // the codeue range of field_number is from 0b0000 - 0b0111 (the highest bit is preserved as `msb`)
    if field_number > 0b0111 {
      panic!("filed_number can not greater than 7")
    }
    self.field_number = field_number;
    self.encode();
  }

  pub fn get_filed_number(&self) -> u8 {
    return self.field_number;
  }

  pub fn get_code(&self) -> u8 {
    return self.code;
  }

  pub fn print(&self) {
    println!("[Tag={:#010b}] wire_type = {:#06b}, filed_number = {:#010b}", self.code, self.wire_type, self.field_number);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ctor_default() {
    let t = Tag::default();
    let code = t.get_code();
    t.print();
    assert_eq!(t.get_wire_type(), 0);
    assert_eq!(t.get_filed_number(), 0);
    assert_eq!(code, 0);
  }

  #[test]
  fn test_1() {
    let mut t = Tag::default();
    t.set_wire_type(WireType::Fixed64);
    t.set_field_number(1);
    t.print();
    assert_eq!(t.get_wire_type(), 1);
    assert_eq!(t.get_filed_number(), 1);
    assert_eq!(t.get_code(), 9);
  }

  #[test]
  #[should_panic]
  fn panic_when_filed_number_over_range() {
    let mut t = Tag::default();
    t.set_field_number(8);
  }
}
