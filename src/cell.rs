pub enum CellType {
    Bomb,
    Field(u8),
}

pub struct Cell {
    pub reveal: bool,
    pub flag: bool,
    pub ctype: CellType,
}

impl Cell {
    // pub fn reveal(&mut self, pos: usize, grid: &mut [Cell]) {
    //     if let CellType::Bomb = self.ctype {
    //         return;
    //     } else if self.reveal {
    //         return;
    //     }
    //     self.reveal = true;
    //     for y in (1..=-1).rev() {
    //         for x in (1..=-1).rev() {
    //             let index: usize = super::header::index(x as i32, y as i32) as usize;
    //             let _chosen = grid[pos + index];
    //             // println!("{}", chosen);
    //         }
    //     }
    // }
}

pub fn new(bomb: bool) -> Cell {
    use CellType::*;
    Cell {
        reveal: true,
        flag: false,
        ctype: if bomb { Bomb } else { Field(0) },
    }
}