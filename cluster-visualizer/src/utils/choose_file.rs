use std::path::PathBuf;
use std::path::Path;
use std::process::Command;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

/// Choose file.
///
/// This will launch zenity file chooser.
pub fn choose_file() -> PathBuf {
    let path = Command::new("zenity")
        .arg("--file-selection")
        .output()
        .unwrap()
        .stdout;

    let path = path.strip_suffix(b"\n").unwrap();
    let path = OsStr::from_bytes(&path);
    let path = Path::new(&path);
    path.to_path_buf()
}


