use std::{collections::{HashMap, VecDeque}, num::ParseIntError};

pub fn test() {
    match parse_asm(
        r#"
                    .ORIG x3000
                    .stringz "ab"
                    .end
                    "#
        .to_owned(),
        // 0000 1001
    ) {
        Ok(val) => {
            println!("x3000 {:0>4X}", val[0x3000]);
            println!("x3001 {:0>4X}", val[0x3001]);
            println!("x3002 {:0>4X}", val[0x3002]);
        }
        Err(e) => {
            println!("{e}")
        }
    };
}

fn is_keyword(s: &str) -> bool {
    match s {
        ".orig" | ".fill" | ".blkw" | ".stringz" | ".end" | ".extern" | "add" | "and" | "not"
        | "br" | "brn" | "brz" | "brp" | "brnz" | "brzp" | "brnp" | "brnzp" | "jmp" | "jsr"
        | "jsrr" | "ld" | "ldr" | "ldi" | "lea" | "st" | "sti" | "str" | "getc" | "out"
        | "puts" | "in" | "halt" => true,
        _ => false,
    }
}

fn parse_unsigned_hex_or_decimal_literal(s: &str) -> Result<usize, ParseIntError> {
    let parse_address_result = if &s[0..1] == "x" {
        usize::from_str_radix(&s[1..], 16)
    } else {
        usize::from_str_radix(s, 10)
    };
    parse_address_result
}

fn parse_signed_hex_or_decimal_literal(s: &str) -> Result<i32, ParseIntError> {
    let parse_address_result = if &s[0..1] == "x" {
        i32::from_str_radix(&s[1..], 16)
    } else {
        i32::from_str_radix(s, 10)
    };
    parse_address_result
}

fn parse_register_without_comma(s: &str) -> Result<u16, String> {
    if &s[0..1] != "r" {
        return Err(format!("Expected a register"));
    }
    match u16::from_str_radix(&s[1..s.len()], 10) {
        Ok(val) => {
            if val > 8 {
                return Err(format!("Invalid register specified"));
            } else {
                return Ok(val);
            }
        }
        Err(e) => return Err(format!("Invalid register specified (error: {e})")),
    }
}

fn parse_register_with_comma(s: &str) -> Result<u16, String> {
    if &s[0..1] != "r" {
        println!("s[0] = {}", &s[0..1]);
        return Err(format!("Expected a register"));
    } else if &s[s.len() - 1..] != "," {
        return Err(format!("Expected more arguments"));
    }
    match u16::from_str_radix(&s[1..s.len() - 1], 10) {
        Ok(val) => {
            if val > 8 {
                return Err(format!("Invalid register specified"));
            } else {
                return Ok(val);
            }
        }
        Err(e) => return Err(format!("Invalid register specified (error: {e})")),
    }
}

fn parse_asm(source: String) -> Result<[i16; 65536], String> {
    let mut tokens = source
        .split_whitespace()
        .map(|e| e.to_lowercase())
        .collect::<Vec<String>>();
    let mut symbol_table = HashMap::<&str, usize>::new();
    let mut update_queue = VecDeque::<(usize, &str)>::new();
    let mut current_mem_address = None;
    let mut return_bytes = [0; 65536];
    let mut token_ptr = 0;
    println!("{:?}", tokens);
    while token_ptr < tokens.len() {
        match tokens[token_ptr].as_str() {
            ".orig" => match current_mem_address {
                Some(_) => return Err(format!("Already inside an .orig block")),
                None => {
                    token_ptr += 1;
                    let parse_orig_address_result =
                        parse_unsigned_hex_or_decimal_literal(tokens[token_ptr].as_str());
                    match parse_orig_address_result {
                        Ok(val) => current_mem_address = Some(val as usize),
                        Err(e) => {
                            return Err(format!(
                                "Invalid memory address supplied to .orig (error: {e})"
                            ))
                        }
                    }
                }
            },
            ".end" => match current_mem_address {
                Some(_) => current_mem_address = None,
                None => return Err(format!("No corresponding .orig directive found")),
            },
            t => match current_mem_address {
                None => return Err(format!("No initial memory address set with .orig")),
                Some(_) => {
                    let mut increment_mem_addr = true;
                    match t {
                        ".fill" => {
                            token_ptr += 1;
                            let parse_fill_value_result =
                                parse_signed_hex_or_decimal_literal(tokens[token_ptr].as_str());
                            match parse_fill_value_result {
                                Ok(val) => return_bytes[current_mem_address.unwrap()] = val as i16,
                                Err(e) => {
                                    return Err(format!(
                                        "Invalid data supplied to .fill (error: {e})"
                                    ))
                                }
                            }
                        }
                        ".blkw" => {
                            token_ptr += 1;
                            let parse_blkw_value_result =
                                parse_unsigned_hex_or_decimal_literal(tokens[token_ptr].as_str());
                            match parse_blkw_value_result {
                                Ok(val) => {
                                    current_mem_address = Some(current_mem_address.unwrap() + val);
                                    increment_mem_addr = false;
                                }
                                Err(e) => {
                                    return Err(format!(
                                        "Invalid offset supplied to .blkw (error: {e})"
                                    ))
                                }
                            }
                        }
                        ".stringz" => {
                            token_ptr += 1;
                            let ascii_string = tokens[token_ptr].as_str();
                            if &ascii_string[0..1] != "\""
                                || &ascii_string[ascii_string.len() - 1..] != "\""
                            {
                                return Err(format!(
                                    "Invalid data supplied to .stringz: not a string"
                                ));
                            }
                            let ascii_bytes = ascii_string[1..ascii_string.len() - 1].as_bytes();
                            for byte in ascii_bytes {
                                return_bytes[current_mem_address.unwrap()] = *byte as i16;
                                current_mem_address = Some(current_mem_address.unwrap() + 1);
                            }
                            return_bytes[current_mem_address.unwrap()] = 0i16;
                            // current_mem_address = Some(current_mem_address.unwrap() + 1);
                        }
                        "add" => {
                            token_ptr += 1;
                            let dr = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading destination register in ADD: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let sr_1 = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading source register 1 in ADD: {e}"
                                    ))
                                }
                            };

                            let bit_5;

                            token_ptr += 1;
                            let sr_2 = match parse_register_without_comma(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    bit_5 = 0;
                                    val
                                }
                                Err(_) => {
                                    match parse_signed_hex_or_decimal_literal(
                                        tokens[token_ptr].as_str(),
                                    ) {
                                        Ok(val) => {
                                            if -16 < val && val < 15 {
                                                bit_5 = 1;
                                                val as u16
                                            } else {
                                                return Err(format!("Error while reading imm5 operand in ADD: imm5 operand exceeds bounds [-16, 15]"));
                                            }
                                        }
                                        Err(_) => {
                                            return Err(format!(
                                                "Error while reading second operand in ADD"
                                            ))
                                        }
                                    }
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b0001 << 12) + (dr << 9) + (sr_1 << 6) + (bit_5 << 5) + match bit_5 {
                                    0 => sr_2,
                                    1 => sr_2 - ((sr_2 >> 5) << 5),
                                    _ => unreachable!()
                                })
                                    as i16;
                        }
                        "and" => {
                            token_ptr += 1;
                            let dr = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading destination register in AND: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let sr_1 = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading source register 1 in AND: {e}"
                                    ))
                                }
                            };

                            let bit_5;

                            token_ptr += 1;
                            let sr_2 = match parse_register_without_comma(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    bit_5 = 0;
                                    val
                                }
                                Err(_) => {
                                    match parse_signed_hex_or_decimal_literal(
                                        tokens[token_ptr].as_str(),
                                    ) {
                                        Ok(val) => {
                                            if -16 <= val && val <= 15 {
                                                bit_5 = 1;
                                                val as u16
                                            } else {
                                                return Err(format!("Error while reading imm5 operand in AND: imm5 operand exceeds bounds [-16, 15]"));
                                            }
                                        }
                                        Err(_) => {
                                            return Err(format!(
                                                "Error while reading second operand in AND"
                                            ))
                                        }
                                    }
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b0101 << 12) + (dr << 9) + (sr_1 << 6) + (bit_5 << 5) + match bit_5 {
                                    0 => sr_2,
                                    1 => sr_2 - ((sr_2 >> 5) << 5),
                                    _ => unreachable!()
                                })
                                    as i16;
                        }
                        "not" => {
                            token_ptr += 1;
                            let dr = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading destination register in NOT: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let sr = match parse_register_without_comma(tokens[token_ptr].as_str())
                            {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading source register in NOT: {e}"
                                    ))
                                }
                            };

                            return_bytes[current_mem_address.unwrap()] =
                                ((0b1001 << 12) + (dr << 9) + (sr << 6) + 0b111111) as i16;
                        }
                        "br" | "brn" | "brz" | "brp" | "brnz" | "brnp" | "brzp" | "brnzp" => {
                            let nzp = match tokens[token_ptr].as_str() {
                                "br" | "brnzp" => 0b111,
                                "brn" => 0b100,
                                "brz" => 0b010,
                                "brp" => 0b001,
                                "brnz" => 0b110,
                                "brnp" => 0b101,
                                "brzp" => 0b011,
                                _ => unreachable!(),
                            };

                            token_ptr += 1;
                            let offset = match parse_signed_hex_or_decimal_literal(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    if -256 <= val && val <= 255 {
                                        val
                                    } else {
                                        return Err(format!("Error while reading PCOffset in BR: PCOffset9 exceeds bounds [-256, 255]"));
                                    }
                                }
                                Err(_) => {
                                    if is_keyword(tokens[token_ptr].as_str()) {
                                        return Err(format!("Error while reading label: found keyword {}", tokens[token_ptr].as_str()));
                                    } else {
                                        update_queue.push_back((current_mem_address.unwrap(), tokens[token_ptr].as_str()));
                                    }
                                    0
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b0000 << 12) + (nzp << 9) + offset - ((offset >> 9) << 9)) as i16;
                        }
                        "jmp" => {
                            token_ptr += 1;
                            let base_r = match parse_register_without_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => return Err(format!("Error matching register in JMP: {e}")),
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b1100 << 12) + (base_r << 6)) as i16;
                        }
                        "jsr" => {
                            token_ptr += 1;
                            let offset = match parse_signed_hex_or_decimal_literal(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    if -1024 <= val && val <= 1023 {
                                        val
                                    } else {
                                        return Err(format!("Error while reading PCOffset in JSR: PCOffset11 exceeds bounds [-1024, 1023]"));
                                    }
                                }
                                Err(_) => {
                                    if is_keyword(tokens[token_ptr].as_str()) {
                                        return Err(format!("Error while reading label: found keyword {}", tokens[token_ptr].as_str()));
                                    } else {
                                        update_queue.push_back((current_mem_address.unwrap(), tokens[token_ptr].as_str()));
                                    }
                                    0
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b0100 << 12) + (0b1 << 11) + offset - ((offset >> 11) << 11)) as i16;
                        }
                        "jsrr" => {
                            token_ptr += 1;
                            let base_r = match parse_register_without_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => return Err(format!("Error matching register in JSRR: {e}")),
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b0100 << 12) + (base_r << 6)) as i16;
                        }
                        "ld" => {
                            token_ptr += 1;
                            let dr = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading destination register in LD: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let offset = match parse_signed_hex_or_decimal_literal(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    if -256 <= val && val <= 255 {
                                        val as u16
                                    } else {
                                        return Err(format!("Error while reading PCOffset in LD: PCOffset9 exceeds bounds [-256, 255]"));
                                    }
                                }
                                Err(_) => {
                                    if is_keyword(tokens[token_ptr].as_str()) {
                                        return Err(format!("Error while reading label: found keyword {}", tokens[token_ptr].as_str()));
                                    } else {
                                        update_queue.push_back((current_mem_address.unwrap(), tokens[token_ptr].as_str()));
                                    }
                                    0
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b0010 << 12) + (dr << 9) + (offset - ((offset >> 9) << 9))) as i16;
                        }
                        "ldi" => {
                            token_ptr += 1;
                            let dr = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading destination register in LDI: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let offset = match parse_signed_hex_or_decimal_literal(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    if -256 <= val && val <= 255 {
                                        val as u16
                                    } else {
                                        return Err(format!("Error while reading PCOffset in LDI: PCOffset9 exceeds bounds [-256, 255]"));
                                    }
                                }
                                Err(_) => {
                                    if is_keyword(tokens[token_ptr].as_str()) {
                                        return Err(format!("Error while reading label: found keyword {}", tokens[token_ptr].as_str()));
                                    } else {
                                        update_queue.push_back((current_mem_address.unwrap(), tokens[token_ptr].as_str()));
                                    }
                                    0
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b1010 << 12) + (dr << 9) + (offset - ((offset >> 9) << 9))) as i16;
                        }
                        "ldr" => {
                            token_ptr += 1;
                            let dr = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading destination register in LDI: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let base_r = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading base register in LDR: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let offset = match parse_signed_hex_or_decimal_literal(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    if -32 <= val && val <= 31 {
                                        val as u16
                                    } else {
                                        return Err(format!("Error while reading PCOffset in LDR: offset6 exceeds bounds [-32, 31]"));
                                    }
                                }
                                Err(_) => {
                                    if is_keyword(tokens[token_ptr].as_str()) {
                                        return Err(format!("Error while reading label: found keyword {}", tokens[token_ptr].as_str()));
                                    } else {
                                        update_queue.push_back((current_mem_address.unwrap(), tokens[token_ptr].as_str()));
                                    }
                                    0
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b0110 << 12) + (dr << 9) + (base_r << 6) + (offset - ((offset >> 9) << 9))) as i16;
                        }
                        "lea" => {
                            token_ptr += 1;
                            let dr = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading destination register in LEA: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let offset = match parse_signed_hex_or_decimal_literal(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    if -256 <= val && val <= 255 {
                                        val as u16
                                    } else {
                                        return Err(format!("Error while reading PCOffset in LEA: PCOffset9 exceeds bounds [-256, 255]"));
                                    }
                                }
                                Err(_) => {
                                    if is_keyword(tokens[token_ptr].as_str()) {
                                        return Err(format!("Error while reading label: found keyword {}", tokens[token_ptr].as_str()));
                                    } else {
                                        update_queue.push_back((current_mem_address.unwrap(), tokens[token_ptr].as_str()));
                                    }
                                    0
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b1110 << 12) + (dr << 9) + (offset - ((offset >> 9) << 9))) as i16;
                        }
                        "st" => {
                            token_ptr += 1;
                            let sr = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading source register in ST: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let offset = match parse_signed_hex_or_decimal_literal(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    if -256 <= val && val <= 255 {
                                        val as u16
                                    } else {
                                        return Err(format!("Error while reading PCOffset in ST: PCOffset9 exceeds bounds [-256, 255]"));
                                    }
                                }
                                Err(_) => {
                                    if is_keyword(tokens[token_ptr].as_str()) {
                                        return Err(format!("Error while reading label: found keyword {}", tokens[token_ptr].as_str()));
                                    } else {
                                        update_queue.push_back((current_mem_address.unwrap(), tokens[token_ptr].as_str()));
                                    }
                                    0
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b0011 << 12) + (sr << 9) + (offset - ((offset >> 9) << 9))) as i16;
                        }
                        "sti" => {
                            token_ptr += 1;
                            let sr = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading source register in STI: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let offset = match parse_signed_hex_or_decimal_literal(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    if -256 <= val && val <= 255 {
                                        val as u16
                                    } else {
                                        return Err(format!("Error while reading PCOffset in STI: PCOffset9 exceeds bounds [-256, 255]"));
                                    }
                                }
                                Err(_) => {
                                    if is_keyword(tokens[token_ptr].as_str()) {
                                        return Err(format!("Error while reading label: found keyword {}", tokens[token_ptr].as_str()));
                                    } else {
                                        update_queue.push_back((current_mem_address.unwrap(), tokens[token_ptr].as_str()));
                                    }
                                    0
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b1011 << 12) + (sr << 9) + (offset - ((offset >> 9) << 9))) as i16;
                        }
                        "str" => {
                            token_ptr += 1;
                            let sr = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading destination register in STR: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let base_r = match parse_register_with_comma(tokens[token_ptr].as_str()) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(format!(
                                        "Error while reading base register in STR: {e}"
                                    ))
                                }
                            };

                            token_ptr += 1;
                            let offset = match parse_signed_hex_or_decimal_literal(
                                tokens[token_ptr].as_str(),
                            ) {
                                Ok(val) => {
                                    if -32 <= val && val <= 31 {
                                        val as u16
                                    } else {
                                        return Err(format!("Error while reading PCOffset in STR: offset6 exceeds bounds [-32, 31]"));
                                    }
                                }
                                Err(_) => {
                                    if is_keyword(tokens[token_ptr].as_str()) {
                                        return Err(format!("Error while reading label: found keyword {}", tokens[token_ptr].as_str()));
                                    } else {
                                        update_queue.push_back((current_mem_address.unwrap(), tokens[token_ptr].as_str()));
                                    }
                                    0
                                }
                            };
                            return_bytes[current_mem_address.unwrap()] =
                                ((0b0111 << 12) + (sr << 9) + (base_r << 6) + (offset - ((offset >> 9) << 9))) as i16;
                        }
                        "getc" => {
                            return_bytes[current_mem_address.unwrap()] = 0b1111_0000_0010_0000u16 as i16;
                        }
                        "out" => {
                            return_bytes[current_mem_address.unwrap()] = 0b1111_0000_0010_0001u16 as i16;
                        }
                        "puts" => {
                            return_bytes[current_mem_address.unwrap()] = 0b1111_0000_0010_0010u16 as i16;
                        }
                        "in" => {
                            return_bytes[current_mem_address.unwrap()] = 0b1111_0000_0010_0011u16 as i16;
                        }
                        "halt" => {
                            return_bytes[current_mem_address.unwrap()] = 0b1111_0000_0010_0101u16 as i16;
                        }
                        label => {
                            symbol_table.insert(label, current_mem_address.unwrap());
                            increment_mem_addr = false;
                        }
                    };
                    if increment_mem_addr {
                        current_mem_address = Some(current_mem_address.unwrap() + 1);
                    }
                }
            },
        }
        token_ptr += 1;
    }
    println!("symbol table {:?}", symbol_table);
    println!("update queue {:?}", update_queue);
    for (addr, label) in update_queue {
        if let Some(label_addr) = symbol_table.get(label) {
            let offset = (*label_addr as isize) - (addr as isize) - 1;
            return_bytes[addr] += offset as i16;
        }
    }
    Ok(return_bytes)
}
