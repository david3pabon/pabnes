
pub struct CPU {
    pub reg_a: u8,
    pub status: u8,
    pub pc: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg_a: 0,
            status: 0,
            pc: 0,
        }
    }

    pub fn run(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let opscode = program[self.pc as usize];
            self.pc += 1;

            match opscode {
                _ => println!("Instruction: {}", opscode)
            }
        }
    }
}