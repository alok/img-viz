use globset::{Glob, GlobSetBuilder};
use rand::{seq::IteratorRandom, thread_rng};
use std::{error::Error, process::Command};
use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, StructOpt)]
#[structopt(about = "View random images.")]
struct Opt {
    #[structopt(help = "Directories to draw images from.", default_value = ".")]
    dirs: Vec<String>,

    #[structopt(
        short = "n",
        long = "num_samples",
        help = "Number of images to view.",
        default_value = "100"
    )]
    num_samples: usize,
}

const IMG_EXTS: [&str; 4] = ["png", "jpg", "jpeg", "gif"];

fn main() -> Result<(), Box<Error>> {
    let args = Opt::from_args();

    let mut globs = GlobSetBuilder::new();
    for e in &IMG_EXTS {
        globs.add(Glob::new(&format!("**/*.{}", e))?);
    }
    let set = globs.build()?;

    let matches = args.dirs.into_iter().flat_map(|dir| {
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|entry| entry.ok().map(DirEntry::into_path))
            .filter(|file| set.is_match(file))
    });

    let samples: _ = matches.choose_multiple(&mut thread_rng(), args.num_samples);

    if !samples.is_empty() {
        Command::new("icat").args(samples).status()?;
    }

    Ok(())
}
