extern crate advent_of_code;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use advent_of_code::day_01;

#[test]
fn should_find_last_floor() {
    let p = env::current_dir().unwrap();
    let mut f = File::open(format!("{}/inputs/day_01.txt", p.display())).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    assert_eq!(232, day_01::last_floor(&s));
}

#[test]
fn should_find_index_of_basement() {
    let p = env::current_dir().unwrap();
    let mut f = File::open(format!("{}/inputs/day_01.txt", p.display())).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    assert_eq!(1783, day_01::index_of_basement(&s));
}
