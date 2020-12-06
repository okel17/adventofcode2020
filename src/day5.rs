#[aoc_generator(day5)]
pub fn generator_input(input: &str) -> Vec<String> {
    input.lines().map(|a| a.to_string()).collect()
}

#[aoc(day5, part1)]
pub fn solve1(boarding_passes: &Vec<String>) -> u32 {
    let seat_ids = get_all_seat_ids(boarding_passes);
    let mut highest_seat_id: u32 = 0;

    for seat_id in seat_ids {
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    return highest_seat_id;
}

#[aoc(day5, part2)]
pub fn solve2(boarding_passes: &Vec<String>) -> u32 {
    let mut seat_ids = get_all_seat_ids(boarding_passes);

    seat_ids.sort();

    for i in 0..(seat_ids.len() - 1) {
        if seat_ids[i+1] - seat_ids[i] == 2 {
            return seat_ids[i] + 1;
        }
    }

    // Don't feel like converting to signed. Being lazy tonight.
    println!("Something went wrong, no seat id found.");
    return 0;
}

pub fn get_all_seat_ids(boarding_passes: &Vec<String>) -> Vec<u32> {
    let mut seat_ids = Vec::new();

    for boarding_pass in boarding_passes {
        let binary_boarding_pass = boarding_pass.chars().map(|c| {
            match c {
                'R' | 'B' => '1',
                'L' | 'F' => '0',
                _ => panic!("Something went wrong in parsing.")
            }
        }).collect::<String>();
        
        let row_binary: String = binary_boarding_pass.chars().take(7).collect();
        let col_binary: String = binary_boarding_pass.chars().skip(7).take(3).collect();

        let row = u32::from_str_radix(row_binary.as_str(), 2).unwrap();
        let col = u32::from_str_radix(col_binary.as_str(), 2).unwrap();

        let seat_id = row * 8 + col;

        seat_ids.push(seat_id);
    }

    return seat_ids
}