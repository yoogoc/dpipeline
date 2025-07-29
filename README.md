# dpipeline

一个用 Rust 编写的高性能异步数据管道工具，支持多种数据源和输出格式的同步处理。

## 特性

- 🚀 **异步处理**: 基于 Tokio 的高性能异步处理框架
- 📥 **多种数据源**: 支持 CSV、JSON Lines 等格式的数据读取
- 📤 **多种数据输出**: 支持 CSV、JSON Lines 等格式的数据写入
- 🔄 **可扩展架构**: 支持自定义数据源、数据转换和数据输出
- 🛡️ **错误处理**: 完整的错误处理和类型安全
- 📊 **Schema 感知**: 自动推断和管理数据结构

## 架构设计

该项目采用模块化设计，主要包含以下核心组件：

- **Source**: 数据源接口，负责读取数据
- **Sink**: 数据输出接口，负责写入数据
- **Transform**: 数据转换接口，支持数据变换
- **Pipeline**: 数据管道，连接数据源、转换和输出

## 安装

确保你已经安装了 Rust（1.70+）。

```bash
git clone https://github.com/yoogoc/dpipeline.git
cd dpipeline
cargo build --release
```

## 快速开始

### 基本用法

```rust
use dpipeline::{Pipeline, source::file::CsvSource, sink::file::JsonLinesSink};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 CSV 数据源
    let source = Box::new(CsvSource::new("input.csv"));

    // 创建 JSON Lines 输出
    let sink = Box::new(JsonLinesSink::new("output.jsonl"));

    // 创建管道
    let pipeline = Pipeline::new(source, vec![], sink);

    // 运行管道
    pipeline.run().await?;

    println!("数据管道处理完成！");

    Ok(())
}
```

### 运行示例

```bash
# 运行默认示例（CSV 转 JSON Lines）
cargo run

# 或者运行发布版本
cargo run --release
```

## 支持的数据格式

### 数据源 (Source)

- **CsvSource**: CSV 文件读取
  - 支持自定义分隔符
  - 支持有/无头部行
  - 自动 Schema 推断

- **JsonLinesSource**: JSON Lines 文件读取
  - 每行一个 JSON 对象
  - 自动 Schema 推断

### 数据输出 (Sink)

- **CsvSink**: CSV 文件写入
  - 支持自定义分隔符
  - 支持自定义头部行
  - 自动类型转换

- **JsonLinesSink**: JSON Lines 文件写入
  - 每行输出一个 JSON 对象
  - 保持原始数据类型

## API 文档

### 创建 CSV 数据源

```rust
// 基本用法
let source = CsvSource::new("data.csv");

// 自定义配置
let source = CsvSource::new("data.csv")
    .with_delimiter(b';')
    .with_header(false);
```

### 创建数据输出

```rust
// JSON Lines 输出
let sink = JsonLinesSink::new("output.jsonl");

// CSV 输出
let sink = CsvSink::new("output.csv")
    .with_delimiter(b'\t')
    .with_headers(vec!["col1".to_string(), "col2".to_string()]);
```

### 自定义数据转换

```rust
use dpipeline::core::Transform;
use async_trait::async_trait;

struct MyTransform;

#[async_trait]
impl Transform for MyTransform {
    async fn transform(&self, record: Record) -> Result<Vec<Record>> {
        // 实现你的数据转换逻辑
        Ok(vec![record])
    }

    async fn get_output_schema(&self, input_schema: &Schema) -> Result<Schema> {
        // 返回输出 Schema
        Ok(input_schema.clone())
    }
}
```

## 项目结构

```
src/
├── core/           # 核心类型和接口定义
│   ├── error.rs    # 错误类型定义
│   ├── record.rs   # 数据记录类型
│   ├── schema.rs   # Schema 定义
│   └── traits.rs   # 核心 trait 定义
├── source/         # 数据源实现
│   └── file.rs     # 文件数据源
├── sink/           # 数据输出实现
│   └── file.rs     # 文件数据输出
├── pipeline.rs     # 数据管道实现
├── lib.rs          # 库入口
└── main.rs         # 示例程序
```

## 依赖

- `tokio`: 异步运行时
- `async-trait`: 异步 trait 支持
- `serde`: 序列化/反序列化
- `serde_json`: JSON 处理
- `anyhow`: 错误处理
- `thiserror`: 错误类型定义
- `tracing`: 日志记录
- `futures`: 异步工具
- `csv`: CSV 处理
- `tokio-stream`: 异步流处理

## 性能特点

- 基于流式处理，内存占用低
- 异步 I/O，高并发处理能力
- 零拷贝设计，减少内存分配
- 类型安全，编译时错误检查

## 扩展开发

### 添加新的数据源

1. 实现 `Source` trait
2. 在 `source` 模块中添加你的实现
3. 导出新的数据源类型

### 添加新的数据输出

1. 实现 `Sink` trait
2. 在 `sink` 模块中添加你的实现
3. 导出新的数据输出类型
