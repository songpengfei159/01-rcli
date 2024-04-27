mod cli;
mod process;

use clap::Parser;
use cli::*;
use process::generate_password;
use process::process_csv;

fn main() -> anyhow::Result<()> {
    // usage: rcil -i <input_file>.csv -o <output_file>.json --delimiter <delimiter>
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        Subcommand::GenPass(opts) => {
            generate_password(
                opts.length,
                opts.uppercase,
                opts.numbers,
                opts.lowercase,
                opts.symbols,
            )?;
        }
    }
    Ok(())
}
