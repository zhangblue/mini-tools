pub mod base64;
pub mod csv;
pub mod gen_pass;
pub mod http;
pub mod text;

use crate::opts::base64::Base64SubCommand;
use crate::opts::csv::CsvOpts;
use crate::opts::gen_pass::GenPassOpts;
use crate::{HttpSubCommand, TextSubCommand};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "cli", version, author, about = "rust编写的命令行工具" , long_about = None)]
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

    #[command(subcommand, about = "base64编解码")]
    Base64(Base64SubCommand),

    #[command(subcommand, about = "对文件进行签名")]
    Text(TextSubCommand),

    #[command(subcommand, about = "启动文件服务")]
    Http(HttpSubCommand),
}

/// 自定义的参数校验函数。用于校验输入文件是否存在
pub fn verify_file(filename: &str) -> Result<String, String> {
    // if input is "-" or file exists
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("输入文件不存在".into())
    }
}

/// 验证文件目录是否存在
pub fn verify_path(path: &str) -> Result<PathBuf, String> {
    let path_buf = PathBuf::from(path);
    if path_buf.exists() {
        Ok(path_buf)
    } else {
        Err(format!("没有这个目录:{}", path))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(verify_file("*"), Err("输入文件不存在".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(verify_file("not-exist"), Err("输入文件不存在".into()));
    }
}
