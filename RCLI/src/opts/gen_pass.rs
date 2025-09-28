use clap::Parser;

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