extern crate async_std;
#[macro_use]
extern crate lazy_static;

use crate::error::DownloadError;
use crate::helpers::destination;

pub async fn download(
    uri: impl std::convert::TryInto<http::uri::Uri, Error = http::uri::InvalidUri>,
    target_dir: impl AsRef<std::path::Path>,
) -> Result<std::ffi::OsString, DownloadError> {
    let output_file: std::ffi::OsString = destination(uri, target_dir)?;
    return Ok(output_file);
}

async fn download_surf(
    uri: impl std::convert::TryInto<http::uri::Uri, Error = http::uri::InvalidUri>,
    target_dir: impl AsRef<std::path::Path>,
) -> Result<std::ffi::OsString, DownloadError> {
    let output_file: std::ffi::OsString = destination(uri, target_dir)?;
    async_std::task::block_on(async {
        let mut res = surf::get(uri.into()).await?;
        async_std::fs::File::create(&output_file)
            .write(res.body_bytes().await?)
            .await?;
        Ok(output_file)
    })
}

mod error;
mod helpers;
#[cfg(test)]
mod tests;
