use crate::opts::OutputFormat;
use serde_json::Value;
use std::fs;

// 将数据写成json文件
pub fn to_json_file(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut ret = Vec::with_capacity(200);
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}
