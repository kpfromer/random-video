use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(parse(from_os_str))]
    location: Option<PathBuf>,

    #[structopt(default_value = "1")]
    count: i32,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let location = args.location.unwrap_or(env::current_dir().unwrap());
    let count = args.count;

    let files = {
        let mut files = WalkDir::new(&location)
            .into_iter()
            .filter_map(|entry| entry.ok().map(|entry| entry.path().to_path_buf()))
            .filter(|path| path.extension().unwrap_or_default().to_str() == Some("mp4"))
            .collect::<Vec<_>>();
        files.shuffle(&mut thread_rng());
        files
    };

    for i in 0..count {
        if let Some(file) = files.get(i as usize) {
            opener::open(file)?;
        }
    }

    Ok(())
}
