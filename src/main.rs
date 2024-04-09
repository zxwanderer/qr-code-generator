extern crate core;

mod args;
mod config;
mod generator;
mod utils;

use crate::args::get_config_from_args;

fn main() {
    let conf = get_config_from_args().unwrap();
    generator::generate(&conf);
}
