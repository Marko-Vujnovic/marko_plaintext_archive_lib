pub fn osstrref_to_String(str_: &std::ffi::OsStr) -> String { str_.to_string_lossy().to_string() }
pub fn to_String(path: &std::path::PathBuf) -> String { osstrref_to_String(path.file_name().unwrap()) }
