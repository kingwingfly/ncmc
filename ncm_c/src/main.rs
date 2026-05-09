use clap::Parser;
use crossbeam_deque::{Injector, Worker};
use ncmc_lib::NcmFile;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Number of threads to use.
    #[clap(short = 'j', long, default_value_t=std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1))]
    threads: usize,
    /// No internet. Do not try to fetch cover from the Internet if not contained in the ncm file.
    #[clap(long)]
    no_internet: bool,
    /// Show less information about decrypt result
    #[clap(short = 'q', long)]
    quiet: bool,
    /// Input files, can be multiple.
    /// e.g. `find . -type f -name '*.ncm' -exec ncm_c {} +` or `fd -e ncm -X ncm_c` or `ncm_c *.ncm`.
    /// The output will be next to the input file.
    input: Vec<PathBuf>,
}

fn main() {
    let args = Cli::parse();
    let global = Arc::new(Injector::new());
    for path in args.input.into_iter() {
        global.push(path);
    }
    let workers = (0..args.threads)
        .map(|_| Worker::new_fifo())
        .collect::<Vec<_>>();
    let stealers = Arc::new(workers.iter().map(|w| w.stealer()).collect::<Vec<_>>());
    workers
        .into_iter()
        .map(|local| {
            let global = global.clone();
            let stealers = stealers.clone();
            std::thread::spawn(move || {
                while let Some(path) = local.pop().or_else(|| {
                    // Otherwise, we need to look for a task elsewhere.
                    std::iter::repeat_with(|| {
                        // Try stealing a batch of tasks from the global queue.
                        global
                            .steal_batch_and_pop(&local)
                            // Or try stealing a task from one of the other threads.
                            .or_else(|| stealers.iter().map(|s| s.steal()).collect())
                    })
                    // Loop while no task was stolen and any steal operation needs to be retried.
                    .find(|s| !s.is_retry())
                    // Extract the stolen task, if there is one.
                    .and_then(|s| s.success())
                }) {
                    match NcmFile::open(&path)
                        .and_then(|f| {
                            if args.no_internet {
                                Ok(f)
                            } else {
                                f.with_cover()
                            }
                        })
                        .and_then(|f| f.save())
                    {
                        Ok(p) => match args.quiet {
                            true => println!("{}", p.display()),
                            false => println!("Decrypted {} => {}", path.display(), p.display()),
                        },
                        Err(e) => match args.quiet {
                            true => eprintln!("Failed to decrypt {}: {}", path.display(), e),
                            false => eprintln!("{}", path.display()),
                        },
                    }
                }
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|jh| jh.join().unwrap());
}
