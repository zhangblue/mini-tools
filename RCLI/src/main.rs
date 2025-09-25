use clap::Parser;
use rcli::{Opts, Subcommand, to_json_file};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            to_json_file(&opts.input, output, opts.format)?;
        }
    }

    Ok(())
}
