use anyhow::{bail, Result};
use nix::unistd::{fork, ForkResult};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::os::unix::process::CommandExt;
use std::{path::PathBuf, process::Command};
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(parse(from_os_str))]
    location: Option<PathBuf>,

    #[structopt(default_value = "1")]
    count: i32,
    // #[structopt(short = "p", long = "player", default_value = "mpv")]
    // player: String,
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
        // if let Ok(ForkResult::Child) = unsafe { fork() } {
        //     if let Some(file) = files.get(files.len() - 1 - i as usize) {
        //         let split: Vec<&str> = args.player.split(" ").collect();
        //         Command::new(split[0])
        //             .args(&split[1..])
        //             .arg(file.to_str().unwrap())
        //             .exec();
        //     } else {
        //         bail!("No files found!")
        //     }
        // }
        if let Some(file) = files.get(i as usize) {
            opener::open(file)?;
        }
    }

    Ok(())
}
