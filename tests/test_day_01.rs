use std::io::prelude::*;
use std::fs::File;
use std::env;
extern crate advent_of_code;

#[test]
fn it_works() {
    let p = env::current_dir().unwrap();
    let mut f = File::open(format!("{}/src/day_01_input.txt", p.display())).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    assert_eq!(232, advent_of_code::calc_floor(&s));
}
