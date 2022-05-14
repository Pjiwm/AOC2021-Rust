use core::panic;
use crate::helpers::file_reader;

pub fn run() {
    let input = file_reader::read(2).unwrap();
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;
    let mut new_depth = 0;

    for n in input {
        let split = n.split(' ').collect::<Vec<&str>>();
        let direction = *split.get(0).unwrap();
        let amount = split.get(1).unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => {
                forward += amount;
                new_depth += aim * amount;
            }
            "down" => {
                depth += amount;
                aim += amount;
            }
            "up" => {
                depth -= amount;
                aim -= amount;
            }
            _ => panic!("Unknown command"),
        }
    }

    println!("part1: {}", forward * depth);
    println!("part2: {}", forward * new_depth);
}
