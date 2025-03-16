# About

The lib for the tool to convert ncm file to mp3/flac/...

网易云音乐的ncm文件转换工具`ncm_c`的库。

# Usage

```rust no_run
use ncmc_lib::NcmFile;

let ncm = NcmFile::open("path/to/your.ncm").unwrap();
ncm.save().unwrap();
```

# features

- `cover_download`: provide `with_cover` method to download cover image from internet if not contained in ncm file.

# Acknowledgement

- [YTSakura233/ncm2mp3](https://github.com/YTSakura233/ncm2mp3)
- [taurusxin/ncmdump](https://github.com/taurusxin/ncmdump)
