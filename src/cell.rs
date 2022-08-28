pub enum CellType {
    Bomb,
    Field(u8),
}

pub struct Cell {
    pub reveal: bool,
    pub flag: bool,
    pub ctype: CellType,
}

impl Cell {}

pub fn reveal(cell: &mut Cell /*, x: i32, y: i32, grid: &mut Vec<Cell>*/) -> bool {
    if let CellType::Bomb = cell.ctype {
        return true;
    }
    if cell.reveal {
        return false;
    }
    cell.reveal = true;
    // if let CellType::Field(num) = cell.ctype {
    //     if num > 0 {
    //         return false;
    //     }
    // }
    // for y2 in -1..=1 {
    //     for x2 in -1..=1 {
    //         if x + x2 >= 0 && y + y2 >= 0 && x + x2 < super::header::W && x + y2 < super::header::H {
    //             let mut grid2 = &mut *grid;
    //             reveal(
    //                 &mut grid2[super::header::index(x + x2, y + y2)],
    //                 x + x2,
    //                 y + y2,
    //                 &mut grid2
    //             );
    //         }
    //     }
    // }
    return false;
}

pub fn new(bomb: bool) -> Cell {
    use CellType::*;
    Cell {
        reveal: false,
        flag: false,
        ctype: if bomb {
            Bomb
        } else {
            Field(0)
        },
    }
}