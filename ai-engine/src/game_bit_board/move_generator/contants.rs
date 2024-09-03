/// Do not rely visually on these constants. Remember that the first position (0)
/// is actually on the bottom left corner of the board, and the last (63) is
/// on the top right corner of the board.

pub const BISHOP_MOVES: [u64; 64] = [
  0x8040201008040200, 0x0080402010080500, 0x0000804020110A00, 0x0000008041221400, 0x0000000182442800, 0x0000010204885000, 0x000102040810A000, 0x0102040810204000,
  0x4020100804020002, 0x8040201008050005, 0x00804020110A000A, 0x0000804122140014, 0x0000018244280028, 0x0001020488500050, 0x0102040810A000A0, 0x0204081020400040,
  0x2010080402000204, 0x4020100805000508, 0x804020110A000A11, 0x0080412214001422, 0x0001824428002844, 0x0102048850005088, 0x02040810A000A010, 0x0408102040004020,
  0x1008040200020408, 0x2010080500050810, 0x4020110A000A1120, 0x8041221400142241, 0x0182442800284482, 0x0204885000508804, 0x040810A000A01008, 0x0810204000402010,
  0x0804020002040810, 0x1008050005081020, 0x20110A000A112040, 0x4122140014224180, 0x8244280028448201, 0x0488500050880402, 0x0810A000A0100804, 0x1020400040201008,
  0x0402000204081020, 0x0805000508102040, 0x110A000A11204080, 0x2214001422418000, 0x4428002844820100, 0x8850005088040201, 0x10A000A010080402, 0x2040004020100804,
  0x0200020408102040, 0x0500050810204080, 0x0A000A1120408000, 0x1400142241800000, 0x2800284482010000, 0x5000508804020100, 0xA000A01008040201, 0x4000402010080402,
  0x0002040810204080, 0x0005081020408000, 0x000A112040800000, 0x0014224180000000, 0x0028448201000000, 0x0050880402010000, 0x00A0100804020100, 0x0040201008040201,
];

pub const BLACK_PAWN_ATTACKS: [u64; 64] = [
  0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
  0x0000000000000002, 0x0000000000000005, 0x000000000000000A, 0x0000000000000014, 0x0000000000000028, 0x0000000000000050, 0x00000000000000A0, 0x0000000000000040,
  0x0000000000000200, 0x0000000000000500, 0x0000000000000A00, 0x0000000000001400, 0x0000000000002800, 0x0000000000005000, 0x000000000000A000, 0x0000000000004000,
  0x0000000000020000, 0x0000000000050000, 0x00000000000A0000, 0x0000000000140000, 0x0000000000280000, 0x0000000000500000, 0x0000000000A00000, 0x0000000000400000,
  0x0000000002000000, 0x0000000005000000, 0x000000000A000000, 0x0000000014000000, 0x0000000028000000, 0x0000000050000000, 0x00000000A0000000, 0x0000000040000000,
  0x0000000200000000, 0x0000000500000000, 0x0000000A00000000, 0x0000001400000000, 0x0000002800000000, 0x0000005000000000, 0x000000A000000000, 0x0000004000000000,
  0x0000020000000000, 0x0000050000000000, 0x00000A0000000000, 0x0000140000000000, 0x0000280000000000, 0x0000500000000000, 0x0000A00000000000, 0x0000400000000000,
  0x0002000000000000, 0x0005000000000000, 0x000A000000000000, 0x0014000000000000, 0x0028000000000000, 0x0050000000000000, 0x00A0000000000000, 0x0040000000000000,
];

// Push moves, excluding attacks
pub const BLACK_PAWN_MOVES: [u64; 64] = [
  0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
  0x0000000000000001, 0x0000000000000002, 0x0000000000000004, 0x0000000000000008, 0x0000000000000010, 0x0000000000000020, 0x0000000000000040, 0x0000000000000080,
  0x0000000000000100, 0x0000000000000200, 0x0000000000000400, 0x0000000000000800, 0x0000000000001000, 0x0000000000002000, 0x0000000000004000, 0x0000000000008000,
  0x0000000000010000, 0x0000000000020000, 0x0000000000040000, 0x0000000000080000, 0x0000000000100000, 0x0000000000200000, 0x0000000000400000, 0x0000000000800000,
  0x0000000001000000, 0x0000000002000000, 0x0000000004000000, 0x0000000008000000, 0x0000000010000000, 0x0000000020000000, 0x0000000040000000, 0x0000000080000000,
  0x0000000100000000, 0x0000000200000000, 0x0000000400000000, 0x0000000800000000, 0x0000001000000000, 0x0000002000000000, 0x0000004000000000, 0x0000008000000000,
  0x0000010100000000, 0x0000020200000000, 0x0000040400000000, 0x0000080800000000, 0x0000101000000000, 0x0000202000000000, 0x0000404000000000, 0x0000808000000000,
  0x0001000000000000, 0x0002000000000000, 0x0004000000000000, 0x0008000000000000, 0x0010000000000000, 0x0020000000000000, 0x0040000000000000, 0x0080000000000000,
];

pub const QUEEN_MOVES: [u64; 64] = [
  0x81412111090503FE, 0x02824222120A07FD, 0x0404844424150EFB, 0x08080888492A1CF7, 0x10101011925438EF, 0x2020212224A870DF, 0x404142444850E0BF, 0x8182848890A0C07F,
  0x412111090503FE03, 0x824222120A07FD07, 0x04844424150EFB0E, 0x080888492A1CF71C, 0x101011925438EF38, 0x20212224A870DF70, 0x4142444850E0BFE0, 0x82848890A0C07FC0,
  0x2111090503FE0305, 0x4222120A07FD070A, 0x844424150EFB0E15, 0x0888492A1CF71C2A, 0x1011925438EF3854, 0x212224A870DF70A8, 0x42444850E0BFE050, 0x848890A0C07FC0A0,
  0x11090503FE030509, 0x22120A07FD070A12, 0x4424150EFB0E1524, 0x88492A1CF71C2A49, 0x11925438EF385492, 0x2224A870DF70A824, 0x444850E0BFE05048, 0x8890A0C07FC0A090,
  0x090503FE03050911, 0x120A07FD070A1222, 0x24150EFB0E152444, 0x492A1CF71C2A4988, 0x925438EF38549211, 0x24A870DF70A82422, 0x4850E0BFE0504844, 0x90A0C07FC0A09088,
  0x0503FE0305091121, 0x0A07FD070A122242, 0x150EFB0E15244484, 0x2A1CF71C2A498808, 0x5438EF3854921110, 0xA870DF70A8242221, 0x50E0BFE050484442, 0xA0C07FC0A0908884,
  0x03FE030509112141, 0x07FD070A12224282, 0x0EFB0E1524448404, 0x1CF71C2A49880808, 0x38EF385492111010, 0x70DF70A824222120, 0xE0BFE05048444241, 0xC07FC0A090888482,
  0xFE03050911214181, 0xFD070A1222428202, 0xFB0E152444840404, 0xF71C2A4988080808, 0xEF38549211101010, 0xDF70A82422212020, 0xBFE0504844424140, 0x7FC0A09088848281,
];

pub const ROOK_MOVES: [u64; 64] = [
  0x01010101010101FE, 0x02020202020202FD, 0x04040404040404FB, 0x08080808080808F7, 0x10101010101010EF, 0x20202020202020DF, 0x40404040404040BF, 0x808080808080807F,
  0x010101010101FE01, 0x020202020202FD02, 0x040404040404FB04, 0x080808080808F708, 0x101010101010EF10, 0x202020202020DF20, 0x404040404040BF40, 0x8080808080807F80,
  0x0101010101FE0101, 0x0202020202FD0202, 0x0404040404FB0404, 0x0808080808F70808, 0x1010101010EF1010, 0x2020202020DF2020, 0x4040404040BF4040, 0x80808080807F8080,
  0x01010101FE010101, 0x02020202FD020202, 0x04040404FB040404, 0x08080808F7080808, 0x10101010EF101010, 0x20202020DF202020, 0x40404040BF404040, 0x808080807F808080,
  0x010101FE01010101, 0x020202FD02020202, 0x040404FB04040404, 0x080808F708080808, 0x101010EF10101010, 0x202020DF20202020, 0x404040BF40404040, 0x8080807F80808080,
  0x0101FE0101010101, 0x0202FD0202020202, 0x0404FB0404040404, 0x0808F70808080808, 0x1010EF1010101010, 0x2020DF2020202020, 0x4040BF4040404040, 0x80807F8080808080,
  0x01FE010101010101, 0x02FD020202020202, 0x04FB040404040404, 0x08F7080808080808, 0x10EF101010101010, 0x20DF202020202020, 0x40BF404040404040, 0x807F808080808080,
  0xFE01010101010101, 0xFD02020202020202, 0xFB04040404040404, 0xF708080808080808, 0xEF10101010101010, 0xDF20202020202020, 0xBF40404040404040, 0x7F80808080808080,
];

pub const WHITE_PAWN_ATTACKS: [u64; 64] = [
  0x0000000000000200, 0x0000000000000500, 0x0000000000000A00, 0x0000000000001400, 0x0000000000002800, 0x0000000000005000, 0x000000000000A000, 0x0000000000004000,
  0x0000000000020000, 0x0000000000050000, 0x00000000000A0000, 0x0000000000140000, 0x0000000000280000, 0x0000000000500000, 0x0000000000A00000, 0x0000000000400000,
  0x0000000002000000, 0x0000000005000000, 0x000000000A000000, 0x0000000014000000, 0x0000000028000000, 0x0000000050000000, 0x00000000A0000000, 0x0000000040000000,
  0x0000000200000000, 0x0000000500000000, 0x0000000A00000000, 0x0000001400000000, 0x0000002800000000, 0x0000005000000000, 0x000000A000000000, 0x0000004000000000,
  0x0000020000000000, 0x0000050000000000, 0x00000A0000000000, 0x0000140000000000, 0x0000280000000000, 0x0000500000000000, 0x0000A00000000000, 0x0000400000000000,
  0x0002000000000000, 0x0005000000000000, 0x000A000000000000, 0x0014000000000000, 0x0028000000000000, 0x0050000000000000, 0x00A0000000000000, 0x0040000000000000,
  0x0200000000000000, 0x0500000000000000, 0x0A00000000000000, 0x1400000000000000, 0x2800000000000000, 0x5000000000000000, 0xA000000000000000, 0x4000000000000000,
  0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
];

// Push moves, excluding attacks
pub const WHITE_PAWN_MOVES: [u64; 64] = [
  0x0000000000000100, 0x0000000000000200, 0x0000000000000400, 0x0000000000000800, 0x0000000000001000, 0x0000000000002000, 0x0000000000004000, 0x0000000000008000,
  0x0000000001010000, 0x0000000002020000, 0x0000000004040000, 0x0000000008080000, 0x0000000010100000, 0x0000000020200000, 0x0000000040400000, 0x0000000080800000,
  0x0000000001000000, 0x0000000002000000, 0x0000000004000000, 0x0000000008000000, 0x0000000010000000, 0x0000000020000000, 0x0000000040000000, 0x0000000080000000,
  0x0000000100000000, 0x0000000200000000, 0x0000000400000000, 0x0000000800000000, 0x0000001000000000, 0x0000002000000000, 0x0000004000000000, 0x0000008000000000,
  0x0000010000000000, 0x0000020000000000, 0x0000040000000000, 0x0000080000000000, 0x0000100000000000, 0x0000200000000000, 0x0000400000000000, 0x0000800000000000,
  0x0001000000000000, 0x0002000000000000, 0x0004000000000000, 0x0008000000000000, 0x0010000000000000, 0x0020000000000000, 0x0040000000000000, 0x0080000000000000,
  0x0100000000000000, 0x0200000000000000, 0x0400000000000000, 0x0800000000000000, 0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000,
  0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
];

pub const KNIGHT_MOVES: [u64; 64] = [
  0x0000000000020400, 0x0000000000050800, 0x00000000000A1100, 0x0000000000142200, 0x0000000000284400, 0x0000000000508800, 0x0000000000A01000, 0x0000000000402000,
  0x0000000002040004, 0x0000000005080008, 0x000000000A110011, 0x0000000014220022, 0x0000000028440044, 0x0000000050880088, 0x00000000A0100010, 0x0000000040200020,
  0x0000000204000402, 0x0000000508000805, 0x0000000A1100110A, 0x0000001422002214, 0x0000002844004428, 0x0000005088008850, 0x000000A0100010A0, 0x0000004020002040,
  0x0000020400040200, 0x0000050800080500, 0x00000A1100110A00, 0x0000142200221400, 0x0000284400442800, 0x0000508800885000, 0x0000A0100010A000, 0x0000402000204000,
  0x0002040004020000, 0x0005080008050000, 0x000A1100110A0000, 0x0014220022140000, 0x0028440044280000, 0x0050880088500000, 0x00A0100010A00000, 0x0040200020400000,
  0x0204000402000000, 0x0508000805000000, 0x0A1100110A000000, 0x1422002214000000, 0x2844004428000000, 0x5088008850000000, 0xA0100010A0000000, 0x4020002040000000,
  0x0400040200000000, 0x0800080500000000, 0x1100110A00000000, 0x2200221400000000, 0x4400442800000000, 0x8800885000000000, 0x100010A000000000, 0x2000204000000000,
  0x0004020000000000, 0x0008050000000000, 0x00110A0000000000, 0x0022140000000000, 0x0044280000000000, 0x0088500000000000, 0x0010A00000000000, 0x0020400000000000,
];

pub const KING_MOVES: [u64; 64] = [
  0x0000000000000302, 0x0000000000000705, 0x0000000000000E0A, 0x0000000000001C14, 0x0000000000003828, 0x0000000000007050, 0x000000000000E0A0, 0x000000000000C040,
  0x0000000000030203, 0x0000000000070507, 0x00000000000E0A0E, 0x00000000001C141C, 0x0000000000382838, 0x0000000000705070, 0x0000000000E0A0E0, 0x0000000000C040C0,
  0x0000000003020300, 0x0000000007050700, 0x000000000E0A0E00, 0x000000001C141C00, 0x0000000038283800, 0x0000000070507000, 0x00000000E0A0E000, 0x00000000C040C000,
  0x0000000302030000, 0x0000000705070000, 0x0000000E0A0E0000, 0x0000001C141C0000, 0x0000003828380000, 0x0000007050700000, 0x000000E0A0E00000, 0x000000C040C00000,
  0x0000030203000000, 0x0000070507000000, 0x00000E0A0E000000, 0x00001C141C000000, 0x0000382838000000, 0x0000705070000000, 0x0000E0A0E0000000, 0x0000C040C0000000,
  0x0003020300000000, 0x0007050700000000, 0x000E0A0E00000000, 0x001C141C00000000, 0x0038283800000000, 0x0070507000000000, 0x00E0A0E000000000, 0x00C040C000000000,
  0x0302030000000000, 0x0705070000000000, 0x0E0A0E0000000000, 0x1C141C0000000000, 0x3828380000000000, 0x7050700000000000, 0xE0A0E00000000000, 0xC040C00000000000,
  0x0203000000000000, 0x0507000000000000, 0x0A0E000000000000, 0x141C000000000000, 0x2838000000000000, 0x5070000000000000, 0xA0E0000000000000, 0x40C0000000000000,
];

pub const BISHOP_RELEVANT_SQUARES: [u64; 64] = [
  0x0040201008040200, 0x0000402010080400, 0x0000004020100A00, 0x0000000040221400, 0x0000000002442800, 0x0000000204085000, 0x0000020408102000, 0x0002040810204000,
  0x0020100804020000, 0x0040201008040000, 0x00004020100A0000, 0x0000004022140000, 0x0000000244280000, 0x0000020408500000, 0x0002040810200000, 0x0004081020400000,
  0x0010080402000200, 0x0020100804000400, 0x004020100A000A00, 0x0000402214001400, 0x0000024428002800, 0x0002040850005000, 0x0004081020002000, 0x0008102040004000,
  0x0008040200020400, 0x0010080400040800, 0x0020100A000A1000, 0x0040221400142200, 0x0002442800284400, 0x0004085000500800, 0x0008102000201000, 0x0010204000402000,
  0x0004020002040800, 0x0008040004081000, 0x00100A000A102000, 0x0022140014224000, 0x0044280028440200, 0x0008500050080400, 0x0010200020100800, 0x0020400040201000,
  0x0002000204081000, 0x0004000408102000, 0x000A000A10204000, 0x0014001422400000, 0x0028002844020000, 0x0050005008040200, 0x0020002010080400, 0x0040004020100800,
  0x0000020408102000, 0x0000040810204000, 0x00000A1020400000, 0x0000142240000000, 0x0000284402000000, 0x0000500804020000, 0x0000201008040200, 0x0000402010080400,
  0x0002040810204000, 0x0004081020400000, 0x000A102040000000, 0x0014224000000000, 0x0028440200000000, 0x0050080402000000, 0x0020100804020000, 0x0040201008040200,
];

pub const ROOK_RELEVANT_SQUARES: [u64; 64] = [
  0x000101010101017E, 0x000202020202027C, 0x000404040404047A, 0x0008080808080876, 0x001010101010106E, 0x002020202020205E, 0x004040404040403E, 0x008080808080807E,
  0x0001010101017E00, 0x0002020202027C00, 0x0004040404047A00, 0x0008080808087600, 0x0010101010106E00, 0x0020202020205E00, 0x0040404040403E00, 0x0080808080807E00,
  0x00010101017E0100, 0x00020202027C0200, 0x00040404047A0400, 0x0008080808760800, 0x00101010106E1000, 0x00202020205E2000, 0x00404040403E4000, 0x00808080807E8000,
  0x000101017E010100, 0x000202027C020200, 0x000404047A040400, 0x0008080876080800, 0x001010106E101000, 0x002020205E202000, 0x004040403E404000, 0x008080807E808000,
  0x0001017E01010100, 0x0002027C02020200, 0x0004047A04040400, 0x0008087608080800, 0x0010106E10101000, 0x0020205E20202000, 0x0040403E40404000, 0x0080807E80808000,
  0x00017E0101010100, 0x00027C0202020200, 0x00047A0404040400, 0x0008760808080800, 0x00106E1010101000, 0x00205E2020202000, 0x00403E4040404000, 0x00807E8080808000,
  0x007E010101010100, 0x007C020202020200, 0x007A040404040400, 0x0076080808080800, 0x006E101010101000, 0x005E202020202000, 0x003E404040404000, 0x007E808080808000,
  0x7E01010101010100, 0x7C02020202020200, 0x7A04040404040400, 0x7608080808080800, 0x6E10101010101000, 0x5E20202020202000, 0x3E40404040404000, 0x7E80808080808000,
];

pub const BLACK_QUEEN_SIDE_PATH_TO_ROOK: u64 = 0xe00000000000000;
pub const BLACK_KING_SIDE_PATH_TO_ROOK: u64 = 0x6000000000000000;

pub const WHITE_QUEEN_SIDE_PATH_TO_ROOK: u64 = 0xe;
pub const WHITE_KING_SIDE_PATH_TO_ROOK: u64 = 0x60;