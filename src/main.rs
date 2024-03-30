mod config;
mod generator;

use clap::Parser;
use config::Config;

const DEFAULT_NAME: &str = "qr-code.bin";
const DEFAULT_SIZE: u32 = 32;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Output filename",
        default_value = DEFAULT_NAME
    )]
    output: String,
    #[arg(
        short,
        long,
        value_name = "URL",
        help = "Url",
        long_help = "Encoded url"
    )]
    url: String,
    #[arg(
    short,
    long,
    value_name = "SIZE",
    help = "qr-code size",
    long_help = "QR-code canvas size",
    default_value_t = DEFAULT_SIZE
    )]
    size: u32,
}

fn main() {
    let args = Args::parse();
    let config = Config {
        output: args.output,
        url: args.url,
        size: args.size,
    };

    generator::generate(&config);
}
