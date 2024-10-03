use super::enums::Color;

pub struct Squares;

impl Squares {

  pub const A1: usize = 0;
  pub const B1: usize = 1;
  pub const C1: usize = 2;
  pub const D1: usize = 3;
  pub const E1: usize = 4;
  pub const F1: usize = 5;
  pub const G1: usize = 6;
  pub const H1: usize = 7;

  pub const A2: usize = 8;
  pub const B2: usize = 9;
  pub const C2: usize = 10;
  pub const D2: usize = 11;
  pub const E2: usize = 12;
  pub const F2: usize = 13;
  pub const G2: usize = 14;
  pub const H2: usize = 15;

  pub const A3: usize = 16;
  pub const B3: usize = 17;
  pub const C3: usize = 18;
  pub const D3: usize = 19;
  pub const E3: usize = 20;
  pub const F3: usize = 21;
  pub const G3: usize = 22;
  pub const H3: usize = 23;

  pub const A4: usize = 24;
  pub const B4: usize = 25;
  pub const C4: usize = 26;
  pub const D4: usize = 27;
  pub const E4: usize = 28;
  pub const F4: usize = 29;
  pub const G4: usize = 30;
  pub const H4: usize = 31;

  pub const A5: usize = 32;
  pub const B5: usize = 33;
  pub const C5: usize = 34;
  pub const D5: usize = 35;
  pub const E5: usize = 36;
  pub const F5: usize = 37;
  pub const G5: usize = 38;
  pub const H5: usize = 39;

  pub const A6: usize = 40;
  pub const B6: usize = 41;
  pub const C6: usize = 42;
  pub const D6: usize = 43;
  pub const E6: usize = 44;
  pub const F6: usize = 45;
  pub const G6: usize = 46;
  pub const H6: usize = 47;

  pub const A7: usize = 48;
  pub const B7: usize = 49;
  pub const C7: usize = 50;
  pub const D7: usize = 51;
  pub const E7: usize = 52;
  pub const F7: usize = 53;
  pub const G7: usize = 54;
  pub const H7: usize = 55;

  pub const A8: usize = 56;
  pub const B8: usize = 57;
  pub const C8: usize = 58;
  pub const D8: usize = 59;
  pub const E8: usize = 60;
  pub const F8: usize = 61;
  pub const G8: usize = 62;
  pub const H8: usize = 63;

  pub const ROW_8: [usize; 8] = [Squares::A8, Squares::B8, Squares::C8, Squares::D8, Squares::E8, Squares::F8, Squares::G8, Squares::H8];
  pub const ROW_7: [usize; 8] = [Squares::A7, Squares::B7, Squares::C7, Squares::D7, Squares::E7, Squares::F7, Squares::G7, Squares::H7];
  pub const ROW_6: [usize; 8] = [Squares::A6, Squares::B6, Squares::C6, Squares::D6, Squares::E6, Squares::F6, Squares::G6, Squares::H6];
  pub const ROW_5: [usize; 8] = [Squares::A5, Squares::B5, Squares::C5, Squares::D5, Squares::E5, Squares::F5, Squares::G5, Squares::H5];
  pub const ROW_4: [usize; 8] = [Squares::A4, Squares::B4, Squares::C4, Squares::D4, Squares::E4, Squares::F4, Squares::G4, Squares::H4];
  pub const ROW_3: [usize; 8] = [Squares::A3, Squares::B3, Squares::C3, Squares::D3, Squares::E3, Squares::F3, Squares::G3, Squares::H3];
  pub const ROW_2: [usize; 8] = [Squares::A2, Squares::B2, Squares::C2, Squares::D2, Squares::E2, Squares::F2, Squares::G2, Squares::H2];
  pub const ROW_1: [usize; 8] = [Squares::A1, Squares::B1, Squares::C1, Squares::D1, Squares::E1, Squares::F1, Squares::G1, Squares::H1];

  pub fn to_string(square: usize) -> String {
    match square {
      0 => String::from("A1"),
      1 => String::from("B1"),
      2 => String::from("C1"),
      3 => String::from("D1"),
      4 => String::from("E1"),
      5 => String::from("F1"),
      6 => String::from("G1"),
      7 => String::from("H1"),
      8 => String::from("A2"),
      9 => String::from("B2"),
      10 => String::from("C2"),
      11 => String::from("D2"),
      12 => String::from("E2"),
      13 => String::from("F2"),
      14 => String::from("G2"),
      15 => String::from("H2"),
      16 => String::from("A3"),
      17 => String::from("B3"),
      18 => String::from("C3"),
      19 => String::from("D3"),
      20 => String::from("E3"),
      21 => String::from("F3"),
      22 => String::from("G3"),
      23 => String::from("H3"),
      24 => String::from("A4"),
      25 => String::from("B4"),
      26 => String::from("C4"),
      27 => String::from("D4"),
      28 => String::from("E4"),
      29 => String::from("F4"),
      30 => String::from("G4"),
      31 => String::from("H4"),
      32 => String::from("A5"),
      33 => String::from("B5"),
      34 => String::from("C5"),
      35 => String::from("D5"),
      36 => String::from("E5"),
      37 => String::from("F5"),
      38 => String::from("G5"),
      39 => String::from("H5"),
      40 => String::from("A6"),
      41 => String::from("B6"),
      42 => String::from("C6"),
      43 => String::from("D6"),
      44 => String::from("E6"),
      45 => String::from("F6"),
      46 => String::from("G6"),
      47 => String::from("H6"),
      48 => String::from("A7"),
      49 => String::from("B7"),
      50 => String::from("C7"),
      51 => String::from("D7"),
      52 => String::from("E7"),
      53 => String::from("F7"),
      54 => String::from("G7"),
      55 => String::from("H7"),
      56 => String::from("A8"),
      57 => String::from("B8"),
      58 => String::from("C8"),
      59 => String::from("D8"),
      60 => String::from("E8"),
      61 => String::from("F8"),
      62 => String::from("G8"),
      63 => String::from("H8"),
      _ => String::from("")
    }
  }
}

pub struct BBPositions;

impl BBPositions {

  pub const A1: u64 = 1 << 0;
  pub const B1: u64 = 1 << 1;
  pub const C1: u64 = 1 << 2;
  pub const D1: u64 = 1 << 3;
  pub const E1: u64 = 1 << 4;
  pub const F1: u64 = 1 << 5;
  pub const G1: u64 = 1 << 6;
  pub const H1: u64 = 1 << 7;

  pub const A2: u64 = 1 << 8;
  pub const B2: u64 = 1 << 9;
  pub const C2: u64 = 1 << 10;
  pub const D2: u64 = 1 << 11;
  pub const E2: u64 = 1 << 12;
  pub const F2: u64 = 1 << 13;
  pub const G2: u64 = 1 << 14;
  pub const H2: u64 = 1 << 15;

  pub const A3: u64 = 1 << 16;
  pub const B3: u64 = 1 << 17;
  pub const C3: u64 = 1 << 18;
  pub const D3: u64 = 1 << 19;
  pub const E3: u64 = 1 << 20;
  pub const F3: u64 = 1 << 21;
  pub const G3: u64 = 1 << 22;
  pub const H3: u64 = 1 << 23;

  pub const A4: u64 = 1 << 24;
  pub const B4: u64 = 1 << 25;
  pub const C4: u64 = 1 << 26;
  pub const D4: u64 = 1 << 27;
  pub const E4: u64 = 1 << 28;
  pub const F4: u64 = 1 << 29;
  pub const G4: u64 = 1 << 30;
  pub const H4: u64 = 1 << 31;

  pub const A5: u64 = 1 << 32;
  pub const B5: u64 = 1 << 33;
  pub const C5: u64 = 1 << 34;
  pub const D5: u64 = 1 << 35;
  pub const E5: u64 = 1 << 36;
  pub const F5: u64 = 1 << 37;
  pub const G5: u64 = 1 << 38;
  pub const H5: u64 = 1 << 39;

  pub const A6: u64 = 1 << 40;
  pub const B6: u64 = 1 << 41;
  pub const C6: u64 = 1 << 42;
  pub const D6: u64 = 1 << 43;
  pub const E6: u64 = 1 << 44;
  pub const F6: u64 = 1 << 45;
  pub const G6: u64 = 1 << 46;
  pub const H6: u64 = 1 << 47;

  pub const A7: u64 = 1 << 48;
  pub const B7: u64 = 1 << 49;
  pub const C7: u64 = 1 << 50;
  pub const D7: u64 = 1 << 51;
  pub const E7: u64 = 1 << 52;
  pub const F7: u64 = 1 << 53;
  pub const G7: u64 = 1 << 54;
  pub const H7: u64 = 1 << 55;

  pub const A8: u64 = 1 << 56;
  pub const B8: u64 = 1 << 57;
  pub const C8: u64 = 1 << 58;
  pub const D8: u64 = 1 << 59;
  pub const E8: u64 = 1 << 60;
  pub const F8: u64 = 1 << 61;
  pub const G8: u64 = 1 << 62;
  pub const H8: u64 = 1 << 63;

  pub const ROW_8: [u64; 8] = [BBPositions::A8, BBPositions::B8, BBPositions::C8, BBPositions::D8, BBPositions::E8, BBPositions::F8, BBPositions::G8, BBPositions::H8];
  pub const ROW_7: [u64; 8] = [BBPositions::A7, BBPositions::B7, BBPositions::C7, BBPositions::D7, BBPositions::E7, BBPositions::F7, BBPositions::G7, BBPositions::H7];
  pub const ROW_6: [u64; 8] = [BBPositions::A6, BBPositions::B6, BBPositions::C6, BBPositions::D6, BBPositions::E6, BBPositions::F6, BBPositions::G6, BBPositions::H6];
  pub const ROW_5: [u64; 8] = [BBPositions::A5, BBPositions::B5, BBPositions::C5, BBPositions::D5, BBPositions::E5, BBPositions::F5, BBPositions::G5, BBPositions::H5];
  pub const ROW_4: [u64; 8] = [BBPositions::A4, BBPositions::B4, BBPositions::C4, BBPositions::D4, BBPositions::E4, BBPositions::F4, BBPositions::G4, BBPositions::H4];
  pub const ROW_3: [u64; 8] = [BBPositions::A3, BBPositions::B3, BBPositions::C3, BBPositions::D3, BBPositions::E3, BBPositions::F3, BBPositions::G3, BBPositions::H3];
  pub const ROW_2: [u64; 8] = [BBPositions::A2, BBPositions::B2, BBPositions::C2, BBPositions::D2, BBPositions::E2, BBPositions::F2, BBPositions::G2, BBPositions::H2];
  pub const ROW_1: [u64; 8] = [BBPositions::A1, BBPositions::B1, BBPositions::C1, BBPositions::D1, BBPositions::E1, BBPositions::F1, BBPositions::G1, BBPositions::H1];

  pub const ROW_8_BB: u64 = 0xff00000000000000;
  pub const ROW_7_BB: u64 = 0xff000000000000;
  pub const ROW_6_BB: u64 = 0xff0000000000;
  pub const ROW_5_BB: u64 = 0xff00000000;
  pub const ROW_4_BB: u64 = 0xff000000;
  pub const ROW_3_BB: u64 = 0xff0000;
  pub const ROW_2_BB: u64 = 0xff00;
  pub const ROW_1_BB: u64 = 0xff;

  pub fn get_row_bb(bb: u64) -> u64 {
      for row in [BBPositions::ROW_8_BB, BBPositions::ROW_7_BB, BBPositions::ROW_6_BB,
       BBPositions::ROW_5_BB, BBPositions::ROW_4_BB, BBPositions::ROW_3_BB,
       BBPositions::ROW_2_BB,BBPositions::ROW_1_BB] {
           if bb & row != 0 {
               return row;
           }
       }

       0
  }

  pub fn is_en_passant_position(color: Color, bb_position: u64) -> bool {
    if color.is_white() {
      BBPositions::ROW_3.contains(&bb_position)
    } else {
      BBPositions::ROW_6.contains(&bb_position)
    }
  }
}

pub fn same_rank(sq1: usize, sq2: usize) -> bool {
    sq1 / 8 == sq2 / 8
}

pub fn same_file(sq1: usize, sq2: usize) -> bool {
    sq1 % 8 == sq2 % 8
}

pub fn same_diagonal(sq1: usize, sq2: usize) -> bool {
    (sq1 / 8) as isize - (sq1 % 8) as isize == (sq2 / 8) as isize - (sq2 % 8) as isize
}

pub fn same_anti_diagonal(sq1: usize, sq2: usize) -> bool {
    (sq1 / 8) as isize + (sq1 % 8) as isize == (sq2 / 8) as isize + (sq2 % 8) as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_rank() {
        assert!(same_rank(Squares::A1, Squares::H1));
        assert!(!same_rank(Squares::A1, Squares::A2));
        assert!(same_rank(Squares::D4, Squares::G4));
    }

    #[test]
    fn test_same_file() {
        assert!(same_file(Squares::A1, Squares::A8));
        assert!(!same_file(Squares::A1, Squares::H1));
        assert!(same_file(Squares::D4, Squares::D4 + 24));
    }

    #[test]
    fn test_same_diagonal() {
        assert!(same_diagonal(Squares::A1, Squares::H8));
        assert!(same_diagonal(Squares::A2, Squares::B3));
        assert!(!same_diagonal(Squares::A1, Squares::A8));
        assert!(same_diagonal(Squares::C1, Squares::E3));
        assert!(same_diagonal(Squares::A3, Squares::E7));
    }

    #[test]
    fn test_same_anti_diagonal() {
        assert!(same_anti_diagonal(Squares::H1, Squares::A8));
        assert!(same_anti_diagonal(Squares::A2, Squares::B1));
        assert!(!same_anti_diagonal(Squares::A1, Squares::A8));
        assert!(same_anti_diagonal(Squares::G2, Squares::E4));
    }
}
