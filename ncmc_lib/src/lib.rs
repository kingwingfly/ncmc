#![doc = include_str!("../README.md")]

mod error;

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyInit};
use base64::{prelude::BASE64_STANDARD, Engine};
use ecb::Decryptor;
use error::{NcmError, Result};
use id3::{
    frame::{Picture, PictureType},
    Tag, TagLike as _,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Seek, Write as _},
    path::{Path, PathBuf},
};

const CORE_KEY: &[u8; 16] = b"hzHRAmso5kInbaxW";
const META_KEY: &[u8; 16] = br#"#14ljk_!\]&0U<'("#;
const KEY_MASK: u8 = 0x64;
const META_MASK: u8 = 0x63;

/// Ncm file
#[derive(Debug)]
pub struct NcmFile {
    file: File,
    path: PathBuf,
    key: Key,
    meta: Meta,
}

impl NcmFile {
    /// Open a ncm file
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_owned();
        let mut file = File::open(&path)?;
        Self::verify_header(&mut file)?;
        let key = Key::new(Self::get_key(&mut file)?);
        let mut meta = Self::get_meta(&mut file)?;
        file.seek_relative(9)?; // CRC(4) and Padding(5)
        meta.cover = Self::get_cover(&mut file)?;
        Ok(Self {
            file,
            path,
            key,
            meta,
        })
    }

    fn verify_header(file: &mut File) -> Result<()> {
        let mut buf = [0; 10];
        file.read_exact(&mut buf)?;
        if &buf[..8] != b"CTENFDAM" {
            return Err(NcmError::Invalid("Invalid file header".to_string()));
        }
        Ok(())
    }

    fn get_key(file: &mut File) -> Result<Vec<u8>> {
        let mut buf = [0; 4];
        file.read_exact(&mut buf)?;
        let length = u32::from_le_bytes(buf) as usize;
        let mut buf = vec![0; length];
        file.read_exact(&mut buf)?;
        buf.iter_mut().for_each(|byte| *byte ^= KEY_MASK);
        let aes = Decryptor::<aes::Aes128>::new_from_slice(CORE_KEY).unwrap();
        let buf = aes
            .decrypt_padded_mut::<Pkcs7>(&mut buf)
            .map_err(|_| NcmError::Invalid("Failed to decrypt key".to_string()))?;
        if &buf[..17] != b"neteasecloudmusic" {
            return Err(NcmError::Invalid("Invalid key header".to_string()));
        }
        let key_data = &buf[17..];
        let mut key_box: [u8; 256] = core::array::from_fn(|i| i as u8);
        let mut last_byte = 0;
        let mut key_offset = 0;
        for i in 0..256 {
            let c = key_box[i]
                .wrapping_add(last_byte)
                .wrapping_add(key_data[key_offset]);
            key_offset += 1;
            if key_offset >= key_data.len() {
                key_offset = 0;
            }
            key_box.swap(i, c as usize);
            last_byte = c;
        }
        Ok(key_box.to_vec())
    }

    fn get_meta(file: &mut File) -> Result<Meta> {
        let mut buf = [0; 4];
        file.read_exact(&mut buf)?;
        let length = u32::from_le_bytes(buf) as usize;
        let mut buf = vec![0; length];
        file.read_exact(&mut buf)?;
        buf.iter_mut().for_each(|byte| *byte ^= META_MASK);
        if &buf[..22] != b"163 key(Don't modify):" {
            return Err(NcmError::Invalid("Invalid metadata header".to_string()));
        }
        let mut buf = BASE64_STANDARD
            .decode(&buf[22..])
            .map_err(|_| NcmError::Invalid("Failed to decode base64 metadata".to_string()))?;
        let aes = Decryptor::<aes::Aes128>::new_from_slice(META_KEY).unwrap();
        let buf = aes
            .decrypt_padded_mut::<Pkcs7>(&mut buf)
            .map_err(|_| NcmError::Invalid("Failed to decrypt metadata".to_string()))?;
        if &buf[..6] != b"music:" {
            return Err(NcmError::Invalid("Invalid meta marker".to_string()));
        }
        serde_json::from_slice(&buf[6..])
            .map_err(|e| NcmError::Invalid(format!("Failed to parse metadata: {}", e)))
    }

    fn get_cover(file: &mut File) -> Result<Vec<u8>> {
        let mut buf = [0; 4];
        file.read_exact(&mut buf)?;
        let length = u32::from_le_bytes(buf);
        let mut buf = vec![0; length as usize];
        file.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// save as general format next to the original ncm file
    pub fn save(self) -> Result<()> {
        let path = self.path.with_extension(&self.meta.format);
        self.save_to(path)
    }

    /// save as general format to the specified path
    pub fn save_to(self, path: impl AsRef<Path>) -> Result<()> {
        let tag = Tag::from(&self.meta);
        self.save_without_tags_to(&path)?;
        tag.write_to_path(path, id3::Version::Id3v24)?;
        Ok(())
    }

    /// savenext to the original ncm file without tags
    pub fn save_without_tags(self) -> Result<()> {
        let path = self.path.with_extension(&self.meta.format);
        self.save_without_tags_to(path)
    }

    /// save to the specified path without tags
    pub fn save_without_tags_to(mut self, path: impl AsRef<Path>) -> Result<()> {
        let mut file = std::fs::File::create(&path)?;
        std::io::copy(&mut self, &mut file)?;
        file.flush()?;
        Ok(())
    }

    /// Get the meta data (including cover, artist, album, etc.)
    pub fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl Read for NcmFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.file.read(buf)?;
        for (i, key) in (0..size).zip(&mut self.key) {
            buf[i] ^= key;
        }
        Ok(size)
    }
}

#[derive(Debug)]
struct Key {
    key: Vec<u8>,
    i: u8,
}

impl Key {
    fn new(key: Vec<u8>) -> Self {
        Self { key, i: 0 }
    }
}

impl Iterator for Key {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.i = self.i.wrapping_add(1);
        Some(
            self.key[self.key[self.i as usize]
                .wrapping_add(self.key[self.key[self.i as usize].wrapping_add(self.i) as usize])
                as usize],
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(missing_docs)]
pub struct Meta {
    pub album: String,
    #[serde(rename = "albumId")]
    pub album_id: usize,
    /// The url of the cover image
    #[serde(rename = "albumPic")]
    pub album_pic: String,
    #[serde(rename = "albumPicDocId")]
    pub album_pic_doc_id: serde_json::Value,
    pub alias: Vec<String>,
    pub artist: Vec<(String, usize)>,
    pub bitrate: usize,
    pub duration: usize,
    pub flag: Option<usize>,
    pub format: String,
    pub gain: Option<f64>,
    #[serde(rename = "musicId")]
    pub music_id: usize,
    #[serde(rename = "musicName")]
    pub music_name: String,
    #[serde(rename = "mvId")]
    pub mv_id: usize,
    #[serde(rename = "transNames")]
    pub trans_names: Vec<String>,
    #[serde(skip)]
    pub cover: Vec<u8>,
}

impl From<&Meta> for Tag {
    fn from(meta: &Meta) -> Self {
        let mut tag = Tag::new();
        tag.set_album(meta.album.clone());
        tag.add_frame(Picture {
            mime_type: "image/jpeg".to_string(),
            picture_type: PictureType::CoverFront,
            description: "Cover".to_string(),
            data: meta.cover.clone(),
        });
        tag.set_artist(
            meta.artist
                .iter()
                .map(|(name, _)| name)
                .fold(String::new(), |acc, x| {
                    if acc.is_empty() {
                        x.to_string()
                    } else {
                        acc + ", " + x
                    }
                }),
        );
        tag.set_duration(meta.duration as u32);
        tag.set_title(meta.music_name.clone());
        tag
    }
}
