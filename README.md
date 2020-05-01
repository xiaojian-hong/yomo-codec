# YoMo-Codec (Alpha)

![CI](https://github.com/10cella/yomo-codec/workflows/CI/badge.svg?branch=doc)

开源工业微服务高性能编解码器，关注：1）基于5G场景的边缘计算优化 2）Streaming Computing 3）MicroServices

# Coding

1. Add `rustfmt`: `rustup component add rustfmt --toolchain stable-x86_64-apple-darwin`
1. Add .githooks: `git config core.hooksPath .githooks`

# Test

`RUST_BACKTRACE=1 cargo test`

# Doc

`cargo doc && open target/doc/settings.html`

# Reference & Learn

[wikipedia: Type-length-value](https://en.wikipedia.org/wiki/Type-length-value)

[LEB128编码方式](https://berryjam.github.io/2019/09/LEB128(Little-Endian-Base-128)%E6%A0%BC%E5%BC%8F%E4%BB%8B%E7%BB%8D/)
[zigzag数据字典方式](https://developers.google.com/protocol-buffers/docs/encoding#signed-integers)

[how to documenting the TVL protocol](https://github.com/lightningnetwork/lightning-rfc/blob/master/01-messaging.md)
[a discussion on a encoding proposal](https://github.com/lightningnetwork/lightning-rfc/pull/607#issuecomment-504466232)

# Tools for debug

[二级制表示器工具](http://calc.50x.eu/)
