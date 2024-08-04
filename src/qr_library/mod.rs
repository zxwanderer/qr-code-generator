mod convert_to_u8;
mod defines;
mod generator;
mod my_qr_core;

pub trait ZxQrCode {
    fn convert_to_zx_8x32_quad(&self) -> Vec<u8>;
    fn convert_to_image(&self, file_format: &str) -> Result<Vec<u8>, String>;
}

pub use my_qr_core::generate;
