use merger::{Conf, Merger};
use std::env;//todo: Use 'clap' crate for cmd line args
use std::process;

fn main() {
    let merger = match Conf::new(env::args()) {
        Ok(conf) => Merger::new(conf),
        Err(err) => {
            eprintln!("Error in configuration: {}", err);
            process::exit(-1);
        }
    };

    if let Err(e) = merger.start() {
        eprintln!("Merge stopped with error: {}", e);
        process::exit(-1);
    }
}
