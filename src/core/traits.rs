use crate::core::{Record, Schema, Result};
use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

pub type RecordStream = Pin<Box<dyn Stream<Item = Result<Record>> + Send>>;

#[async_trait]
pub trait Source: Send + Sync {
    async fn get_schema(&self) -> Result<Schema>;
    
    async fn read(&self) -> Result<RecordStream>;
    
    async fn close(&self) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
pub trait Sink: Send + Sync {
    async fn write(&mut self, record: Record) -> Result<()>;
    
    async fn write_batch(&mut self, records: Vec<Record>) -> Result<()> {
        for record in records {
            self.write(record).await?;
        }
        Ok(())
    }
    
    async fn flush(&mut self) -> Result<()> {
        Ok(())
    }
    
    async fn close(&mut self) -> Result<()> {
        self.flush().await
    }
}

#[async_trait]
pub trait Transform: Send + Sync {
    async fn transform(&self, record: Record) -> Result<Vec<Record>>;
    
    async fn get_output_schema(&self, input_schema: &Schema) -> Result<Schema>;
}

pub enum SourceMode {
    Batch,
    Stream,
}

pub enum SinkMode {
    Append,
    Overwrite,
    Update,
}