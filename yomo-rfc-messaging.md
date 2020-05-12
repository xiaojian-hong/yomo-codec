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

* Binary is good for encoding/decoding, especially for server side. can skip the data don't care about directly
* Not design for saving bytes, aimed for 5G & Wifi6
* Not related to connection protocol, just a message format

## Tag-Length-Value Format

The TLV format is in binary format, formed by:

1. `tag`: a 1-byte big-endian field indicating the type of message
2. `length`: a `varint` type but always use `u64` as internal encoding, indicates how many bytes behind this segment presented the value
3. `value`: the payload of value

## Varint

Used for represents a variable length integer. Use [LEB128 Encoding](https://google.com/) way.

The Rule is:

1. Unsigned integer encoding using [zigzag](https://developers.google.com/protocol-buffers/docs/encoding#signed-integers). Transform all the `i64` data to `u64`, so we don't need to care about the signed bit when encode/decode
1. The highest bit of Every byte reperesent as MSB(Most-Significant-Bit): when `MSB` is `1` means following byte is also the `varint` part; `0` means this byte is last byte of whole varint. The lowest 7-bits used for represent the value.
1. Use big-endian encode value.

### Example

an i32 value `259` in Dec we represent in binary is `0000 0000 0000 0000 0000 0001 0000 0011`, it use 4 bytes. When we encode it to YoMo-Codec, has 4 steps:

1. The valid bytes are `0000 0001 0000 0011`
2. Choosing 7-bits as value bits: `0000 0010 0000 0011`
3. Use big-endian sequence, transform to: `0000 0011 0000 0010`
4. Change MSB as `1` except the last byte: `1000 0011 0000 0010`

## Fundamental Types

There are 5 types meta types in `Tag`:

1. `String`: use `0x00` describe it in `Tag`, `UTF-8` encoding
2. `Integer`: describe as `0x01`, `Varint` type
3. `Float`: describe as `0x02`, is [IEEE754]() format as big-endian
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

First define this message struct, just like a `.proto` file did :

1. Use `Tag = 0x01` describe `"name"`, its value is a `String` type which is `CELLA`, the length of this string is `5`
2. Use `Tag = 0x02` describe `"age"`, its value is an i64 integer `2`

Will be encode as this:

`0x01 0x06 0x00 0x43 0x45 0x4C 0x4C 0x41 0x02 0x02 0x01 0x02`

Explain:

```
0x01 -> Tag=0x01 means key="name" node
    0x06 -> The length of the key's value is 6 (1 byte type + 5 bytes value)
        0x00 -> The type of value is String (0x00 in fundamental type)
            0x43 0x45 0x4C 0x4C 0x41 -> UTF-8 string for `CELLA`
0x02 -> Tag=0x02 means key="age" node
    0x02 -> The length of the key's value is 2 (1 byte type + 1 byte value)
        0x01 -> The type of value is Varint (0x01 in fundamental type)
            0x02 -> 0x02 is zigzag format which represent i64 number 1
```


## Complex Types
