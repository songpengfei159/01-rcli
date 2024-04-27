use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 12)]
    pub length: u8,
    #[arg(short, long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(short, long, default_value_t = true)]
    pub numbers: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(short, long, default_value_t = false)]
    pub symbols: bool,
}
