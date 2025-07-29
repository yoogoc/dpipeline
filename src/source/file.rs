use crate::core::{Source, Record, Schema, Field, DataType, Result, PipelineError, RecordStream};
use async_trait::async_trait;
use futures::stream::{StreamExt};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

pub struct CsvSource {
    file_path: String,
    has_header: bool,
    delimiter: u8,
}

impl CsvSource {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        Self {
            file_path: file_path.as_ref().to_string_lossy().into_owned(),
            has_header: true,
            delimiter: b',',
        }
    }
    
    pub fn with_delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }
    
    pub fn with_header(mut self, has_header: bool) -> Self {
        self.has_header = has_header;
        self
    }
}

#[async_trait]
impl Source for CsvSource {
    async fn get_schema(&self) -> Result<Schema> {
        let file = File::open(&self.file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        
        if let Some(first_line) = lines.next_line().await? {
            let headers: Vec<String> = if self.has_header {
                first_line.split(self.delimiter as char)
                    .map(|s| s.trim().to_string())
                    .collect()
            } else {
                (0..first_line.split(self.delimiter as char).count())
                    .map(|i| format!("column_{}", i))
                    .collect()
            };
            
            let fields = headers.into_iter()
                .map(|name| Field {
                    name,
                    data_type: DataType::String,
                    nullable: true,
                    description: None,
                })
                .collect();
            
            Ok(Schema::new(fields))
        } else {
            Err(PipelineError::Source(anyhow::anyhow!("Empty CSV file")))
        }
    }
    
    async fn read(&self) -> Result<RecordStream> {
        let file = File::open(&self.file_path).await?;
        let reader = BufReader::new(file);
        let lines = LinesStream::new(reader.lines());
        
        let schema = self.get_schema().await?;
        let field_names: Vec<String> = schema.field_names().into_iter().map(|s| s.to_string()).collect();
        let has_header = self.has_header;
        let delimiter = self.delimiter;
        
        let stream = lines
            .enumerate()
            .filter_map(move |(index, line_result)| {
                let field_names = field_names.clone();
                async move {
                    match line_result {
                        Ok(line) => {
                            if has_header && index == 0 {
                                return None;
                            }
                            
                            let values: Vec<&str> = line.split(delimiter as char).collect();
                            let mut data = HashMap::new();
                            
                            for (i, value) in values.iter().enumerate() {
                                if let Some(field_name) = field_names.get(i) {
                                    data.insert(
                                        field_name.clone(),
                                        Value::String(value.trim().to_string())
                                    );
                                }
                            }
                            
                            Some(Ok(Record::with_data(data)))
                        }
                        Err(e) => Some(Err(PipelineError::Io(e))),
                    }
                }
            });
        
        Ok(Box::pin(stream))
    }
}

pub struct JsonLinesSource {
    file_path: String,
}

impl JsonLinesSource {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        Self {
            file_path: file_path.as_ref().to_string_lossy().into_owned(),
        }
    }
}

#[async_trait]
impl Source for JsonLinesSource {
    async fn get_schema(&self) -> Result<Schema> {
        let file = File::open(&self.file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        
        if let Some(first_line) = lines.next_line().await? {
            let json_value: Value = serde_json::from_str(&first_line)?;
            
            if let Some(obj) = json_value.as_object() {
                let fields = obj.keys()
                    .map(|key| Field {
                        name: key.clone(),
                        data_type: DataType::Json,
                        nullable: true,
                        description: None,
                    })
                    .collect();
                
                Ok(Schema::new(fields))
            } else {
                Err(PipelineError::Schema("First line is not a JSON object".to_string()))
            }
        } else {
            Err(PipelineError::Source(anyhow::anyhow!("Empty JSON Lines file")))
        }
    }
    
    async fn read(&self) -> Result<RecordStream> {
        let file = File::open(&self.file_path).await?;
        let reader = BufReader::new(file);
        let lines = LinesStream::new(reader.lines());
        
        let stream = lines.filter_map(|line_result| {
            async move {
                match line_result {
                    Ok(line) => {
                        match serde_json::from_str::<Value>(&line) {
                            Ok(Value::Object(obj)) => {
                                let data = obj.into_iter().collect();
                                Some(Ok(Record::with_data(data)))
                            }
                            Ok(_) => Some(Err(PipelineError::Schema(
                                "Line is not a JSON object".to_string()
                            ))),
                            Err(e) => Some(Err(PipelineError::Serialization(e))),
                        }
                    }
                    Err(e) => Some(Err(PipelineError::Io(e))),
                }
            }
        });
        
        Ok(Box::pin(stream))
    }
}