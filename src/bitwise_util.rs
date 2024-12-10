pub fn u32_to_u8_arr(val: u32) -> [u8; 4] {
    [
        (val >> 24) as u8,
        ((val >> 16) & 0xff) as u8,
        ((val >> 8) & 0xff) as u8,
        (val & 0xff) as u8,
    ]
}

pub fn u32_to_u8_arr_experimental(val: u32) -> [u8; 4] {
    let mut res = [0_u8;4];
    let range = ((u32::BITS / 8)) as usize;
    for bit_incr in (0..range) {
        println!("{}, {}", (bit_incr * 8) + 8, bit_incr);
        res[bit_incr] = ((val >> bit_incr * 8) & 0xff) as u8;
    };
    res.reverse();
    res
}

pub fn u16_to_u8_arr(val: u16) -> [u8; 2] {
    [((val >> 8) & 0xff) as u8, (val & 0xff) as u8]
}

pub fn u16_to_u8_arr_experimental(val: u16) -> [u8; 2] {
    let mut res = [0_u8;2];
    let range = ((u32::BITS / 8)) as usize;
    for bit_incr in (0..range) {
        println!("{}, {}", (bit_incr * 8) + 8, bit_incr);
        res[bit_incr] = ((val >> bit_incr * 8) & 0xff) as u8;
    };
    res.reverse();
    res
}