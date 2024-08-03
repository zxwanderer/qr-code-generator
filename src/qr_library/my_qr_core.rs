use crate::qr_library::defines::MyImageBuffer;
use crate::qr_library::{convert_to_u8, generator, ZxQrCode};

pub struct MyQrCore {
    image_buffer: MyImageBuffer,
}

impl ZxQrCode for MyQrCore {
    fn convert_to_zx_quad(&self) -> Vec<u8> {
        convert_to_u8::convert(&self.image_buffer)
    }

    fn save_to_image(&self, path: &String) -> Result<(), String> {
        self.image_buffer.save(&path).map_err(|e| e.to_string())
    }
}

pub fn generate(data: &String, size: u32) -> impl ZxQrCode {
    let image_buffer = generator::generate(data, size);
    MyQrCore { image_buffer }
}
