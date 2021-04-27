use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::PathBuf;
use std::{collections::HashSet, env};
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(parse(from_os_str))]
    location: Option<PathBuf>,

    #[structopt(default_value = "1")]
    count: i32,

    #[structopt(short, long, default_value = "mp4")]
    extension: Vec<String>,
}

fn main() -> Result<()> {
    let Cli {
        location,
        count,
        extension,
    } = Cli::from_args();
    let location = location.unwrap_or(env::current_dir().unwrap());
    let extension: HashSet<String> = extension.into_iter().collect();

    let files = {
        let mut files = WalkDir::new(&location)
            .into_iter()
            .filter_map(|entry| entry.ok().map(|entry| entry.path().to_path_buf()))
            .filter(|path| {
                if let Some(ext) = path.extension().unwrap_or_default().to_str() {
                    extension.contains(&String::from(ext))
                } else {
                    false
                }
            })
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
