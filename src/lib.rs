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
    // TLV:
    // Tag = 0x01 (Integer)
    // Length = 0x02 (means 1 byte length)
    // Value = 0x01 (-1)
    // 1) read `Tag`, the first byte
    let v = vec![0x01, 0x02, 0x01];
    let mut t = tag::Tag::default();
    t.read_type(v[0]);
    assert_eq!(t.get_type(), 0x01);
    // 2) read `Length`
    let mut vlen = varint::Varint::new();
    let next_pos = vlen.read(v, 1);
    assert_eq!(next_pos, 1);
    let len = vlen.to_i64();
    assert_eq!(len, 1);
    // 3) read `Value`
    let mut vval = varint::Varint::new();
    vval.read(vec![0x01, 0x02, 0x01], (len as usize) + next_pos);
    assert_eq!(vval.to_i64(), -1);
  }
}