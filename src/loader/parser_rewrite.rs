use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum Token {
    Op(String),
    Register(i16),
    NumericLiteral(i16),
    StringLiteral(String),
    Label(String),
    Comma,
    Newline,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Op(op_name) => {
                write!(f, "{} {}", "instruction", *op_name)
            }
            Self::Register(register_number) => {
                write!(f, "{} {}", "register", (*register_number).to_string())
            }
            Self::NumericLiteral(val) => {
                write!(f, "{} {}", "numeric literal", (*val).to_string())
            }
            Self::StringLiteral(string_val) => {
                write!(f, "{} {}", "string literal", *string_val)
            }
            Self::Label(label_name) => {
                write!(f, "{} {}", "label", *label_name)
            }
            Self::Comma => {
                write!(f, "comma")
            }
            _ => write!(f, "whitespace"),
        }
    }
}

struct ParserState {
    cursor: usize,
    characters: Vec<char>,
    parsed_tokens: Vec<Token>,
    line_number: usize,
}

impl ParserState {
    fn new(s: &str) -> Self {
        Self {
            cursor: 0,
            characters: s.to_lowercase().chars().collect::<Vec<char>>(),
            parsed_tokens: vec![],
            line_number: 1,
        }
    }

    fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(c) => {
                self.cursor += 1;
                Some(c)
            }
            None => None,
        }
    }

    fn push_token(&mut self, t: Token) {
        self.parsed_tokens.push(t);
    }

    fn increment_line_number(&mut self) {
        self.line_number += 1;
    }

    fn get_token(&self, addr: usize) -> &Token {
        &self.parsed_tokens[addr]
    }

    fn parse_one_token(&mut self) -> Result<(), String> {
        if self.peek() == None {
            return Err(format!("End of file"));
        }
        let current_char = self.pop().unwrap().clone();
        let next_char = self.peek();
        if let c = current_char {
            // temp_string.push(*c);
            match c {
                ' ' => {},
                // ' ' => self.push_token(Token::Space),
                ',' => self.push_token(Token::Comma),
                '\n' => {
                    self.push_token(Token::Newline);
                    self.increment_line_number();
                }
                '\r' => {
                    self.pop();
                    self.push_token(Token::Newline);
                    self.increment_line_number();
                }
                'r' if next_char.is_some() && next_char.unwrap().is_ascii_digit() => {
                    let mut temp_number_string = String::new();
                    loop {
                        match self.peek() {
                            Some(' ') | Some(',') | Some('\n') | Some('\r') => break,
                            Some(_) => {
                                temp_number_string.push(*(self.pop().unwrap()));
                            }
                            None => break,
                        }
                    }
                    match i16::from_str_radix(&temp_number_string, 10) {
                        Ok(val) => {
                            if 0 <= val && val <= 7 {
                                self.push_token(Token::Register(val));
                            } else {
                                return Err(format!("Register number is out of bounds (should be between R0 and R7; found R{val})"));
                            }
                        }
                        Err(e) => {
                            return Err(format!("Failed to parse register number: {e}"));
                        }
                    }
                }
                '\"' => {
                    let mut temp_string = String::new();
                    loop {
                        match self.peek() {
                            Some('\"') => {
                                self.pop();
                                break;
                            }
                            Some(_) => {
                                temp_string.push(*(self.pop().unwrap()));
                            }
                            None => return Err(format!("Unexpected EOF")),
                        }
                    }
                    self.push_token(Token::StringLiteral(temp_string));
                }
                'x' /*if next_char.is_some() && next_char.unwrap().is_ascii_hexdigit()*/ => {
                    let mut temp_number_string = String::new();
                    loop {
                        match self.peek() {
                            Some(' ') | Some(',') | Some('\n') | Some('\r') => break,
                            Some(_) => {
                                temp_number_string.push(*(self.pop().unwrap()));
                            }
                            None => break,
                        }
                    }
                    // println!("{}", temp_number_string);
                    match u16::from_str_radix(&temp_number_string, 16) {
                        Ok(val) => {
                            self.push_token(Token::NumericLiteral(val as i16));
                        }
                        Err(e) => {
                            return Err(format!("Failed to parse hexadecimal numeric literal: {e}"));
                        }
                    }
                }
                '-' if next_char.is_some() && next_char.unwrap().is_ascii_digit() => {
                    let mut temp_number_string = String::new();
                    temp_number_string.push(current_char);
                    loop {
                        match self.peek() {
                            Some(' ') | Some(',') | Some('\n') | Some('\r') => break,
                            Some(_) => {
                                temp_number_string.push(*(self.pop().unwrap()));
                            }
                            None => break,
                        }
                    }
                    match i16::from_str_radix(&temp_number_string, 10) {
                        Ok(val) => {
                            self.push_token(Token::NumericLiteral(val as i16));
                        }
                        Err(e) => {
                            return Err(format!("Failed to parse hexadecimal numeric literal: {e}"));
                        }
                    }
                }
                c if c.is_ascii_digit() => {
                    let mut temp_number_string = String::new();
                    temp_number_string.push(current_char);
                    loop {
                        match self.peek() {
                            Some(' ') | Some(',') | Some('\n') | Some('\r') => break,
                            Some(_) => {
                                temp_number_string.push(*(self.pop().unwrap()));
                            }
                            None => break,
                        }
                    }
                    match i16::from_str_radix(&temp_number_string, 10) {
                        Ok(val) => {
                            self.push_token(Token::NumericLiteral(val as i16));
                        }
                        Err(e) => {
                            return Err(format!("Failed to parse hexadecimal numeric literal: {e}"));
                        }
                    }
                }
                _ => {
                    let mut temp_string = String::new();
                    temp_string.push(current_char);
                    loop {
                        match self.peek() {
                            Some(' ') | Some(',') | Some('\n') | Some('\r') => break,
                            Some(_) => {
                                temp_string.push(*(self.pop().unwrap()));
                            }
                            None => break,
                        }
                    }
                    let opcodes = vec![
                        ".orig", ".fill", ".blkw", ".stringz", ".end",
                        "add", "and", "not",
                        "br", "jmp", "jsr", "jsrr", 
                        "ld", "ldi", "ldr", "lea",
                        "st", "sti", "str",
                        "getc", "out", "puts", "in", "halt",
                    ];
                    if opcodes.contains(&(temp_string.as_str())) {
                        self.push_token(Token::Op(temp_string));
                    } else {
                        self.push_token(Token::Label(temp_string));
                    }
                }
            }
        } else {
            return Err(format!("End of file"));
        }
        Ok(())
    }

    fn generate_bin_from_asm(&mut self) -> Result<[i16; 65536], String> {
        let mut symbol_table = HashMap::<String, usize>::new();
        let mut update_queue = VecDeque::<(usize, String)>::new();
        let mut current_mem_address: Option<usize> = None;
        let mut return_bytes = [0; 65536];
        let mut token_ptr = 0;

        while token_ptr < self.parsed_tokens.len() {
            let mut increment_mem_address = true;
            if let Some(addr) = current_mem_address {
                // parse when inside a .orig block
                match self.get_token(token_ptr) {
                    Token::Op(op_name) => match op_name.as_str() {
                        ".orig" => {
                            return Err(format!("Invalid .orig: already inside a .orig block"))
                        }
                        ".end" => {
                            increment_mem_address = false;
                            current_mem_address = None;
                        }
                        ".fill" => {
                            token_ptr += 1;
                            if let Token::NumericLiteral(val) = self.get_token(token_ptr) {
                                return_bytes[addr] = *val;
                            } else {
                                return Err(format!("Expected numeric literal after .fill"));
                            }
                        }
                        ".blkw" => {
                            token_ptr += 1;
                            if let Token::NumericLiteral(val) = self.get_token(token_ptr) {
                                current_mem_address = Some(addr + (*val as u16) as usize);
                                increment_mem_address = false;
                            } else {
                                return Err(format!("Expected numeric literal after .blkw"));
                            }
                        }
                        ".stringz" => {
                            token_ptr += 1;
                            let Token::StringLiteral(string_val) = self.get_token(token_ptr) else {
                                return Err(format!("Expected string literal after .stringz"));
                            };
                            for byte in string_val.as_bytes() {
                                return_bytes[current_mem_address.unwrap()] = *byte as i16;
                                current_mem_address = Some(current_mem_address.unwrap() + 1);
                            }
                            return_bytes[current_mem_address.unwrap()] = 0;
                        }
                        "add" => {
                            token_ptr += 1;
                            let Token::Register(dr) = self.get_token(token_ptr) else {
                                return Err(format!("Expected register as first argument of ADD"));
                            };

                            token_ptr += 1;
                            let Token::Comma = self.get_token(token_ptr) else {
                                return Err(format!("Expected comma"));
                            };

                            token_ptr += 1;
                            let Token::Register(sr_1) = self.get_token(token_ptr) else {
                                return Err(format!("Expected register as second argument of ADD"));
                            };

                            token_ptr += 1;
                            let Token::Comma = self.get_token(token_ptr) else {
                                return Err(format!("Expected comma"));
                            };

                            let bit_5: &i16;
                            let sr_2: &i16;

                            token_ptr += 1;
                            match self.get_token(token_ptr) {
                                Token::NumericLiteral(val) => {
                                    if -16 <= *val && *val <= 15 {
                                        bit_5 = &1;
                                        println!("bit 5 is {bit_5}");
                                        sr_2 = val;
                                    } else {
                                        return Err(format!("imm5 operand of ADD is outside allowed bounds (allowed: -16 to 15, found: {val})"))
                                    }
                                }
                                Token::Register(val) => {
                                    bit_5 = &0;
                                    sr_2 = val;
                                }
                                _ => return Err(format!("Expected register or numeric literal as third argument of ADD")),
                            }

                            return_bytes[addr] = (0b0001 << 12) + (dr << 9) + (sr_1 << 6) + (bit_5 << 5) + last_n_bits(*sr_2, 5);
                        }
                        "and" => {
                            token_ptr += 1;
                            let Token::Register(dr) = self.get_token(token_ptr) else {
                                return Err(format!("Expected register as first argument of AND"));
                            };

                            token_ptr += 1;
                            let Token::Comma = self.get_token(token_ptr) else {
                                return Err(format!("Expected comma"));
                            };

                            token_ptr += 1;
                            let Token::Register(sr_1) = self.get_token(token_ptr) else {
                                return Err(format!("Expected register as second argument of AND"));
                            };

                            token_ptr += 1;
                            let Token::Comma = self.get_token(token_ptr) else {
                                return Err(format!("Expected comma"));
                            };

                            let bit_5: &i16;
                            let sr_2: &i16;

                            token_ptr += 1;
                            match self.get_token(token_ptr) {
                                Token::NumericLiteral(val) => {
                                    if -16 <= *val && *val <= 15 {
                                        bit_5 = &1;
                                        sr_2 = val;
                                    } else {
                                        return Err(format!("imm5 operand of AND is outside allowed bounds (allowed: -16 to 15, found: {val})"))
                                    }
                                }
                                Token::Register(val) => {
                                    bit_5 = &0;
                                    sr_2 = val;
                                }
                                _ => return Err(format!("Expected register or numeric literal as third argument of AND")),
                            }

                            return_bytes[addr] = (0b0101 << 12) + (dr << 9) + (sr_1 << 6) + (bit_5 << 5) + last_n_bits(*sr_2, 5);
                        }
                        _ => {}
                    },
                    Token::Label(label_name) => {
                        symbol_table.insert(label_name.clone(), addr);
                        increment_mem_address = false;
                    }
                    Token::Newline => self.increment_line_number(),
                    other => {
                        return Err(format!(
                            "Unexpected token: expected instruction or label, found {other}"
                        ))
                    }
                }
                if increment_mem_address {
                    current_mem_address = Some(current_mem_address.unwrap() + 1);
                }
            } else {
                // parse when outside a .orig block
                match self.get_token(token_ptr) {
                    Token::Op(op_name) => match op_name.as_str() {
                        ".orig" => {
                            token_ptr += 1;
                            if let Token::NumericLiteral(address) = self.get_token(token_ptr) {
                                current_mem_address = Some((*address as u16) as usize);
                                println!(
                                    "current mem addr is set to {}",
                                    current_mem_address.unwrap()
                                );
                            } else {
                                return Err(format!("Expected numeric literal after .orig"));
                            }
                        }
                        ".end" => return Err(format!("Invalid .end: no .orig block found")),
                        op => return Err(format!("Failed to parse {op}: no .orig block found")),
                    },
                    Token::Newline => self.increment_line_number(),
                    other => {
                        return Err(format!("Unexpected token: expected .orig, found {other}"))
                    }
                }
            }
            token_ptr += 1;
        }
        Ok(return_bytes)
    }
}

fn last_n_bits(val: i16, num_bits: usize) -> i16 {
    val - ((val >> num_bits) << num_bits)
}

pub fn test() -> Result<(), String> {
    let mut state = ParserState::new(r#".orig xF000 add r0, r0, -16"#);
    while state.peek() != None {
        state.parse_one_token()?;
    }
    match state.generate_bin_from_asm() {
        Ok(_) => {
            println!("ok!");
        }
        Err(e) => {
            println!("{e}");
        }
    }
    println!("{:?}", state.parsed_tokens);
    Ok(())
}
