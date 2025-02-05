# About

A tool to convert ncm file to mp3/flac/...

网易云音乐的ncm文件转换工具。

# Usage

```shell
ncmc -h

find . -type f -name '*.ncm' -print0 | xargs -0 ncmc
```

# Installation

```shell
cargo install ncmc
```
or download the binary from [release page](https://github.com/kingwingfly/ncmc/releases)

# Acknowledgement

[YTSakura233/ncm2mp3](https://github.com/YTSakura233/ncm2mp3)
