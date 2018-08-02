#![feature(rust_2018_preview)]
#![feature(proc_macro_path_invoc)]
#![warn(rust_2018_idioms)]

use quicli::prelude::structopt::StructOpt;

/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
}

fn main() {}
