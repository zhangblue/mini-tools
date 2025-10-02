use clap::{Parser, ValueEnum};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// 转换csv文件的参数
#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// 输入文件路径
    #[arg(short, long, value_parser = crate::opts::verify_file)]
    pub input: String,

    /// 输出文件路径
    #[arg(short, long)]
    // default_value 含义为调用了："output.json".into()后进行赋值
    pub output: Option<String>,

    #[arg(short, long, default_value = "json")]
    pub format: OutputFormat,

    /// 分隔符
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// CSV文件是否有头
    #[arg(long, default_value_t = true)] // default_value_t 这个是直接赋值
    pub header: bool,
}

#[derive(Debug, Parser, Copy, Clone, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// OutputFormat 实现 FromStr trait，用于在对&str.parse()函数转成 OutputFormat类型时使用
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("无效的format类型")),
        }
    }
}

// 实现 Display trait 可以把 OutputFormat 输出成文本。
impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Into::<&str>::into(*self) 这样的写法是因为实现了给&str实现了 From<OutputFormat> trait，这个trait就是在掉用 .into() 函数时所调用的底层trait
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
