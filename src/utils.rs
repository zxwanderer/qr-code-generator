use std::ffi::OsStr;
use std::path::Path;

pub fn get_extension(s: &str) -> Result<&str, String> {
    let ext = Path::new(s).extension().and_then(OsStr::to_str);
    match ext {
        None => Err("Output filename extension not found".to_string()),
        Some(e) => Ok(e),
    }
}
