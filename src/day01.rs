use crate::helpers::file_reader;

pub fn run() {
    let mut incr_count = 0;
    let input = file_reader::read(1).unwrap();
    let input = file_reader::string_to_i32(input).unwrap();
    for n in 1..input.len() {
        if input.get(n) > input.get(n - 1) {
            incr_count += 1;
        }
    }
    println!("part1: {}", incr_count);

    incr_count = 0;
    for n in 2..input.len() - 1 {
        let first_sum =
            input.get(n - 2).unwrap() + input.get(n - 1).unwrap() + input.get(n).unwrap();
        let second_sum =
            input.get(n - 1).unwrap() + input.get(n).unwrap() + input.get(n + 1).unwrap();
        if second_sum > first_sum {
            incr_count += 1;
        }
    }
    println!("part2: {}", incr_count);
}
