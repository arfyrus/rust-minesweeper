pub const W: i32 = 15;
pub const H: i32 = 15;
pub const DISPLAY_SPACE: usize = 5;
pub const BOMB_PERCENT: i32 = 75;

pub fn index(x: i32, y: i32) -> usize {
    (x + y * W) as usize
}