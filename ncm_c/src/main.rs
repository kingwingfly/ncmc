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
                if let Err(e) = NcmFile::open(&path).and_then(|mut f| f.save()) {
                    eprintln!("Failed when decrypting {}: {e}", path.display());
                }
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|jh| jh.join().unwrap());
}
