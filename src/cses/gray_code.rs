fn gray_code(i: u32, width: usize) -> String {
    format!("{:0>width$b}", i ^ (i >> 1))
}

pub fn get_codes_list(bit_len: u32) -> Vec<String> {
    (0..1 << bit_len)
        .map(|i| gray_code(i, bit_len as usize))
        .collect()
}
