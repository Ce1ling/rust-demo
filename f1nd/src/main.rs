use std::{env, fmt::Debug, process};

use f1nd::{f1nd, Config};

fn main() {
    let config = Config::from(env::args()).unwrap_or_else(|err| {
        print_err(err);
        process::exit(-1);
    });

    if let Err(err) = f1nd(config) {
        print_err(err);
        process::exit(-1);
    }
}

fn print_err<T: Debug>(err: T) {
    eprintln!("f1nd err: {:?}", err)
}
