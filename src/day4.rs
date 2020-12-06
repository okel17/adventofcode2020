use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn generator_input(input: &str) -> Vec<HashMap<String, String>> {
    /* 
     * REFLECTION: Why not just split on \n\n? Also, could have used the opportunity to play with Option types or Regex.
     * 
     * 
     * Algorithm to reformat for ease of use:
     * 1. Convert newlines to tildes.
     * 2. Double tildes means a break between passports, change back to single newline.
     * 3. Single tilde means we're within a passport, change it to a space to keep whole passport in one line.
     * 4. Turn into key-value pairs.
     */ 
    let reformatted_input = input.replace("\n", "~").replace("~~", "\n").replace("~", " ");
    
    reformatted_input.lines().map(|passport| {
        let fields = passport.split(" ");
        let mut reformatted_passport = HashMap::new();

        for field in fields {
            let split_fields = field.split(":").collect::<Vec<&str>>();
            reformatted_passport.insert(split_fields[0].to_string(), split_fields[1].to_string());
        }

        return reformatted_passport;
    }).collect()
}


#[aoc(day4, part1)]
pub fn solve1(passports: &Vec<HashMap<String, String>>) -> u32 {
    let mut valid_passports_count = 0;

    for passport in passports {
        if has_all_required_fields(passport) {  
            valid_passports_count += 1;
        }
    }

    return valid_passports_count;
}

#[aoc(day4, part2)]
pub fn solve2(passports: &Vec<HashMap<String, String>>) -> u32 {
    let mut valid_passports_count = 0;

    for passport in passports {
        if has_all_required_fields(passport) && 
           is_valid_birth_year(passport.get("byr").unwrap()) &&
           is_valid_issue_year(passport.get("iyr").unwrap()) &&
           is_valid_height(passport.get("hgt").unwrap().to_string()) &&
           is_valid_expiration_year(passport.get("eyr").unwrap()) &&
           is_valid_hair_color(passport.get("hcl").unwrap().to_string()) &&
           is_valid_eye_color(passport.get("ecl").unwrap()) &&
           is_valid_passport_id(passport.get("pid").unwrap()) {  
            valid_passports_count += 1;
        }
    }

    return valid_passports_count;
}

pub fn has_all_required_fields(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr") &&
    passport.contains_key("iyr") &&
    passport.contains_key("eyr") &&
    passport.contains_key("hgt") &&
    passport.contains_key("hcl") &&
    passport.contains_key("ecl") &&
    passport.contains_key("pid") 
    /* cid is totally optional, just ignore it. */
}

pub fn is_valid_birth_year(byr_string: &String) -> bool {
    let byr = byr_string.parse::<u32>().expect("Something went wrong converting byr.");
    
    byr >= 1920 && byr <= 2002

}

pub fn is_valid_issue_year(iyr_string: &String) -> bool {
    let iyr = iyr_string.parse::<u32>().expect("Something went wrong converting iyr.");
    
    iyr >= 2010 && iyr <= 2020
}

pub fn is_valid_expiration_year(eyr_string: &String) -> bool {
    let eyr = eyr_string.parse::<u32>().expect("Something went wrong converting eyr.");
    
    eyr >= 2020 && eyr <= 2030
}

pub fn is_valid_height(hgt: String) -> bool {
    if hgt.matches("cm").count() > 0 {
        let mut mutable_height = hgt.clone();

        mutable_height.retain(|c| c.is_numeric());
        let height = mutable_height.parse::<u32>().expect("Something went wrong converting hgt.");    
        return height >= 150 && height <= 193;
    }
    else if hgt.matches("in").count() > 0 {
        let mut mutable_height = hgt.clone();

        mutable_height.retain(|c| c.is_numeric());
        let height = mutable_height.parse::<u32>().expect("Something went wrong converting hgt.");
        return height >= 59 && height <= 76;
    }
    
    return false;
}

pub fn is_valid_hair_color(hcl: String) -> bool {
    let has_tag = hcl.chars().nth(0).unwrap() == '#';
    let mut mutable_hair_color = hcl.clone();

    mutable_hair_color.remove(0);

    return has_tag && mutable_hair_color.chars().all(char::is_alphanumeric);
}

pub fn is_valid_eye_color(ecl: &String) -> bool {
    match ecl.as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false
    }
}

pub fn is_valid_passport_id(pid_string: &String) -> bool {
    return pid_string.chars().count() == 9 && pid_string.chars().all(char::is_numeric);
}