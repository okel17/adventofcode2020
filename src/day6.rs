//TODO: try itertools.
use std::convert::TryInto;

#[aoc_generator(day6)]
pub fn generator_input(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>()
}

#[aoc(day6, part1)]
pub fn solve1(group_forms: &Vec<String>) -> u32 {
    let mut selected_responses: u32 = 0;
    
    for group_form in group_forms {
        let mut form_chars = group_form.as_str().chars().filter(|c| *c != '\n').collect::<Vec<char>>();

        form_chars.sort();
        form_chars.dedup();

        let uniquely_selected_responses: u32 = form_chars.len().try_into().unwrap();
        selected_responses += uniquely_selected_responses
    }

    return selected_responses;
}

#[aoc(day6, part2)]
pub fn solve2(group_forms: &Vec<String>) -> u32 {
    let mut selected_responses: u32 = 0;
    
    for group_form in group_forms {
        let mut user_forms = group_form.split("\n").map(|s| s.to_string());

        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            if user_forms.clone().all(|form| {form.matches(c).count() > 0}) {
                selected_responses += 1;
            }
        }
    }

    return selected_responses;
}