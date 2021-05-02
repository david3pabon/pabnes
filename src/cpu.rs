
pub struct CPU {
    pub reg_a: u8,
    pub reg_x: u8,
    pub status: u8,
    pub pc: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg_a: 0,
            reg_x: 0,
            status: 0,
            pc: 0,
        }
    }

    pub fn execute(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let opscode = program[self.pc as usize];
            self.pc += 1;

            match opscode {
                0xA9 => {
                    let param = program[self.pc as usize];
                    self.pc += 1;
                    
                    self.lda(param);
                }
    
                0xAA => self.tax(),
    
                0x00 => return,
                
                _ => println!("Instruction: {}", opscode)
            }
        }
    }

    fn lda(&mut self, value: u8) {
        self.reg_a = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }
  
    fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.update_zero_and_negative_flags(self.reg_x);
    }
   
     fn update_zero_and_negative_flags(&mut self, result: u8) {
         if result == 0 {
             self.status = self.status | 0b0000_0010;
         } else {
             self.status = self.status & 0b1111_1101;
         }
 
         if result & 0b1000_0000 != 0 {
             self.status = self.status | 0b1000_0000;
         } else {
             self.status = self.status & 0b0111_1111;
         }
     }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lda_immidiate_load_data() {
       let mut cpu = CPU::new();
       
       cpu.execute(vec![0xa9, 0x05, 0x00]);

       assert_eq!(cpu.reg_a, 0x05);
       assert!(cpu.status & 0b0000_0010 == 0b00);
       assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        
        cpu.execute(vec![0xa9, 0x00, 0x00]);
        
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
    
    #[test]
    fn test_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.reg_a = 10;

        cpu.execute(vec![0xaa, 0x00]);

        assert_eq!(cpu.reg_x, 10)
    }

    #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();

       cpu.execute(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.reg_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0xff;

        cpu.execute(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 1)
    }
}