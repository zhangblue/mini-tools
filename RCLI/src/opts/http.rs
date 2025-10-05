use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "启动一个web服务")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    /// 文件目录
    #[arg(short, long, default_value = "." ,value_parser=crate::opts::verify_path)]
    pub dir: PathBuf,

    /// 端口
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

