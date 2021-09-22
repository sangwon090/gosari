use std::env;
use std::fs::File;
use gosari::format::Format;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let format = Format::detect(&mut file);

    println!("{:?}", format);
}
