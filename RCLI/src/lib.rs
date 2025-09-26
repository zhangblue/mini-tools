mod opts;
mod process;
pub use opts::{GenPassOpts, Opts, Subcommand};
pub use process::{process_gen_pass, to_json_file};
