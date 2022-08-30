pub mod cell;
pub mod config;

use crate::cell::*;
use crate::config::*;
use rand::Rng;
use std::io;

fn index(x: i32, y: i32) -> usize {
    (x + y * W) as usize
}

fn print_grid(grid: &mut Vec<Cell>) {
    const SPACE: usize = DISPLAY_SPACE;
    for y in -1..H {
        for x in -1..W {
            if y == -1 {
                print!("[{:=^width$}]", x + 1, width = SPACE - 2);
                continue;
            } else if x == -1 {
                print!("[{:=^width$}]", y + 1, width = SPACE - 2);
                continue;
            }
            let shown: String = {
                if grid[index(x, y)].reveal {
                    match grid[index(x, y)].ctype {
                        CellType::Bomb => format!("(*)"),
                        CellType::Field(num) => {
                            if num == 0 {
                                format!(" ")
                            } else {
                                format!("{}", num)
                            }
                        },
                    }
                } else if grid[index(x, y)].flag {
                    format!("%")
                } else {
                    format!(": :")
                }
            };
            if grid[index(x, y)].last_sel {
                print!(">{:^width$}<", shown, width = SPACE - 2);
                grid[index(x, y)].last_sel = false;
            } else {
                print!("{:^SPACE$}", shown);
            }
        }
        print!("\n");
    }
}

fn main() {
    let mut grid: Vec<Cell> = Vec::new();
    let mut _bomb_amount: usize = 0;

    for _i in 0..W * H {
        let choice = rand::thread_rng().gen_range(1..=100);
        if choice >= BOMB_PERCENT {
            grid.push(new(true));
            _bomb_amount += 1;
        } else {
            grid.push(new(false));
        }
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

    'game_loop: loop {
        print_grid(&mut grid);

        println!("Reveal(r) or flag(f)?");
        let mut r_or_f = String::new();
        io::stdin().read_line(&mut r_or_f).expect("Failed to read line.");
        let r_or_f = r_or_f.trim();

        println!("Choose a spot to {}. (x-ENT-y-ENT)", match r_or_f {
            "r" => String::from("reveal"),
            "f" => String::from("flag"),
            _ => {
                println!("Invalid choice");
                continue 'game_loop;
            }
        });

        let mut pos_x = String::new();
        io::stdin().read_line(&mut pos_x).expect("Failed to read line");
        let mut pos_y = String::new();
        io::stdin().read_line(&mut pos_y).expect("Failed to read line");

        let pos_x: i32 = pos_x.trim().parse().expect("Not a number");
        let pos_x = pos_x - 1;
        let pos_y: i32 = pos_y.trim().parse().expect("Not a number");
        let pos_y = pos_y - 1;

        match r_or_f {
            "r" => {
                grid[index(pos_x, pos_y)].last_sel = true;
                if grid[index(pos_x, pos_y)].reveal() {
                    println!("You lost!");
                    break 'game_loop;
                }
            }
            "f" => {
                grid[index(pos_x, pos_y)].last_sel = true;
                if grid[index(pos_x, pos_y)].flag() {
                    _bomb_amount -= 1;
                    if _bomb_amount == 0 {
                        println!("You win!");
                        break 'game_loop;
                    }
                }
            }
            _ => println!("Invalid input"),
        }
    }

    for i in &mut grid {
        i.reveal = true;
    }
    print_grid(&mut grid);
}
