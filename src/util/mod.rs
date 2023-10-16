/// Obtains the bits of `val` corresponding to `[start : end]`, inclusive.
/// Assumes that `start > end`. Bit 0 is the rightmost (least significant) bit.
pub fn bits(val: u16, start: u16, end: u16) -> u16 {
    ((val as u32 & (2u32.pow((start + 1) as u32) - 1u32 - 
        if end == 0 {
            0
        } else {
            2u32.pow((end - 1) as u32)
        }
    )) / 2u32.pow(end as u32)) as u16
}

/// Returns the input value, interpreted as if it were encoded as a two's complement 16-bit integer.
pub fn sext(val: u16) -> i16 {
    if val == 0 {
        0
    } else {
        // 11000
        // log_2(24) = 4
        // 2^(4) = 16
        // 
        val as i16 - 2 * 2i16.pow(val.ilog2())
    }
}

/*
pub fn twoc_to_unsigned(val: u16) -> i16 {
    if val >= 0 {
        val
    } else {
        // 11001
        // twoc = -8
        // unsg = 16
        // 
        val as i16 - 2 * 2i16.pow(val.ilog2())
    }
}
*/
