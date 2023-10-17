use crate::util::{bits, sext};

pub struct State {
    pub pc_star: i16,
    pub ir: i16,
    pub mem: [i16; 65536],
    pub reg: [i16; 8],
    // 0b100 = 4 = n
    // 0b010 = 2 = z
    // 0b001 = 1 = p
    pub psr: i16, 
}

impl State {
    pub fn print(&self) {
        println!("PROGRAM STATE");
        println!("PC*: x{:0<4X}", self.pc_star);
        println!("PC*: x{}", self.pc_star);
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
        self.ir = self.mem[(self.pc_star as i16) as usize];
        self.pc_star += 1;
        println!(">>> DEBUG: Current instruction is x{:0>4X}", bits(self.ir, 15, 0));
        match bits(self.ir, 15, 12) {
            0b0000 => {
                println!(">>> DEBUG: Executing BR");
                println!(">>> DEBUG: Adding PCOffset9 {:0>16b} to PC* x{:0>4X}", bits(self.ir, 8, 0) as i16, self.pc_star);
                if bits(self.ir, 11, 9) & bits(self.psr, 2, 0) != 0 {
                    self.pc_star = self.pc_star.wrapping_add(bits(self.ir, 8, 0) as i16);
                }
                println!(">>> DEBUG: PC* = x{:0<4X}", self.pc_star);
            }
            0b0001 => {
                println!(">>> DEBUG: Executing ADD");
                if bits(self.ir, 5, 5) == 0 {
                    if bits(self.ir, 4, 3) != 0 {
                        return Err("Error: Malformed instruction: bits [4:3] of ADD using source register must be 0");
                    }
                    println!(">>> DEBUG: ADD mode: source register");
                    self.reg[bits(self.ir, 11, 9) as usize] =
                        (self.reg[bits(self.ir, 8, 6) as usize]) 
                            .wrapping_add(self.reg[bits(self.ir, 2, 0) as usize]);
                } else {
                    println!(">>> DEBUG: ADD mode: immediate int literal");
                    self.reg[bits(self.ir, 11, 9) as usize] =
                        (self.reg[bits(self.ir, 8, 6) as usize]) 
                            .wrapping_add(sext(bits(self.ir, 4, 0), 5));
                }
                self.print();
            }
            0b0101 => {
                println!(">>> DEBUG: Executing AND");
                if bits(self.ir, 5, 5) == 0 {
                    if bits(self.ir, 4, 3) != 0 {
                        return Err("Error: Malformed instruction: bits [4:3] of AND using source register must be 0");
                    }
                    self.reg[bits(self.ir, 11, 9) as usize] =
                        (self.reg[bits(self.ir, 8, 6) as usize]) 
                            & (self.reg[bits(self.ir, 2, 0) as usize]);
                } else {
                    self.reg[bits(self.ir, 11, 9) as usize] =
                        (self.reg[bits(self.ir, 8, 6) as usize]) 
                            & (sext(bits(self.ir, 4, 0), 5));
                }
                self.print();
            }
            _ => {
                println!(">>> DEBUG: UNIMPLEMENTED INSTRUCTION");
            }
        }
        println!(">>> DEBUG: Instruction complete\n");
        Ok(())
    }
}
