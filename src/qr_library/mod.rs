mod convert_to_u8;
mod defines;
mod generator;
mod my_qr_core;

pub trait ZxQrCode {
    fn convert_to_zx_quad(&self) -> Vec<u8>;
    fn convert_to_image(&self, file_format: &String) -> Result<Vec<u8>, String>;
    fn save_to_image(&self, path: &String) -> Result<(), String>;
}

pub use my_qr_core::generate;
