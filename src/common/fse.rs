use std::fs;
use std::io::Write;
use std::path::Path;

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
