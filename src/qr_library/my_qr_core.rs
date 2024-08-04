use crate::qr_library::defines::{BytesVector, MyImageBuffer};
use crate::qr_library::{convert_to_u8, generator, ZxQrCode};
use image::ImageFormat;
use std::io::{BufWriter, Cursor};

pub struct MyQrCore {
    image_buffer: MyImageBuffer,
}

impl ZxQrCode for MyQrCore {
    fn convert_to_zx_quad(&self) -> Vec<u8> {
        convert_to_u8::convert(&self.image_buffer)
    }

    fn convert_to_image(&self, file_format: &String) -> Result<Vec<u8>, String> {
        let err_msg = format!("Unknown format {file_format}");

        let image_format = ImageFormat::from_path(file_format).expect(&err_msg);
        let mut c = Cursor::new(Vec::new());
        match self
            .image_buffer
            .write_to(&mut c, image_format)
            .map_err(|e| e.to_string())
        {
            Ok(()) => Ok(c.into_inner()),
            Err(e) => Err(e),
        }
    }

    fn save_to_image(&self, path: &String) -> Result<(), String> {
        self.image_buffer.save(&path).map_err(|e| e.to_string())
    }
}

pub fn generate(data: &String, size: u32) -> impl ZxQrCode {
    let image_buffer = generator::generate(data, size);
    MyQrCore { image_buffer }
}
