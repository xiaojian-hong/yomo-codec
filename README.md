# YoMo-Codec (Alpha)

![CI](https://github.com/10cella/yomo-codec/workflows/CI/badge.svg?branch=doc)

开源工业微服务高性能编解码器

# Coding

1. Add `rustfmt`: `rustup component add rustfmt --toolchain stable-x86_64-apple-darwin`
1. Add .githooks: `git config core.hooksPath .githooks`

# Test

`RUST_BACKTRACE=1 cargo test`

# Doc

`cargo doc && open target/doc/settings.html`

# Reference & Learn

[LEB128编码方式](https://berryjam.github.io/2019/09/LEB128(Little-Endian-Base-128)%E6%A0%BC%E5%BC%8F%E4%BB%8B%E7%BB%8D/)
[zigzag数据字典方式](https://developers.google.com/protocol-buffers/docs/encoding#signed-integers)

# Tools for debug

[二级制表示器工具](http://calc.50x.eu/)
