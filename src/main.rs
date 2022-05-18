use clap::Parser;

mod key_generator;
use key_generator::GenMode;

/// Key Generator
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Key Generator",
    long_about = "Simple program to Key Generator"
)]
struct Args {
    /// Generate mode [easy|medium|hard]
    #[clap(short, long, default_value = "hard")]
    mode: String,

    /// Key length
    #[clap(short, long, default_value_t = 10)]
    len: u8,

    /// Key size
    #[clap(short, long, default_value_t = 3)]
    size: u8
}

fn main() {
    let args: Args = Args::parse();
    let gen_mode: GenMode = args.mode.to_string().into();
    let len: u8 = args.len;
    let size: u8 = args.size;
    key_generator::gen(gen_mode, len, size);
}
