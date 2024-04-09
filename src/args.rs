use crate::config::*;
use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Args {
    #[arg(
				short,
				long,
				value_name = "FILE",
				help = "Output filename",
				default_value = DEFAULT_NAME,
				value_parser = output_value_parser
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

pub fn get_config_from_args() -> Result<Config, String> {
    let args = Args::parse();
    let mode = get_mode_by_path(&args.output).unwrap();

    let c = Config {
        output: args.output,
        url: args.url,
        size: args.size,
        mode,
    };
    Ok(c)
}

fn output_value_parser(s: &str) -> Result<String, String> {
    match get_mode_by_path(&s) {
        Ok(_) => Ok(s.to_string()),
        Err(e) => Err(e),
    }
}
