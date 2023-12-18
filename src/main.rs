pub type BitBoard = u64;

#[repr(usize)]
#[rustfmt::skip]
pub enum SquareLabels {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

pub fn set_bit(mut bitboard: BitBoard, square: usize) -> BitBoard{
    bitboard |= 1u64 << square;
    return bitboard;
}

pub fn get_bit(bitboard: BitBoard, square: usize) -> bool {
    if bitboard & 1u64 << square != 0 {
        return true
    } else {
        return false
    }
}

pub fn pop_bit(mut bitboard: BitBoard, square: usize) -> BitBoard{
    bitboard ^= 1u64 << square;
    return bitboard;
}


pub fn print_bitboard(bitboard: BitBoard) -> BitBoard {
    println!();
    for r in 0..8 {
        print!("  {}   ", 8 - r);
        for f in 0..8 {
            let square = 8 * r + f;
            if get_bit(bitboard, square) == true {
                print!("1 ")
            } else { 
                print!("0 ")
            }
        }
        println!()
    }
    println!("\n      a b c d e f g h");

    return bitboard
}

pub fn set_position(fen: &str) -> [BitBoard; 15] {
    let mut position:[u64; 15] = [0u64; 15];
    let fen_parts: Vec<&str> = fen.split(" ").collect();
    let board_rows: Vec<&str> = fen_parts[0].split("/").collect();
    for i in 0..8 {
        let pieces = board_rows[i].chars();
        for p in pieces {
            match p {
                '8' => print!(". . . . . . . . "),
                '7' => print!(". . . . . . . "),
                '6' => print!(". . . . . . "),
                '5' => print!(". . . . . "),
                '4' => print!(". . . . "),
                '3' => print!(". . . "),
                '2' => print!(". . "),
                '1' => print!(". "),
                _ => print!("{} ", p)
            }
        }
        println!();
    }
    
    let bitboard = 0u64;
    return position
}


fn main() {
    let mut bitboard: BitBoard = 0u64;

    bitboard = set_bit(bitboard, SquareLabels::A6 as usize);
    bitboard = set_bit(bitboard, SquareLabels::E4 as usize);
    bitboard = set_bit(bitboard, SquareLabels::D7 as usize);
    bitboard = set_bit(bitboard, SquareLabels::G1 as usize);

    bitboard = print_bitboard(bitboard);

    bitboard = pop_bit(bitboard, SquareLabels::D7 as usize);

    bitboard = print_bitboard(bitboard);

    let fen: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";

    let position: [u64; 15] = set_position(fen);

    println!("{:?}", position);

}