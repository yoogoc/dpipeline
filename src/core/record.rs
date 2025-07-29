use crate::core::{Schema, PipelineError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub data: HashMap<String, Value>,
    pub metadata: HashMap<String, String>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_data(data: HashMap<String, Value>) -> Self {
        Self {
            data,
            metadata: HashMap::new(),
        }
    }
    
    pub fn set_field(&mut self, name: String, value: Value) {
        self.data.insert(name, value);
    }
    
    pub fn get_field(&self, name: &str) -> Option<&Value> {
        self.data.get(name)
    }
    
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
    
    pub fn validate_against_schema(&self, schema: &Schema) -> Result<()> {
        for field in &schema.fields {
            let value = self.data.get(&field.name);
            
            if value.is_none() && !field.nullable {
                return Err(PipelineError::Schema(
                    format!("Required field '{}' is missing", field.name)
                ));
            }
            
            if let Some(value) = value {
                if !self.is_value_compatible_with_type(value, &field.data_type) {
                    return Err(PipelineError::Schema(
                        format!("Field '{}' has incompatible type", field.name)
                    ));
                }
            }
        }
        Ok(())
    }
    
    fn is_value_compatible_with_type(&self, value: &Value, expected_type: &crate::core::DataType) -> bool {
        use crate::core::DataType;
        match (value, expected_type) {
            (Value::String(_), DataType::String) => true,
            (Value::Number(n), DataType::Integer) if n.is_i64() => true,
            (Value::Number(n), DataType::Float) if n.is_f64() => true,
            (Value::Bool(_), DataType::Boolean) => true,
            (Value::String(_), DataType::DateTime) => true, // Assume string represents datetime
            (_, DataType::Json) => true, // Any JSON value is acceptable
            (Value::String(_), DataType::Bytes) => true, // Base64 encoded bytes
            _ => false,
        }
    }
}

impl Default for Record {
    fn default() -> Self {
        Self::new()
    }
}