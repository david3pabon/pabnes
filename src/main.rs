// use std::env;
// use std::fs;

mod cpu;

use cpu::CPU;

fn main() {
    let data: Vec<u8> = vec![0x85, 0x10, 0x00];

    let mut cpu = CPU::new();
    cpu.reg_a = 0x55;

    cpu.load_and_run(data);

    // println!("{}", cpu.mem_read(0x10));
}