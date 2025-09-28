use clap::Parser;
use rcli::{Opts, Subcommand, process_base64, process_gen_pass, to_json_file};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            to_json_file(&opts.input, output, opts.format)?;
        }
        Subcommand::GenPass(opts) => {
            process_gen_pass(&opts)?;
        }
        Subcommand::Base64(sub_cmd) => {
            process_base64(&sub_cmd)?;
        }
    }
    Ok(())
}
