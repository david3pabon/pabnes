// use std::env;
// use std::fs;

mod cpu;

use cpu::CPU;

fn main() {
    // let mut args: Vec<String> = env::args().collect();
    // args.remove(0);
    // let filename = args.pop().expect("No file");

    // let data: Vec<u8> = fs::read(&filename).expect("Unable to read file");

    let data: Vec<u8> = vec![0xe8, 0xe8, 0x00];

    let mut cpu = CPU::new();

    cpu.reg_x = 0xff;

    cpu.execute(data);

    println!("{}", cpu.reg_x);
}