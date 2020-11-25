use std::env;
use std::fs;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let filename = match args.pop() {
        Some(val) => val,
        None => panic!("No such file"),
    };

    let data: Vec<u8> = fs::read(&filename).expect("Unable to read file");
    let byte = *data.get(0).unwrap();
    assert_eq!(78, byte);
}