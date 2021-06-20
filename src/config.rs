use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opts {
    #[structopt(short, long)]
    pub verbose: bool,

    // #[structopt(short, long, parse(from_os_str))]
    // pub output: PathBuf,

    #[structopt(short = "i", long, required = true)]
    pub interface: String,
}

