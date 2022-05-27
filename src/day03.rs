use crate::helpers::file_reader;
pub fn run() {
    let input = file_reader::read(3).unwrap();
    let bin_len = input.get(0).unwrap().chars().count();
    let file_len = input.len();
    let mut gamma_bit_vec = Vec::<char>::with_capacity(bin_len);
    let mut epsilon_bit_vec = Vec::<char>::with_capacity(bin_len);
    let mut horizontal_index = 0;
    // part 2
    let mut oxygen_generator_vec = input.clone();
    let mut co2_scrubber_vec = input.clone();
    let mut oxygen_generator_rule = Vec::<char>::with_capacity(bin_len);
    let mut co2_scrubber_rule = Vec::<char>::with_capacity(bin_len);

    while horizontal_index < bin_len {
        let mut one_count = 0;
        let first_bin = input.get(0).unwrap().chars().nth(horizontal_index).unwrap();
        let mut one_count = 0;
        for n in &input {
            let binary_num = n.chars().nth(horizontal_index).unwrap();
            if binary_num == '1' {
                one_count += 1;
            }

            if first_bin == '1' {
                one_count += 1;
            }
        }

        // part 1 count most common bit
        if one_count > input.len() / 2 {
            gamma_bit_vec.push('1');
            epsilon_bit_vec.push('0');
        } else {
            gamma_bit_vec.push('0');
            epsilon_bit_vec.push('1');
        }
        horizontal_index += 1;

        // part 2 check the majority
        if one_count >= file_len / 2 {
            oxygen_generator_rule.push('1');
            co2_scrubber_rule.push('0');
        } else {
            co2_scrubber_rule.push('1');
            oxygen_generator_rule.push('0');
        }
    }

    // Filtering binaries
    horizontal_index = 0;
    while horizontal_index < bin_len {
        // TODO fix this auto generated junk
        for (o, c) in oxygen_generator_vec.iter_mut().zip(co2_scrubber_vec.iter_mut()) {
            let o_bit = o.chars().nth(horizontal_index).unwrap();
            let c_bit = c.chars().nth(horizontal_index).unwrap();
            println!("{} {}", o_bit, c_bit);
        }
        horizontal_index += 1;
    }

    // Finish part 1
    let gamma_rate = gamma_bit_vec.into_iter().collect::<String>();
    let epsilon_rate = epsilon_bit_vec.into_iter().collect::<String>();
    println!(
        "{}",
        isize::from_str_radix(&gamma_rate, 2).unwrap()
            * isize::from_str_radix(&epsilon_rate, 2).unwrap(),
    );
}
