pub fn last_floor(s: &str) -> i32 {
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

pub fn index_of_basement(s: &str) -> i32 {
    let mut current_floor = 0;
    let mut index = 0;
    for c in s.chars() {
        current_floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if current_floor == -1 {
            return index + 1;
        }
        index += 1;
    }
    return index;
}
