use std::borrow::Borrow;
use std::str::FromStr;

use crate::download;

// use std::env::temp_dir
fn temp_dir() -> std::path::PathBuf {
    return std::path::PathBuf::from_str("/tmp").unwrap();
}

lazy_static! {
    pub static ref TEMP_DIR: std::path::PathBuf = temp_dir().join(module_path!());
    pub static ref TEMP_DIR_O: Option<&'static str> = TEMP_DIR.to_str();
    pub static ref TEMP_DIR_S: &'static str = TEMP_DIR_O.unwrap();
}

#[async_test]
async fn test_download() {
    if !TEMP_DIR.exists() {
        std::fs::create_dir_all(TEMP_DIR.borrow() as &std::path::PathBuf).unwrap();
    }

    // test with invalid URL: " \"<>\\^`{|}"
    let output_dir = download(
        "http://www.rust-lang.org",
        std::path::Path::new(TEMP_DIR_S.borrow() as &'static str),
    )
    .await
    .unwrap();
    assert_eq!(
        std::path::Path::new(output_dir.as_os_str())
            .file_name()
            .unwrap(),
        "www.rust-lang.org"
    )
}
