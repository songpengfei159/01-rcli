mod cli;

pub use clap::Parser;
use cli::*;
fn main() {
    // usage: rcil -i <input_file>.csv -o <output_file>.json --delimiter <delimiter>
    let opts = Opts::parse();
    println!("{:?}", opts);
}