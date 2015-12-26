pub fn calc_floor(s: &str) -> i32 {
    let mut current_floor = 0;
    for c in s.chars() {
        current_floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }
    return current_floor;
}
