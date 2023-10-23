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

pub fn test() -> IResult<&'static str, i16> {
    println!("parse add from register {:0>16b}", parse_add_from_reg("add r1, r2, r3\n")?.1);
    println!("parse add from imm5 {:0>16b}", parse_add_from_imm5("add r1, r2, xF\n")?.1);
    parse_add_from_reg("add r1, r2, r3")
}
