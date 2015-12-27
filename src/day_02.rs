use std::str::FromStr;

pub fn amount_of_paper(s: &str) -> i32 {
    let mut total_area = 0;
    for line in s.split('\n') {
        let tokens: Vec<&str> = line.split('x').collect();
        if tokens.len() != 3 {
            continue;
        }
        let l: i32 = i32::from_str(tokens[0]).unwrap();
        let w: i32 = i32::from_str(tokens[1]).unwrap();
        let h: i32 = i32::from_str(tokens[2]).unwrap();

        let mut d = Vec::new();
        d.push(l * w);
        d.push(w * h);
        d.push(l * h);

        let all_sides = 2 * d.iter().fold(0, |sum, x| sum + x);
        let min_side = d.iter().min().unwrap();
        total_area += all_sides + min_side;
    }
    return total_area;
}


pub fn amount_of_ribbon(s: &str) -> i32 {
    let mut total_area = 0;
    for line in s.split('\n') {
        let strs: Vec<&str> = line.split('x').collect();
        if strs.len() != 3 {
            continue;
        }
        let mut d: Vec<i32> = strs.iter()
                                  .map(|&x| i32::from_str(x).unwrap())
                                  .collect();
        d.sort();

        let wrap = d[0] * 2 + d[1] * 2;
        let bow = d.iter().fold(1, |product, x| product * x);
        total_area += wrap + bow;
    }
    return total_area;
}
