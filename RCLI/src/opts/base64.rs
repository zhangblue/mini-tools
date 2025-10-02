use clap::{Parser, ValueEnum};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "base64 编码")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "base64 解码")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    /// 需要编码的内容
    #[arg(short, long, default_value = "-", value_parser=crate::opts::verify_file)]
    pub input: String,

    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    /// 需要解码的内容
    #[arg(short, long, default_value = "-", value_parser=crate::opts::verify_file)]
    pub input: String,
    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "url-safe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("无效的格式化 format: {}", s)),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url-safe",
        }
    }
}
// 给Base64Format 实现display trait
impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}