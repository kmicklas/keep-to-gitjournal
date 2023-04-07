use clap::Parser;
use std::path::PathBuf;

mod keep;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Google Keep directory, Takeout/Keep in extracted Takeout archive")]
    keep_dir: PathBuf,

    #[arg(short, long, default_value = ".")]
    output_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let notes = keep::read_notes(&args.keep_dir)?;

    println!("{:?}", notes);

    Ok(())
}
