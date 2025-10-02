use crate::opts::base64::Base64Format;
use crate::{Base64SubCommand, get_reader};
use base64::Engine;
use base64::prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};
use std::io::Read;

pub fn process_base64(sub_command: &Base64SubCommand) -> anyhow::Result<()> {
    match sub_command {
        Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
        Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
    }
    Ok(())
}

fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    // 这里有个点需要注意。因为stdin和file返回的是两个不同的类型，所以不能同时返回给reader。所以这里需要找到这里俩的共同点：这俩都实现了Read trait。所以reader的类型可以是 Box<dyn Read>
    let mut reader: Box<dyn Read> = get_reader(input.trim())?;
    // 因为都实现了Read trait，所以都可以使用 read_to_end 读取数据
    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer);

    let encode = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&buffer),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&buffer),
    };
    println!();
    print!("{}", encode);
    Ok(())
}

fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    // 这里有个点需要注意。因为stdin和file返回的是两个不同的类型，所以不能同时返回给reader。所以这里需要找到这里俩的共同点：这俩都实现了Read trait。所以reader的类型可以是 Box<dyn Read>
    let mut reader = get_reader(input)?;

    // 因为都实现了Read trait，所以都可以使用 read_to_end 读取数据
    let mut buffer = String::new();
    let _ = reader.read_to_string(&mut buffer);
    let buffer = buffer.trim();

    let decode = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buffer),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(buffer),
    }?;

    let decoded = String::from_utf8(decode)?;
    println!("{}", decoded);
    Ok(())
}
