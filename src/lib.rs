extern crate async_std;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate futures_await_test;

use async_std::prelude::*;

use crate::error::DownloadError;
use crate::helpers::destination;

pub async fn download<E: Into<DownloadError>>(
    uri: impl std::convert::TryInto<http::uri::Uri, Error = E>,
    target_dir: impl AsRef<std::path::Path>,
) -> Result<std::ffi::OsString, DownloadError> {
    let output_file: std::ffi::OsString = destination(uri, target_dir)?;
    return Ok(output_file);
}

async fn download_surf<E: Into<DownloadError>>(
    uri: impl std::convert::TryInto<http::uri::Uri, Error = E>,
    target_dir: impl AsRef<std::path::Path>,
) -> Result<std::ffi::OsString, DownloadError> {
    let uri: http::uri::Uri = match uri.try_into() {
        Ok(uri) => uri,
        Err(e) => return Err(e.into()),
    };
    let output_file: std::ffi::OsString = destination(&uri, target_dir)?;
    async_std::task::block_on(async {
        let body_bytes: &[u8] = &surf::get(uri.to_string()).await?.body_bytes().await?;
        async_std::fs::File::create(&output_file)
            .await?
            .write_all(body_bytes)
            .await?;
        /* .sync_all().await?; */
        Ok(output_file)
    })
}

mod error;
mod helpers;
#[cfg(test)]
mod tests;
