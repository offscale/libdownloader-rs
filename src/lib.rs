extern crate async_std;
#[macro_use]
extern crate lazy_static;

// Result<OsString, (SendRequestError|std::core::convert::Infallible|actix_http::error::PayloadError)
#[derive(Debug)]
enum DownloadError {
    Io(std::io::Error),
    ParseUri(http::uri::InvalidUri),
    HttpError(http::Error),
    None,
}

impl From<std::io::Error> for DownloadError {
    fn from(e: std::io::Error) -> Self {
        DownloadError::Io(e)
    }
}

impl From<http::uri::InvalidUri> for DownloadError {
    fn from(e: http::uri::InvalidUri) -> Self {
        DownloadError::ParseUri(e)
    }
}

impl From<http::Error> for DownloadError {
    fn from(e: http::Error) -> Self {
        DownloadError::HttpError(e)
    }
}

// #[async_std::main]
fn download(
    uri: impl std::convert::TryInto<http::uri::Uri, Error = http::uri::InvalidUri>,
    target_dir: impl AsRef<std::path::Path>,
) -> Result<std::ffi::OsString, DownloadError> {
    let output_file: std::ffi::OsString = destination(uri, target_dir)?;

    return Ok(output_file);
}

fn destination(
    uri: impl std::convert::TryInto<http::uri::Uri, Error = http::uri::InvalidUri>,
    target_dir: impl AsRef<std::path::Path>,
) -> Result<std::ffi::OsString, DownloadError> {
    let uri: http::uri::Uri = match uri.try_into() {
        Ok(uri) => Ok(uri),
        Err(e) => Err(DownloadError::ParseUri(e)),
    }?;

    let get_host = || -> Result<String, DownloadError> {
        match uri.host() {
            Some(host) => Ok(String::from(host)),
            None => Err(DownloadError::None),
        }
    };
    let path: String = match &uri.path_and_query() {
        Some(path_and_query) => {
            let p: String = path_and_query.path().to_owned();
            if p == "/" {
                get_host()
            } else {
                Ok(p)
            }
        }
        None => get_host(),
    }?;
    drop(get_host);
    drop(uri);
    let output_pathbuf: std::path::PathBuf = target_dir.as_ref().join(path);
    Ok(output_pathbuf.as_os_str().to_owned())
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_download() {
        if !TEMP_DIR.exists() {
            std::fs::create_dir_all(TEMP_DIR.borrow() as &std::path::PathBuf).unwrap();
        }

        // test with invalid URL: " \"<>\\^`{|}"
        let output_dir = download(
            "http://www.rust-lang.org",
            std::path::Path::new(TEMP_DIR_S.borrow() as &'static str),
        )
        .unwrap();
        assert_eq!(
            std::path::Path::new(output_dir.as_os_str())
                .file_name()
                .unwrap(),
            "www.rust-lang.org"
        )
    }
}
