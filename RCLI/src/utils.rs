use std::fs::File;
use std::io::Read;

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        // 这里返回的是stdin。
        Box::new(std::io::stdin())
    } else {
        // 这里返回的是文件读取
        Box::new(File::open(input)?)
    };
    Ok(reader)
}