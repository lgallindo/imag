#[macro_use] extern crate clap;

use cli::Config;
use runtime::Runtime;

mod cli;
mod runtime;
mod module;
mod storage;

fn main() {
    let mut config = Config::new();
    cli::configure(&mut config);

    let rt = Runtime::new(config);

    println!("Hello, world!");
}