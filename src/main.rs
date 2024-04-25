mod cli;

use clap::Parser;
use cli::*;

fn main() -> anyhow::Result<()>{
    // usage: rcil -i <input_file>.csv -o <output_file>.json --delimiter <delimiter>
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            process_csv(&opts.input, opts.output, OutputFormat::Json)?;
        }
    }
    Ok(())
}