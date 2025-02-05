use ncm2mp3_lib::NcmFile;

fn main() {
    let mut ncm = NcmFile::open("test/ヰ世界情緒 - シリウスの心臓.ncm").unwrap();
    println!("{:?}", ncm);
    ncm.save().unwrap();
}
