//! Netease Cloud Music Crypt

mod error;

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyInit};
use base64::{prelude::BASE64_STANDARD, Engine};
use ecb::Decryptor;
use error::{NcmError, Result};
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
    key: Vec<u8>,
    meta: Meta,
}

impl NcmFile {
    /// Open a ncm file
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_owned();
        let mut file = File::open(&path).unwrap();
        Self::verify_header(&mut file)?;
        let key = Self::get_key(&mut file)?;
        let meta = Self::get_meta(&mut file)?;
        file.seek_relative(9)?; // CRC(4) and Padding(5)
        Self::skip_image(&mut file)?;
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
            return Err(NcmError::Invalid);
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
            .map_err(|_| NcmError::Invalid)?;
        if &buf[..17] != b"neteasecloudmusic" {
            return Err(NcmError::Invalid);
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
            return Err(NcmError::Invalid);
        }
        let mut buf = BASE64_STANDARD
            .decode(&buf[22..])
            .map_err(|_| NcmError::Invalid)?;
        let aes = Decryptor::<aes::Aes128>::new_from_slice(META_KEY).unwrap();
        let buf = aes
            .decrypt_padded_mut::<Pkcs7>(&mut buf)
            .map_err(|_| NcmError::Invalid)?;
        if &buf[..6] != b"music:" {
            return Err(NcmError::Invalid);
        }
        serde_json::from_slice(&buf[6..]).map_err(|_| NcmError::Invalid)
    }

    fn skip_image(file: &mut File) -> Result<()> {
        let mut buf = [0; 4];
        file.read_exact(&mut buf)?;
        let length = u32::from_le_bytes(buf) as i64;
        file.seek_relative(length)?;
        Ok(())
    }

    /// save as general format next to the original ncm file
    pub fn save(mut self) -> Result<()> {
        let mut file = std::fs::File::create(self.path.with_extension(&self.meta.format))?;
        let mut buf = vec![];
        let size = self.file.read_to_end(&mut buf)?;
        for i in 1..size + 1 {
            let j = i & 0xff;
            buf[i - 1] ^= self.key[self.key[j]
                .wrapping_add(self.key[self.key[j].wrapping_add(j as u8) as usize])
                as usize];
        }
        file.write_all(&buf)?;
        file.flush()?;
        Ok(())
    }

    /// Get the meta data
    pub fn meta(&self) -> &Meta {
        &self.meta
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Meta {
    pub album: String,
    #[serde(rename = "albumId")]
    pub album_id: usize,
    #[serde(rename = "albumPic")]
    pub album_pic: String,
    #[serde(rename = "albumPicDocId")]
    pub album_pic_doc_id: usize,
    pub alias: Vec<String>,
    pub artist: Vec<(String, usize)>,
    pub bitrate: usize,
    pub duration: usize,
    pub flag: usize,
    pub format: String,
    pub gain: f64,
    #[serde(rename = "musicId")]
    pub music_id: usize,
    #[serde(rename = "musicName")]
    pub music_name: String,
    #[serde(rename = "mvId")]
    pub mv_id: usize,
    #[serde(rename = "transNames")]
    pub trans_names: Vec<String>,
}
