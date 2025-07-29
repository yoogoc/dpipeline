use crate::core::{Record, Result, Sink};
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};

pub struct CsvSink {
    file_path: String,
    delimiter: u8,
    headers: Option<Vec<String>>,
    writer: Option<BufWriter<tokio::fs::File>>,
    headers_written: bool,
}

impl CsvSink {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        Self {
            file_path: file_path.as_ref().to_string_lossy().into_owned(),
            delimiter: b',',
            headers: None,
            writer: None,
            headers_written: false,
        }
    }

    pub fn with_delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }

    pub fn with_headers(mut self, headers: Vec<String>) -> Self {
        self.headers = Some(headers);
        self
    }

    async fn ensure_writer(&mut self) -> Result<()> {
        if self.writer.is_none() {
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&self.file_path)
                .await?;
            self.writer = Some(BufWriter::new(file));
        }
        Ok(())
    }

    async fn write_headers_if_needed(&mut self, record: &Record) -> Result<()> {
        if !self.headers_written {
            let headers = if let Some(ref headers) = self.headers {
                headers.clone()
            } else {
                record.data.keys().cloned().collect()
            };

            if let Some(ref mut writer) = self.writer {
                let header_line = headers.join(&(self.delimiter as char).to_string());
                writer.write_all(header_line.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }

            self.headers_written = true;
        }
        Ok(())
    }
}

#[async_trait]
impl Sink for CsvSink {
    async fn write(&mut self, record: Record) -> Result<()> {
        self.ensure_writer().await?;
        self.write_headers_if_needed(&record).await?;

        let headers = if let Some(ref headers) = self.headers {
            headers.clone()
        } else {
            record.data.keys().cloned().collect()
        };

        let values: Vec<String> = headers
            .iter()
            .map(|key| {
                record
                    .data
                    .get(key)
                    .map(|v| match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => String::new(),
                        _ => serde_json::to_string(v).unwrap_or_else(|_| String::new()),
                    })
                    .unwrap_or_default()
            })
            .collect();

        if let Some(ref mut writer) = self.writer {
            let line = values.join(&(self.delimiter as char).to_string());
            writer.write_all(line.as_bytes()).await?;
            writer.write_all(b"\n").await?;
        }

        Ok(())
    }

    async fn flush(&mut self) -> Result<()> {
        if let Some(ref mut writer) = self.writer {
            writer.flush().await?;
        }
        Ok(())
    }

    async fn close(&mut self) -> Result<()> {
        self.flush().await?;
        self.writer = None;
        Ok(())
    }
}

pub struct JsonLinesSink {
    file_path: String,
    writer: Option<BufWriter<tokio::fs::File>>,
}

impl JsonLinesSink {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        Self {
            file_path: file_path.as_ref().to_string_lossy().into_owned(),
            writer: None,
        }
    }

    async fn ensure_writer(&mut self) -> Result<()> {
        if self.writer.is_none() {
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&self.file_path)
                .await?;
            self.writer = Some(BufWriter::new(file));
        }
        Ok(())
    }
}

#[async_trait]
impl Sink for JsonLinesSink {
    async fn write(&mut self, record: Record) -> Result<()> {
        self.ensure_writer().await?;

        let json_line = serde_json::to_string(&record.data)?;

        if let Some(ref mut writer) = self.writer {
            writer.write_all(json_line.as_bytes()).await?;
            writer.write_all(b"\n").await?;
        }

        Ok(())
    }

    async fn flush(&mut self) -> Result<()> {
        if let Some(ref mut writer) = self.writer {
            writer.flush().await?;
        }
        Ok(())
    }

    async fn close(&mut self) -> Result<()> {
        self.flush().await?;
        self.writer = None;
        Ok(())
    }
}
