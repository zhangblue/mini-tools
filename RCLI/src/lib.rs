mod opts;
mod process;
pub use opts::{
    Opts, Subcommand, base64::Base64DecodeOpts, base64::Base64EncodeOpts, base64::Base64SubCommand,
};
pub use process::{process_base64, process_gen_pass, to_json_file};
