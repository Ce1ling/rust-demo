use playground::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("playground err: {:?}", err);
    }
}
