use crate::utils::get_extension;

pub const DEFAULT_NAME: &str = "qr-code.png";
pub const DEFAULT_SIZE: u32 = 32;
pub const EXTENSION_LIST: [&str; 2] = ["bin", "png"];

pub enum OutputMode {
    Binary,
    Image,
}

pub struct Config {
    pub output: String,
    pub url: String,
    pub size: u32,
    pub mode: OutputMode,
}

pub fn get_mode_by_path(path: &str) -> Result<OutputMode, String> {
    let ext = get_extension(path).unwrap();
    match ext {
        "bin" => Ok(OutputMode::Binary),
        "png" => Ok(OutputMode::Image),
        &_ => {
            let err = format!(
                "Invalid output extension {:?}. Valid extension is {:?}",
                ext, EXTENSION_LIST
            );
            return Err(err);
        }
    }
}
