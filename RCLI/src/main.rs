use clap::Parser;
use rcli::{process_gen_pass, to_json_file, Opts, Subcommand};

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
    }

    Ok(())
}
