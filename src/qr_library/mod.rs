use crate::qr_library::defines::MyImageBuffer;

mod convert_to_u8;
mod defines;
mod generator;

pub trait ZxQrCode {
    fn convert_to_zx_quad(&self) -> BytesVector;
    fn save_to_image(&self, path: &String) -> Result<(), String>;
}

pub use defines::BytesVector;

struct MyQrCore {
    image_buffer: MyImageBuffer,
}

impl ZxQrCode for MyQrCore {
    fn convert_to_zx_quad(&self) -> BytesVector {
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
