use crate::args::get_config_from_args;

mod args;
mod qr_library;
mod utils;

use crate::utils::get_extension;
use qr_library::ZxQrCode;

fn main() {
    let conf = get_config_from_args().unwrap();
    let my_qr = qr_library::generate(&conf.url, conf.size);

    let ext = get_extension(&conf.output).unwrap();
    match ext {
        "bin" => {
            let buf = my_qr.convert_to_zx_quad();
            utils::save_buffer(&buf, &conf.output).unwrap();
        }
        &_ => {
            let buf = my_qr.convert_to_image(&conf.output).unwrap();
            utils::save_buffer(&buf, &conf.output).unwrap();
        }
    }
}
