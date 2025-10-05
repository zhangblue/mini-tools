mod opts;
mod process;
mod utils;

pub use opts::{
    Opts, Subcommand, base64::Base64DecodeOpts, base64::Base64EncodeOpts, base64::Base64SubCommand,
    http::HttpServeOpts, http::HttpSubCommand, text::TextSubCommand,
};
pub use process::{process_base64, process_gen_pass, process_http_serve, process_text, to_json_file};
pub use utils::get_reader;
