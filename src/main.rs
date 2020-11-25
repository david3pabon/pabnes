use std::env;
use std::fs;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let filename = args.pop().expect("No file");

    let data: Vec<u8> = fs::read(&filename).expect("Unable to read file");
    let bytes = &data[0..3];
    assert_eq!(bytes, "NES".as_bytes());
}