use std::{net::IpAddr, process::exit};

use clap::Parser;
use tokio::sync::mpsc;

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // 参数校验
    args_validate(&args);

    // 建立channel
    let (rx, mut tx) = mpsc::channel(2);

    // 建立发送端task
    let sender_task = tokio::spawn(async move {
        let mut tasks = vec![];

        for port in args.port_start..=args.port_end {
            // println!("? {}:{}", args.addr, port);
            let rx = rx.clone();
            let task = tokio::spawn(async move {
                let scan_attempt = scan(args.addr, port, rx).await;

                if let Err(err) = scan_attempt {
                    eprintln!("error: {err}");
                }
            });

            tasks.push(task);
        }

        for task in tasks {
            task.await.unwrap();
        }
        drop(rx);
    });

    // 建立接收端task
    let receiver_task = tokio::spawn(async move {
        while let Some((addr, port)) = tx.recv().await {
            println!("={}:{}", addr, port);
        }
    });

    let _ = sender_task.await.unwrap();
    let _ = receiver_task.await.unwrap();

    Ok(())
}

async fn scan(
    addr: IpAddr,
    port: u16,
    results_tx: mpsc::Sender<(IpAddr, u16)>,
) -> Result<(), mpsc::error::SendError<(IpAddr, u16)>> {
    let connection_attempt = tokio::net::TcpStream::connect((addr, port)).await;
    if let Ok(_open) = connection_attempt {
        results_tx.send((addr, port)).await.unwrap();
    };

    Ok(())
}
