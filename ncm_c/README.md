# About

A tool to convert ncm file to mp3/flac/...

网易云音乐的ncm文件转换工具。

# Usage

```shell
ncm_c -h

find . -type f -name '*.ncm' -print0 | xargs -0 ncm_c
```

# Installation

```shell
cargo install ncm_c
```
or download the binary from [release page](https://github.com/kingwingfly/ncmc/releases)

# Acknowledgement

[YTSakura233/ncm2mp3](https://github.com/YTSakura233/ncm2mp3)
