use std::convert::TryInto;

#[aoc_generator(day2)]
pub fn generator_input_2(input: &str) -> Vec<PassKey> {
    let pass_keys = input.lines().map(|l| {
        let replaced_lines = l.trim().replace("-", " ").replace(":", "");
        let reformatted_lines = replaced_lines.split(" ").collect::<Vec<&str>>();
        let first_number = reformatted_lines[0].parse::<u32>().unwrap();
        let second_number = reformatted_lines[1].parse::<u32>().unwrap();

        PassKey {
            first_number: first_number,
            second_number: second_number,
            letter: reformatted_lines[2].to_string(),
            password_under_test: reformatted_lines[3].to_string()
        }
    }).collect::<Vec<PassKey>>();

    return pass_keys;
}

pub struct PassKey {
    first_number: u32,
    second_number: u32,
    letter: String,
    password_under_test: String
}

#[aoc(day2, part1)]
pub fn solve_part1(passkeys: &Vec<PassKey>) -> u32 {
    let is_valid_arr: Vec<bool> = passkeys.iter().map(|p| {
        let pass = p.password_under_test.as_str();
        let letter = p.letter.as_str();

        let matches: u32 = pass.matches(letter).count().try_into().unwrap();

        matches >= p.first_number && matches <= p.second_number
    }).collect();

    let keep_valid = is_valid_arr.into_iter().filter(|&x| x == true).collect::<Vec<bool>>();

    return keep_valid.len().try_into::<>().unwrap();
}

#[aoc(day2, part2)]
pub fn solve_part2(passkeys: &Vec<PassKey>) -> u32 {
    let is_valid_arr: Vec<bool> = passkeys.iter().map(|p| {
        let first_index: usize = p.first_number.try_into::<>().unwrap();
        let second_index: usize = p.second_number.try_into::<>().unwrap();

        let matches_first_index: bool = p.password_under_test.chars().nth(first_index - 1).unwrap() == p.letter.chars().nth(0).unwrap();
        let matches_second_index: bool = p.password_under_test.chars().nth(second_index - 1).unwrap() == p.letter.chars().nth(0).unwrap();

        return (matches_first_index && !matches_second_index) || (!matches_first_index && matches_second_index);
    }).collect::<Vec<bool>>();

    let keep_valid = is_valid_arr.into_iter().filter(|&x| x == true).collect::<Vec<bool>>();

    return keep_valid.len().try_into::<>().unwrap();
}
