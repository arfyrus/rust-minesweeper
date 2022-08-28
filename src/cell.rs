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
    pub fn reveal(&mut self) -> bool {
        if self.reveal {
            return false;
        }
        self.reveal = true;

        if let CellType::Bomb = self.ctype {
            return true;
        }
        return false;
    }

    pub fn flag(&mut self) -> bool {
        self.flag = true;
        if let CellType::Bomb = self.ctype {
            return true;
        }
        return false;
    }
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