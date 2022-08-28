pub mod cell;
pub mod header;

use rand::Rng;
use crate::header::*;
use crate::cell::*;

fn print_grid(grid: &Vec<Cell>) {
    const SPACE: usize = 3;
    for y in -1..H {
        for x in -1..W {
            if y == -1 {
                print!("[{:=^SPACE$}]", x + 1);
                continue;
            } else if x == -1 {
                print!("[{:=^SPACE$}]", y + 1);
                continue;
            }
            let shown: String = {
                if grid[index(x, y)].reveal {
                    match grid[index(x, y)].ctype {
                        CellType::Bomb => format!("B"),
                        CellType::Field(num) => format!("{}", num),
                    }
                } else {
                    format!("##")
                }
            };
            print!(" {:^SPACE$} ", shown);
        }
        print!("\n");
    }
}

fn main() {
    let mut grid: Vec<Cell> = Vec::new();
    for _i in 0..W * H {
        let choice = rand::thread_rng().gen_range(1..=100);
        grid.push(new(choice >= 75));
    }

    for y in 0..H {
        for x in 0..W {
            if let CellType::Bomb = grid[index(x, y)].ctype {
                continue;
            }
            for y2 in -1..=1 {
                for x2 in -1..=1 {
                    if x + x2 >= 0 && y + y2 >= 0 && x + x2 < W && y + y2 < H {
                        let chosen = &mut grid[index(x + x2, y + y2)];
                        if let CellType::Bomb = chosen.ctype {
                            if let CellType::Field(num) = grid[index(x, y)].ctype {
                                grid[index(x, y)].ctype = CellType::Field(num + 1);
                            }
                        }
                    }
                }
            }
        }
    }
    print_grid(&grid);
}