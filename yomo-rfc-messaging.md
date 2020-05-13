# YoMo-Codec: Base Format

## Overview

Version 0 (v202003). This protocol defines the framing individual message formats.

## TOC

* [YoMo Message Format](#yomo-message-format)
* [TLV: Tag-Length-Value Format](#tag-length-value-format)
* [Fundamental Types](#fundamental-types)
* [TLTV Format](#tltv-format)
* [Complex Types](complex-types)

## YoMo Message Format

YoMo-Codec defines YoMo message formats.

* Binary is good for encoding/decoding, especially for server side. You can directly skip the data which you don't care at all.
* Not design for saving bytes, aimed for 5G & Wifi6
* Not related to connection protocol, just a message format

## Tag-Length-Value Format

The TLV is in binary format, formed by:

1. `tag`: a 1-byte big-endian field, which indicates the type of message
2. `length`: a `varint` type but always use `u64` as internal encoding, which indicates the size of value field (how many bytes behind this segment represent the value field)
3. `value`: the payload of value

## Varint

Varint represents a variable-length integer in [LEB128 Encoding](https://google.com/search?q=LEB128+Encoding) format.

The rules are:

1. Uses [zigzag](https://developers.google.com/protocol-buffers/docs/encoding#signed-integers) encoding to transform all signed integers to unsigned integers (`i64` to `u64`), so we don't need to care about the signed integers when encoding/decoding.
2. The highest bit of each byte reperesents as MSB (Most-Significant-Bit): when `MSB` is `1`, it indicates the following byte is also a part of `varint`, `0` indicates this byte is last byte of whole varint. The lower 7-bits are used to represent the value.
3. Uses little-endian encoding.

### Example

An i32 value `259` in Dec we represent in binary is `0000 0000 0000 0000 0000 0001 0000 0011`, it uses 4 bytes. When we encode it in YoMo-Codec, there are 4 steps:

1. The valid bytes are `0000 0001 0000 0011`
2. Choose 7-bits as value bits: `0000 0010 0000 0011`
3. Use little-endian sequence, transform to: `0000 0011 0000 0010`
4. Change MSB as `1` except the last byte: `1000 0011 0000 0010`

## Fundamental Types

There are 5 meta types in `Tag`:

1. `String`: describe as `0x00`, `UTF-8` encoding
2. `Integer`: describe as `0x01`, `Varint` type
3. `Float`: describe as `0x02`, [IEEE754](https://en.wikipedia.org/wiki/IEEE_754) format as big-endian
4. `UUID`: describe as `0x03`, 128-bits fixed-length
5. `SCode`: describe as `0x04`, wait-for-using
6. `Binary`: describe as `0x80`, raw binary data

## TLTV Format

If we want to transform this `JSON` format object as YoMo-Codec:

```json
{
    "name": "CELLA",
    "age" : 1
}
```

First define the message struct, just like a `.proto` file does:

1. Use `Tag = 0x01` to describe `"name"`, its value is a `String` type which is `CELLA`, the length of this string is `5`
2. Use `Tag = 0x02` to describe `"age"`, its value is an i64 integer `2`

Will be encoded as:

`0x01 0x06 0x00 0x43 0x45 0x4C 0x4C 0x41 0x02 0x02 0x01 0x02`

Explanation:

```
0x01 -> Tag=0x01 means key="name" node
    0x06 -> The length of value is 6 (1 byte type + 5 bytes value)
        0x00 -> The type of value is String (0x00 in fundamental type)
            0x43 0x45 0x4C 0x4C 0x41 -> UTF-8 string for `CELLA`
0x02 -> Tag=0x02 means key="age" node
    0x02 -> The length of value is 2 (1 byte type + 1 byte value)
        0x01 -> The type of value is Varint (0x01 in fundamental type)
            0x02 -> 0x02 is zigzag format which represents i64 number 1
```


## Complex Types
