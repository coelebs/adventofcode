use std::env;
use std::fs;
use env_logger::Env;
use log::error;

pub fn aocinit(callback :fn(String)) {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("usage: {} <FILENAME>", args[0]);
        return; //TODO figure out returning errors without creating my own
    }

    let input = fs::read_to_string(&args[1])
        .expect("Should be able to read input")
        .trim()
        .to_string();
    callback(input);
}
