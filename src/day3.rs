#[aoc_generator(day3)]
pub fn generator_input(input: &str) -> Vec<String> {
    input.lines().map(|a| a.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn solve1(ski_map: &Vec<String>) -> u64 {
    return solve(ski_map, 1, 3);
}

#[aoc(day3, part2)]
pub fn solve2(ski_map: &Vec<String>) -> u64 {
   solve(ski_map, 1, 1) * solve(ski_map, 1, 3) * solve(ski_map, 1, 5) * solve(ski_map, 1, 7) * solve(ski_map, 2, 1)
}

pub fn solve(ski_map: &Vec<String>, slope_rise: usize, slope_run: usize) -> u64 {
    let mut trees_hit: u64 = 0;
    let mut column: usize = 0;

    let maximum_rows = ski_map.len();
    let maximum_columns = ski_map[0].len();

    for row in (0..maximum_rows).step_by(slope_rise) {
        let square = ski_map[row].chars().nth(column).unwrap();

        if square == '#' {
            trees_hit += 1;
        }

        column = (column + slope_run) % maximum_columns;
    }

    return trees_hit;
}