use crate::qr_library::defines::{BytesVector, MyImageBuffer};
use crate::qr_library::{convert_to_u8, generator, ZxQrCode};
use image::ImageFormat;
use std::io::Cursor;

pub struct MyQrCore {
    image_buffer: MyImageBuffer,
}

impl ZxQrCode for MyQrCore {
    fn convert_to_zx_8x32_quad(&self) -> Vec<u8> {
        convert_to_u8::convert(&self.image_buffer)
    }
    fn convert_to_image(&self, file_extension: &str) -> Result<Vec<u8>, String> {
        let err_msg = format!("Unknown format {file_extension}");

        let image_format = ImageFormat::from_extension(file_extension).expect(&err_msg);
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
}

pub fn generate(data: &String, size: u32) -> impl ZxQrCode {
    let image_buffer = generator::generate(data, size);
    MyQrCore { image_buffer }
}
