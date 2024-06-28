pub fn to_bb_position(position: u64) -> u64 {
    1 << position
}

pub fn to_decimal_position(position: u64) -> u64 {
    1 >> position
}
