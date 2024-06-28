pub enum BoardConsts {
    a_file             = 0x00000000000000FF,
    h_file             = 0xFF00000000000000,
    first_rank         = 0x0101010101010101,
    eighth_rank        = 0x8080808080808080,
    a1_h8_diagonal     = 0x8040201008040201,
    h1_a8_antidiagonal = 0x0102040810204080,
    light_squares      = 0x55AA55AA55AA55AA,
    dark_squares       = 0xAA55AA55AA55AA55
}