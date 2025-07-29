use crate::core::{Source, Sink, Transform, Result};

pub struct Pipeline {
    source: Box<dyn Source>,
    transforms: Vec<Box<dyn Transform>>,
    sink: Box<dyn Sink>,
}

impl Pipeline {
    pub fn new(
        source: Box<dyn Source>,
        transforms: Vec<Box<dyn Transform>>,
        sink: Box<dyn Sink>,
    ) -> Self {
        Self {
            source,
            transforms,
            sink,
        }
    }
    
    pub async fn run(mut self) -> Result<()> {
        let mut stream = self.source.read().await?;
        
        while let Some(record_result) = futures::StreamExt::next(&mut stream).await {
            let mut record = record_result?;
            
            for transform in &self.transforms {
                let transformed = transform.transform(record.clone()).await?;
                if let Some(first_record) = transformed.into_iter().next() {
                    record = first_record;
                } else {
                    continue;
                }
            }
            
            self.sink.write(record).await?;
        }
        
        self.sink.close().await?;
        self.source.close().await?;
        
        Ok(())
    }
}