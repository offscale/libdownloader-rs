#[macro_use]
extern crate lazy_static;

use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::str::FromStr;

use actix_rt;
use actix_web::client::Client;
use async_std::fs::File;
use async_std::prelude::*;
use http::uri::Uri;

// Result<OsString, (SendRequestError|std::core::convert::Infallible|actix_http::error::PayloadError)
#[actix_rt::main]
async fn download(uri: &'static str, target_dir: &'static str) -> Result<OsString, ()> {
    let client: Client = Client::default();
    let uri: Uri = Uri::from_str(uri).unwrap();

    // Create request builder and send request
    let response = client
        .get(&uri)
        .header("User-Agent", "Actix-web")
        .send()
        .await; // <- Send http request

    println!("Response: {:?}", response);

    let get_host = || uri.host().unwrap().to_owned();

    let path: String = match &uri.path_and_query() {
        Some(path_and_query) => {
            let p: String = path_and_query.path().to_owned();
            if p == "/" {
                get_host()
            } else {
                p
            }
        }
        None => get_host(),
    };

    let output_pathbuf: PathBuf = PathBuf::from_str(target_dir).unwrap().join(path);
    let output_file: &OsStr = output_pathbuf.as_os_str();
    let mut file: File = File::create(output_file).await.unwrap();
    file.write_all(&response.unwrap().body().await.unwrap())
        .await
        .unwrap();
    // file.write_all(b"Hello, world!").await.unwrap();
    return Ok(output_file.to_owned());
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    // use std::env::temp_dir;
    use std::fs::create_dir_all;
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::download;

    fn temp_dir() -> PathBuf {
        return PathBuf::from_str("/tmp").unwrap();
    }

    lazy_static! {
        pub static ref TEMP_DIR: PathBuf = temp_dir().join(module_path!());
        pub static ref TEMP_DIR_O: Option<&'static str> = TEMP_DIR.to_str();
        pub static ref TEMP_DIR_S: &'static str = TEMP_DIR_O.unwrap();
    }

    #[test]
    fn test_download() {
        if !TEMP_DIR.exists() {
            create_dir_all(TEMP_DIR.borrow() as &PathBuf).unwrap();
        }

        //let output_dir =
        download(
            "http://www.rust-lang.org",
            TEMP_DIR_S.borrow() as &'static str,
        )
        .unwrap();
        //assert_eq!(output_dir, "foo")
    }
}
