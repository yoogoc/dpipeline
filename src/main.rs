use dpipeline::{Pipeline, source::file::CsvSource, sink::file::JsonLinesSink};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 示例：CSV转JSON Lines
    let source = Box::new(CsvSource::new("input.csv"));
    let sink = Box::new(JsonLinesSink::new("output.jsonl"));
    
    let pipeline = Pipeline::new(source, vec![], sink);
    
    pipeline.run().await?;
    
    println!("Data pipeline completed successfully!");
    
    Ok(())
}
