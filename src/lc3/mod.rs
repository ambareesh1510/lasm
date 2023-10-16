use crate::util::{bits, sext};

pub struct State {
    pub pc_star: u16,
    pub ir: u16,
    pub mem: [u16; 65536],
    pub reg: [u16; 8],
    // 0b100 = 4 = n
    // 0b010 = 2 = z
    // 0b001 = 1 = p
    pub psr: u16, 
}

impl State {
    pub fn print(&self) {
        println!("PROGRAM STATE");
        println!("PC*: x{:0<4X}", self.pc_star);
        println!("IR : x{:0<4X}", self.ir);
        println!("");

        println!("REGISTERS");
        for i in 0..8 {
            println!("R{i} : x{:0<4X}", self.reg[i]);
        }
        println!("");

        println!("FLAGS");
        println!("CC : {:0<3b}", bits(self.psr, 2, 0));
    }

    pub fn execute_next_instruction(&mut self) -> Result<(), &str> {
        self.ir = self.mem[self.pc_star as usize];
        self.pc_star += 1;
        println!("bits = {:b}", bits(self.ir, 15, 0));
        match bits(self.ir, 15, 12) {
            0b0000 => {
                println!("Executing BR");
                if bits(self.ir, 11, 9) & bits(self.psr, 2, 0) != 0 {
                    println!("{}", bits(self.ir, 8, 0));
                    println!("{}", sext(bits(self.ir, 8, 0)));
                    self.pc_star = (self.pc_star as i16 + sext(bits(self.ir, 8, 0))) as u16;
                }
                println!("PC* = {:0<4X}", self.pc_star);
            }
            /*
            0b0001 => {
                println!("Executing ADD");
                if bits(self.ir, 5, 5) == 0 {
                    self.reg[bits(self.ir, 11, 9) as usize] = sext(self.reg[bits(self.ir, 8, 6) as usize]) + sext(self.reg[bits(self.ir, 2, 0) as usize]);
                }
            }
            */
            _ => {
                println!("UNIMPLEMENTED INSTRUCTION");
            }
        }
        Ok(())
    }
}
