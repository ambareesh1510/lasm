use std::collections::HashMap;

use nom::{
    IResult,
    bytes::complete::{tag_no_case, tag},
    sequence::tuple, character::complete::{i16, space0, line_ending, hex_digit1}, 
    combinator::opt, number::complete::hex_u32,
};

fn parse_add_from_reg(input: &str) -> IResult<&str, i16> {
    let (remainder, _) = tag_no_case("add ")(input)?;
    let (remainder, (_, dr, _)) = tuple((
        tag_no_case("r"),
        i16,
        tuple((tag(","), space0)),
    ))(remainder)?;
    let (remainder, (_, sr_1, _)) = tuple((
        tag_no_case("r"),
        i16,
        tuple((tag(","), space0)),
    ))(remainder)?;
    let (remainder, (_, sr_2, _)) = tuple((
        tag_no_case("r"),
        i16,
        tuple((opt(space0), opt(line_ending)))
    ))(remainder)?;
    let return_data = Ok((
        remainder,
        ((0b0001u16 as i16) << 12)
            + (dr << 9)
            + (sr_1 << 6)
            + sr_2,
    ));
    return_data
}

fn parse_add_from_imm5(input: &str) -> IResult<&str, i16> {
    let (remainder, _) = tag_no_case("add ")(input)?;
    let (remainder, (_, dr, _)) = tuple((
        tag_no_case("r"),
        i16,
        tuple((tag(","), space0)),
    ))(remainder)?;
    let (remainder, (_, sr_1, _)) = tuple((
        tag_no_case("r"),
        i16,
        tuple((tag(","), space0)),
    ))(remainder)?;
    let (remainder, (_, sr_2, _)) = tuple((
        tag_no_case("x"),
        hex_digit1,
        tuple((opt(space0), opt(line_ending)))
    ))(remainder)?;
    let sr_2 = i16::from_str_radix(sr_2, 16).unwrap();
    println!("sr2 = {sr_2}");
    let return_data = Ok((
        remainder,
        ((0b0001u16 as i16) << 12)
            + (dr << 9)
            + (sr_1 << 6)
            + sr_2,
    ));
    return_data
}

pub fn test() {
    match parse_asm(".ORIG x3000".to_owned()) {
        Ok(_) => {}
        Err(e) => {
            println!("{e}")
        }
    };
}

fn parse_asm(source: String) -> Result<[i16; 65536], String> {
    let tokens = source.split_whitespace().collect::<Vec<&str>>();
    let symbol_table = HashMap::<&str, usize>::new();
    let mut current_mem_address = None;
    let mut return_bytes = [0; 65536];
    let mut token_ptr = 0;
    println!("{:?}", tokens);
    while token_ptr < tokens.len() {
        match tokens[token_ptr].to_lowercase().as_str() {
            ".orig" => match current_mem_address {
                Some(_) => return Err(format!("Already inside an .orig block")),
                None => {
                    token_ptr += 1;
                    let parse_orig_address_result = if &tokens[token_ptr][0..1] == "x" {
                        u16::from_str_radix(&tokens[token_ptr][1..], 16)
                    } else {
                        u16::from_str_radix(tokens[token_ptr], 10)
                    };
                    match parse_orig_address_result {
                        Ok(val) => current_mem_address = Some(val as usize),
                        Err(e) => return Err(format!("Invalid memory address supplied to .orig (error: {e})")),
                    }
                }
            }
            ".end" => match current_mem_address {
                Some(_) => current_mem_address = None,
                None => return Err(format!("No corresponding .orig directive found")),
            }
            _ => {}
        }
        token_ptr += 1;
    }
    Ok(return_bytes)
}
