use clap::Parser;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "使用私钥对消息进行签名")]
    Sign(TextSignOpts),
    #[command(about = "验证消息")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    /// 需要编码的内容
    #[arg(short, long, default_value = "-" ,value_parser=crate::opts::verify_file)]
    pub input: String,

    /// 密钥
    #[arg(short, long, default_value = "-", value_parser=crate::opts::verify_file)]
    pub key: String,

    #[arg(long, default_value = "black3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, default_value = "-", value_parser=crate::opts::verify_file, help = "需要解码的内容")]
    pub input: String,
    #[arg(short, long, help = "key")]
    pub key: String,
    #[arg(short, long, help = "签名")]
    pub sig: String,
    #[arg(long, default_value = "black3", help = "格式")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

// 将 &str转成enum
impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "black3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("无效的format类型")),
        }
    }
}

// 将enum转成&str
impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "black3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

// 实现Display可以将enum 输出
impl Display for TextSignFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
