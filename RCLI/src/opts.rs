use clap::Parser;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command(name = "cli", version, author, about = "文件处理工具" , long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}
#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "将CSV文件转换成其他格式的文件")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "生成随机密码")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Parser, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

/// 转换csv文件的参数
#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// 输入文件路径
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    /// 输出文件路径
    #[arg(short, long)]
    // default_value 含义为调用了："output.json".into()后进行赋值
    pub output: Option<String>,

    #[arg(short, long, default_value = "json", value_parser=parse_format)]
    pub format: OutputFormat,

    /// 分隔符
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// CSV文件是否有头
    #[arg(long, default_value_t = true)] // default_value_t 这个是直接赋值
    pub header: bool,
}

/// 随机生成密码的参数
#[derive(Debug, Parser)]
pub struct GenPassOpts {

    /// 密码的长度
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    /// 是否需要大写
    #[arg(long, default_value = "true")]
    pub uppercase: bool,

    /// 是否包含小写
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    /// 是否包含数字
    #[arg(long, default_value_t = true)]
    pub numbers: bool,

    /// 是否包含符号
    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}

/// 自定义的参数校验函数。用于校验输入文件是否存在
fn verify_input_file(filename: &str) -> Result<String, String> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("输入文件不存在".into())
    }
}

// 验证输入的内容是否合法
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // 这里之所以可以将format转成OutputFormat,是因为给 OutputFormat 实现了FromStr trait。
    format.parse::<OutputFormat>()
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
