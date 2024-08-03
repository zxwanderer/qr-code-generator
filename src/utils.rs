use crate::qr_library::BytesVector;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn get_extension(s: &str) -> Result<&str, String> {
    let ext = Path::new(s).extension().and_then(OsStr::to_str);
    match ext {
        None => Err("Output filename extension not found".to_string()),
        Some(e) => Ok(e),
    }
}

pub fn save_buffer(buffer: &BytesVector, path: &String) -> Result<(), String> {
    let mut file = fs::File::create(path).unwrap();
    file.write_all(&buffer.to_vec()).unwrap();
    Ok(())
}
