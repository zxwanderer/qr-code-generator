use crate::args::get_config_from_args;
use crate::generator::generate;
use crate::utils::{get_extension, save_buffer};

mod args;
mod convert_to_u8;
mod defines;
mod generator;
mod utils;

fn main() {
    let conf = get_config_from_args().unwrap();
    let image_buf = generate(&conf.url, conf.size);

    let ext = get_extension(&conf.output).unwrap();
    match ext {
        "bin" => {
            let buffer = convert_to_u8::convert(&image_buf);
            save_buffer(buffer, &conf.output).unwrap();
        }
        &_ => image_buf.save(&conf.output).unwrap(),
    }
}
