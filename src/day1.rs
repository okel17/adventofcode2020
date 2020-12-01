use std::collections::HashSet;

#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().expect("Something went wrong in parsing.")).collect()
}

#[aoc(day1, part1)]
fn solver_part1(input: &Vec<i32>) -> i32 {
    let mut seen: HashSet<i32> = HashSet::new();

    for num in input {
        seen.insert(*num);
    }

    for num in input {
        let inverse = 2020 - *num;

        if seen.contains(&inverse) {
            return inverse * *num;
        }
    }
    println!("No two numbers add up to 2020");
    return -1;
}

#[aoc(day1, part2)]
fn solver_part2(input: &Vec<i32>) -> i32 {
    let mut seen: HashSet<i32> = HashSet::new();

    for num in input {
        seen.insert(*num);
    }

    // See if a (2020 - number) is in the input.
    for i in input {
        for j in input {
            let partial_sum = *i + *j;
            let inverse = 2020 - partial_sum;

            if seen.contains(&inverse) {
                return inverse * partial_sum;
            }
        }
    }

    println!("No two numbers add up to 2020");
    return -1;
}
