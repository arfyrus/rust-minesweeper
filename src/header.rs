pub const W: i32 = 10;
pub const H: i32 = 10;
pub fn index(x: i32, y: i32) -> usize {
    (x + y * W) as usize
}