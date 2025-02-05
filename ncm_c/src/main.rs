use clap::Parser;
use ncmc_lib::NcmFile;
use std::path::PathBuf;
use std::thread;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Input files, can be multiple.
    /// e.g. `find . -type f -name '*.ncm' -print0 | xargs -0 ncm_c`.
    /// The output will be next to the input file.
    input: Vec<PathBuf>,
}

fn main() {
    let args = Cli::parse();
    args.input
        .into_iter()
        .map(|path| {
            thread::spawn(move || {
                let mut ncm = NcmFile::open(&path).unwrap();
                match ncm.save() {
                    Ok(_) => println!("Decrypted {}", path.display()),
                    Err(e) => eprintln!("Failed when decrypting {}: {e}", path.display()),
                }
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|jh| jh.join().unwrap());
}
