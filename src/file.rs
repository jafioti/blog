#![allow(unused)]

use std::{io::Write, path::Path};

use axum::{
    http::{
        header::{CACHE_CONTROL, CONTENT_ENCODING, CONTENT_TYPE},
        HeaderValue, StatusCode,
    },
    response::{IntoResponse, Response},
};
use chrono::Duration;
use flate2::{write::GzEncoder, Compression};
use tokio::io::AsyncReadExt;

pub struct File {
    file: Vec<u8>,
    mime_type: String,
    encoding: Option<String>,
    cache_time: u64,
}

impl File {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self, tokio::io::Error> {
        Ok(Self {
            file: {
                let mut bytes = vec![];
                tokio::fs::File::open(&path)
                    .await?
                    .read_to_end(&mut bytes)
                    .await?;
                bytes
            },
            mime_type: mime_guess::from_path(&path)
                .first_raw()
                .unwrap_or("")
                .to_string(),
            encoding: None,
            cache_time: 604800, // Defaults to 1 week
        })
    }

    pub fn compress(mut self) -> std::io::Result<Self> {
        self.encoding = Some("gzip".to_string());
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(&self.file)?;
        self.file = e.finish()?;
        Ok(self)
    }

    pub fn cache(self, time: Duration) -> Self {
        Self {
            cache_time: time.num_seconds() as u64,
            ..self
        }
    }

    pub fn mime_type<T: ToString>(mut self, mime_type: T) -> Self {
        self.mime_type = mime_type.to_string();
        self
    }
}

impl IntoResponse for File {
    fn into_response(self) -> Response {
        let mut response = Response::builder().status(StatusCode::OK).header(
            CONTENT_TYPE,
            HeaderValue::from_str(&self.mime_type).unwrap(),
        );
        if let Some(encoding) = &self.encoding {
            response = response.header(CONTENT_ENCODING, HeaderValue::from_str(encoding).unwrap());
        }
        if self.cache_time > 0 {
            response = response.header(CACHE_CONTROL, format!("max-age={}", self.cache_time));
        }
        response
            .body(self.file.into_response())
            .unwrap()
            .into_response()
    }
}

/// Exists only to handle the funky string replacements done for embedding
pub struct StringFile {
    pub file: String,
    mime_type: String,
    encoding: Option<String>,
    cache_time: u64,
}

impl StringFile {
    pub async fn load(path: &str) -> Result<Self, tokio::io::Error> {
        Ok(Self {
            file: {
                let mut s = String::new();
                tokio::fs::File::open(path)
                    .await?
                    .read_to_string(&mut s)
                    .await?;
                s
            },
            encoding: None,
            mime_type: mime_guess::from_path(path)
                .first_raw()
                .unwrap_or("")
                .to_string(),
            cache_time: 604800, // Defaults to 1 week
        })
    }

    pub fn cache(mut self, time: Duration) -> Self {
        self.cache_time = time.num_seconds() as u64;
        self
    }

    pub fn mime_type<T: ToString>(mut self, mime_type: T) -> Self {
        self.mime_type = mime_type.to_string();
        self
    }
}

impl IntoResponse for StringFile {
    fn into_response(self) -> Response {
        let mut response = Response::builder().status(StatusCode::OK).header(
            CONTENT_TYPE,
            HeaderValue::from_str(&self.mime_type).unwrap(),
        );

        if self.cache_time > 0 {
            response = response.header(CACHE_CONTROL, format!("max-age={}", self.cache_time));
        }

        response.body(self.file).unwrap().into_response()
    }
}
