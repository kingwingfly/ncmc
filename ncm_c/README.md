# About

A tool to convert ncm file to mp3/flac/...

网易云音乐的ncm文件转换工具。

# Usage

```sh
ncm_c -h

ncm_c *.ncm
# for unix-pro guys
find . -type f -name '*.ncm' -exec ncm_c {} +
# for `fd` user
fd -e ncm -X ncm_c

# decrtpt and gather
fd -e ncm -X ncm_c --quiet | xargs -d'\n' mv -t .
```

# Installation

```shell
cargo install ncm_c
```
or download the binary from [release page](https://github.com/kingwingfly/ncmc/releases)

# Others

The core file parser is published as a crate: [ncmc_lib](https://crates.io/crates/ncmc_lib).

# Acknowledgement

- [YTSakura233/ncm2mp3](https://github.com/YTSakura233/ncm2mp3)
- [taurusxin/ncmdump](https://github.com/taurusxin/ncmdump)
