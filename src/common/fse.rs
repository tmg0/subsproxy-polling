use serde::de::DeserializeOwned;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use toml::de::Error as TomlError;
use toml::Value;

pub fn ensure_file(file_path: &str) -> fs::File {
    ensure_dir(dirname(file_path));
    let file = fs::File::create(file_path).unwrap();
    return file;
}

pub fn ensure_dir(dir_path: &str) {
    if !fs::metadata(dir_path).is_ok() {
        fs::create_dir_all(dir_path).unwrap();
    }
}

pub fn dirname(file_path: &str) -> &str {
    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        parent.to_str().unwrap_or("")
    } else {
        ""
    }
}

pub fn write(mut file: fs::File, content: &str) {
    file.write_all(content.as_bytes()).unwrap();
}

pub fn read_toml<T: DeserializeOwned>(file_path: &str) -> Result<T, TomlError> {
    let mut file = fs::File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parsed: Value = contents.parse()?;
    let config: T = parsed.try_into()?;

    Ok(config)
}
