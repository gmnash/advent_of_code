extern crate advent_of_code;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use advent_of_code::day_02;

#[test]
fn should_calc_amount_of_paper() {
    let p = env::current_dir().unwrap();
    let mut f = File::open(format!("{}/inputs/day_02.txt", p.display())).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    assert_eq!(1586300, day_02::amount_of_paper(&s));
}

#[test]
fn should_calc_amount_of_ribbon() {
    let p = env::current_dir().unwrap();
    let mut f = File::open(format!("{}/inputs/day_02.txt", p.display())).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    assert_eq!(3737498, day_02::amount_of_ribbon(&s));
}
