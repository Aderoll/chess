struct Board {
    white: Bitmap,
    black: Bitmap,
    pieces: [Piece; 64],
}

struct Bitmap {
    map: u64,
}

#[derive(Clone, Copy, Debug)]
enum Piece {
    Empty,
    WhitePawn,
    WhiteCastle,
    WhiteKnight,
    WhiteBishop,
    WhiteKing,
    WhiteQueen,
    BlackPawn,
    BlackCastle,
    BlackKnight,
    BlackBishop,
    BlackKing,
    BlackQueen,
}

impl Board {
    fn new() -> Board {
        Board {
            white: Bitmap {
                map: 0xFFFF, //(2 ^ 16 - 1)
            },
            black: Bitmap {
                map: 0xFFFF000000000000, //((2 ^ 63 - 1) - (2 ^ 48 - 1) + 2 ^ 64)
            },
            pieces: [Piece::Empty; 64],
        }
    }
    fn has_piece(&self, position: usize) -> bool {
        if (self.combined().map >> position) & 1 != 0 {
            return true;
        }
        false
    }
    fn is_empty(&self) -> bool {
        self.combined().map == 0
    }
    fn is_single_pop(&self) -> bool {
        self.combined().map != 0 && (self.combined().map & (self.combined().map - 1)) == 0
    }
    fn combined(&self) -> Bitmap {
        Bitmap {
            map: (self.white.map | self.black.map),
        }
    }
    fn print(&self) -> () {
        let white = self.white.map;
        let black = self.black.map;
        println!("white: {white}\nblack: {black}\n");
        for i in 0..8 {
            for j in 0..8 {
                if self.has_piece(i * 8 + j) {
                    print!("1");
                } else {
                    print!("0");
                }
            }
            println!();
        }
    }
}

pub fn run() -> () {
    let board = Board::new();
    board.print();
}
