use clap::Parser;
use rcli::{
    Opts, Subcommand, process_base64, process_gen_pass, process_http_serve, process_text,
    to_json_file,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

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
        Subcommand::Text(sum_cmd) => {
            process_text(&sum_cmd)?;
        }
        Subcommand::Http(cmd) => process_http_serve(cmd).await?,
    }
    Ok(())
}
