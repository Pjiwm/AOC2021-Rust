use crate::helpers::file_reader;
pub fn run() {
    let input = file_reader::read(3).unwrap();
    let bin_len = input.get(0).unwrap().chars().count();
    let mut gamma_bit_vec = Vec::<char>::with_capacity(bin_len);
    let mut epsilon_bit_vec = Vec::<char>::with_capacity(bin_len);
    let mut horizontal_index = 0;

    while horizontal_index < bin_len {
        let mut one_count = 0;
        for n in &input {
            let binary_num = n.chars().nth(horizontal_index).unwrap();
            if binary_num == '1' {
                one_count += 1;
            }
        }

        if one_count > input.len() / 2 {
            gamma_bit_vec.push('1');
            epsilon_bit_vec.push('0');
        } else {
            gamma_bit_vec.push('0');
            epsilon_bit_vec.push('1');
        }
        horizontal_index += 1;
    }
    let gamma_rate = gamma_bit_vec.into_iter().collect::<String>();
    let epsilon_rate = epsilon_bit_vec.into_iter().collect::<String>();
    println!(
        "{}",
        isize::from_str_radix(&gamma_rate, 2).unwrap()
            * isize::from_str_radix(&epsilon_rate, 2).unwrap(),
    );
}
