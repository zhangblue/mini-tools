use std::{net::IpAddr, process::exit};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// 扫描地址
    #[arg(long)]
    addr: IpAddr,

    /// 扫描的开始端口
    #[arg(long, default_value_t = 1)]
    port_start: u16,

    /// 扫描的结束端口
    #[arg(long, default_value_t = 1024)]
    port_end: u16,
}

fn args_validate(args: &Args) {
    if args.port_start <= 0 {
        eprintln!("参数错误: port_start 必须大于0");
        exit(2);
    }

    if args.port_end < args.port_start {
        eprintln!("参数错误: port_end 必须大于 port_start");
        exit(2);
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // 参数校验
    args_validate(&args);

    for port in args.port_start..=args.port_end {
        let scan_attempt = scan(args.addr, port).await;
        if let Some((addr, port)) = scan_attempt {
            println!("{}:{}", addr, port);
        }
    }
}

async fn scan(addr: IpAddr, port: u16) -> Option<(IpAddr, u16)> {
    let connection_attempt = tokio::net::TcpStream::connect((addr, port)).await;
    if let Ok(_open) = connection_attempt {
        return Some((addr, port));
    }
    return None;
}
