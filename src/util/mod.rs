/// Obtains the bits of `val` corresponding to `[start : end]`, inclusive.
/// Assumes that `start > end`. Bit 0 is the rightmost (least significant) bit.
pub fn bits(val: i16, start: u16, end: u16) -> u16 {
    (((val as i32 + 2i32.pow(16)) as u32 & (2u32.pow((start + 1) as u32) - 1u32 - 
        if end == 0 {
            0
        } else {
            2u32.pow((end - 1) as u32)
        }
    )) / 2u32.pow(end as u32)) as u16
}

/// Returns the input value, interpreted as if it were encoded as a two's complement 16-bit integer.
pub fn sext(val: u16, len: u16) -> i16 {
    if val == 0 {
        0
    } else if val < 2u16.pow((len - 1) as u32) as u16 {
        return val as i16
    } else {
        // 0000 0000 0001 0000
        // 1111 1111 1111 0000
        (2i32.pow(16) -
            2i32.pow(val.ilog2() + 1) +
            val as i32) as i16
    }
}


pub fn unsext(val: i16) -> u16 {
    if val >= 0 {
        val as u16
    } else {
        // 11001
        // twoc = -7
        // unsg = 25
        // 
        // (val as i16 + 2 * 2i16.pow((-1 * val as i16).ilog2())) as u16
        (2i32.pow(16) + val as i32) as u16
    }
}
