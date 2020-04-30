use std::vec::Vec;

pub const MSB: u8 = 0b1000_0000;
const DROP_MSB: u8 = 0b0111_1111;

/// 描述变长类型（little-endian）
///
/// # 参考
///
/// [LEB128编码方式](https://berryjam.github.io/2019/09/LEB128(Little-Endian-Base-128)%E6%A0%BC%E5%BC%8F%E4%BB%8B%E7%BB%8D/)
///
/// [zigzag数据字典方式](https://developers.google.com/protocol-buffers/docs/encoding#signed-integers)
///
/// [二级制表示器工具](http://calc.50x.eu/)
///
/// # 规则
///
/// 一个Byte是一个单元，最高位用作msb(most significant bit)：
///
/// + 当该Byte最高位为1时，表示后一个Byte也是该值的一部分，继续向后seek一个Byte处理
/// + 当该Byte最高位为0时，表示这是值的最后一个表述部分，停止seek
///
/// 所以表示值的有效位为7个bits。例如：`0100 0001`，表示该Byte就是整个值，如果该值类型为数值类型，则是`0x41`（十进制是101），如果该值类型是字符（不是Rust的char，它是4bytes的），表示'A'；而`1000 0010 0000 0001`在解析编码的时候，会先取出第一个Byte `1000 0010`，首先发现msb位为1，则表示下一个Byte也是该值的一部分，则继续读取下一个值`0000 0001`，发现该Byte的msb为0，则完成该Varint值的读取，得到的两个描述值的Byte分别为`0000 0010`和`0000 0001`, 按照big-endian规则，该值是`0000 0010 0000 0001`，如果该值是数值类型，则是`0x21`, 十进制是577.
#[derive(Debug, Default)]
pub struct Varint {
    vec: Vec<u8>,
}

/// zigzag from : https://developers.google.com/protocol-buffers/docs/encoding#signed-integers
fn zigzag_encode(from: i64) -> u64 {
    ((from << 1) ^ (from >> 63)) as u64
}

/// zigzag from : https://developers.google.com/protocol-buffers/docs/encoding#signed-integers
fn zigzag_decode(from: u64) -> i64 {
    ((from >> 1) ^ (-((from & 1) as i64)) as u64) as i64
}

impl Varint {
    /// Create an empty Varint
    pub const fn new() -> Varint {
        Varint { vec: Vec::new() }
    }

    /// Get the raw 'value bytes'
    pub fn into_bytes(&self) -> &Vec<u8> {
        &self.vec
    }

    /// 给定的bytes，给定起始位置，读取一个完整的`Varint`值，并返回读取字节长度
    pub fn read(&mut self, val: &[u8], at: usize) -> usize {
        let mut curr = at;
        let pos = loop {
            if val.len() == 0 {
                break 0;
            }
            self.vec.push(val[curr]);
            // if `val[curr]`'s msb is `1000 0000`, continue reading next one
            if val[curr] & MSB == 0 {
                break curr - at + 1;
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

    /// Convert to i64 type value
    ///
    /// if vec are [0x81, 0x82, 0x03], which in binary is [0b1000_0001, 0b1000_0010, 0b0000_0011]
    /// 1. remove msb: 0b0000_0001, 0b0000_0010, 0b0000_0011
    /// 2. is little-endian, 每一个byte都选底7位，组合的结果是: 0000_0000_1100_0001_0000_0001
    /// 3. 用u64表示结果: 0XC101 = 49409
    /// 4. zigzag_decode(49409) = -24705
    pub fn to_i64(&self) -> i64 {
        let len = self.vec.len();
        let mut result: u64 = 0;
        for i in 0..len {
            result |= ((self.vec[i] & DROP_MSB) as u64) << (i * 7);
        }
        return zigzag_decode(result);
    }

    /// encode i64 to varint number
    ///
    /// 1. data:i64 = 255
    /// 2. zigzag_encode(data) as u64 = 510（0b0000_0001_1111_1110)
    /// 3. to Varint codec: [0b0111_1110， 0b0000_0011]
    /// 4. add msb: [0b1111_1110， 0b0000_0011]
    pub fn from_i64(&mut self, data: i64) {
        let mut val = zigzag_encode(data);
        while val > DROP_MSB as u64 {
            let element: u8 = (val as u8 & DROP_MSB) | MSB;
            self.vec.push(element);
            val >>= 7;
        }
        self.vec.push(val as u8)
    }

    /// Convert to u64 type value
    ///
    /// if vec are [0x81, 0x82, 0x03], which in binary is [0b1000_0001, 0b1000_0010, 0b0000_0011]
    /// 1. remove msb: 0b0000_0001, 0b0000_0010, 0b0000_0011
    /// 2. is little-endian, 每一个byte都选低7位，组合的结果是: 0000_0000_1100_0001_0000_0001
    /// 3. 用u64表示结果: 0XC101 = 49409
    pub fn to_u64(&self) -> u64 {
        let len = self.vec.len();
        let mut result: u64 = 0;
        for i in 0..len {
            result |= ((self.vec[i] & DROP_MSB) as u64) << (i * 7);
        }
        result
    }

    /// encode i64 to varint number
    ///
    /// 1. data:i64 = 255 (0b1111_1111)
    /// 2. to Varint codec: [0b0111_1111, 0b0000_0001]
    /// 3. add msb: [0b1111_1111，0b0000_0001]
    pub fn from_u64(&mut self, data: u64) {
        let mut val = data;
        while val > DROP_MSB as u64 {
            let element: u8 = (val as u8 & DROP_MSB) | MSB;
            self.vec.push(element);
            val >>= 7;
        }
        self.vec.push(val as u8)
    }

    /// Convert to bool. empty means false,
    pub fn to_bool(&self) -> bool {
        if self.len() == 0 {
            return false;
        } else {
            return self.vec[0] == 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_1byte_varint_from_0() {
        let v = [0x01, 0x02];
        let mut reader = Varint::new();
        let delta = reader.read(&v, 0);
        assert_eq!(reader.into_bytes(), &[1]);
        assert_eq!(delta, 1);
    }

    #[test]
    fn from_u32_1_byte() {
        let v = [0x01, 0x01, 0x02];
        let mut reader = Varint::new();
        let delta = reader.read(&v, 1);
        assert_eq!(reader.into_bytes(), &[1]);
        assert_eq!(delta, 1);
    }

    #[test]
    fn read_multi_bytes_varint_from_0() {
        // 1. valid bits are [0,1,2] -> 0x81, 0x82, 0x03
        // 2. remove msb: 0x01, 0x02, 0x03
        let v = [0x81, 0x82, 0x03, 0x01, 0x01];
        let mut reader = Varint::new();
        let delta = reader.read(&v, 0);
        assert_eq!(reader.into_bytes(), &[0x81, 0x82, 0x03]);
        assert_eq!(delta, 3);
    }

    #[test]
    fn from_to_u64() {
        let data: u64 = 2;
        let mut reader = Varint::new();
        reader.from_u64(data);
        assert_eq!(reader.into_bytes(), &[0x02]);
        reader = Varint::new();
        reader.read(&[0x02], 0);
        assert_eq!(reader.to_u64(), data);
    }

    #[test]
    fn to_i64() {
        let v = [0x81, 0x82, 0x03, 0x01, 0x01];
        let mut reader = Varint::new();
        let delta = reader.read(&v, 0);
        assert_eq!(reader.into_bytes(), &[0x81, 0x82, 0x03]);
        assert_eq!(delta, 3);
        assert_eq!(reader.to_i64(), -24705)
    }

    #[test]
    fn from_i64() {
        let mut data: i64 = 255;
        let mut reader = Varint::new();
        reader.from_i64(data);
        assert_eq!(reader.into_bytes(), &[0b1111_1110, 0b0000_0011]);
        assert_eq!(reader.to_i64(), data);
        reader = Varint::new();
        data = -1;
        reader.from_i64(data);
        assert_eq!(reader.into_bytes(), &[0x01]);
        assert_eq!(reader.to_i64(), data);
    }

    #[test]
    fn i64() {
        let data: i64 = 1000;
        for i in (-1 * data)..data {
            let mut reader = Varint::new();
            reader.from_i64(i);
            let mut reader2 = Varint::new();
            reader2.read(reader.into_bytes(), 0);
            assert_eq!(reader2.to_i64(), i);
        }
    }

    #[test]
    fn to_bool_empty_is_false() {
        let v = [];
        let mut reader = Varint::new();
        reader.read(&v, 0);
        assert_eq!(reader.to_bool(), false);
    }

    #[test]
    fn to_bool_false_or_true() {
        let mut v = [0x00];
        let mut reader = Varint::new();
        reader.read(&v, 0);
        assert_eq!(reader.to_bool(), false);
        v = [0x01];
        reader = Varint::new();
        reader.read(&v, 0);
        assert_eq!(reader.to_bool(), true);
    }

    #[test]
    fn zigzag() {
        // let mut reader = Varint::new();
        assert_eq!(100, zigzag_decode(zigzag_encode(100)));
        assert_eq!(-100, zigzag_decode(zigzag_encode(-100)));
    }
}
