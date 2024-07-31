extern crate core;

mod args;
mod generator;
mod utils;

use crate::args::get_config_from_args;
use crate::generator::{generate, save_binary, save_image};
use crate::utils::get_extension;

fn main() {
    let conf = get_config_from_args().unwrap();
    let buf = generate(&conf.url, conf.size);

    let ext = get_extension(&conf.output).unwrap();
    match ext {
        "bin" => save_binary(&buf, &conf.output),
        &_ => save_image(&buf, &conf.output),
    }
}
