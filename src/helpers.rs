use crate::error::DownloadError;

pub(crate) fn destination(
    uri: impl std::convert::TryInto<http::uri::Uri, Error = http::uri::InvalidUri>,
    target_dir: impl AsRef<std::path::Path>,
) -> Result<std::ffi::OsString, DownloadError> {
    let uri: http::uri::Uri = match uri.try_into() {
        Ok(uri) => Ok(uri),
        Err(e) => Err(DownloadError::ParseUri(e)),
    }?;
    // .map_err(|e| Err(DownloadError::ParseUri(e)))?;

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
