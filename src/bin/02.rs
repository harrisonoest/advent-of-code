advent_of_code::solution!(2);

fn find_invalid_id_part_one(nums: u64) -> bool {
    let nums_str = nums.to_string();
    let halfway = nums_str.len() / 2;
    let (first_half, second_half) = nums_str.split_at(halfway);
    first_half == second_half
}

fn find_invalid_id_part_two(nums: u64) -> bool {
    let nums_str = nums.to_string();

    for i in 2..=nums_str.len() {
        let sub_str_len = nums_str.len() / i;
        for j in 1..=sub_str_len {
            if nums_str.chars().nth(i) != nums_str.chars().nth(i + j * sub_str_len) {
                continue;
            }
        }
        return true;
    }

    false
}

pub fn part_one(_input: &str) -> Option<u64> {
    /*
    Ideas:  - Split the strings by commas
            - Loop over the strings
            - Split the string by the hyphen
            - The split halves of the string are the range that we need to check
    */

    let parts: Vec<&str> = _input.split(",").collect();
    let mut ret_val = 0;

    for p in parts {
        let range: Vec<&str> = p.split("-").collect();
        let start = range[0].parse::<u64>().unwrap_or_default();
        let end = range[1].parse::<u64>().unwrap_or_default();

        for r in start..=end {
            if find_invalid_id_part_one(r) {
                // println!("Found invalid id: {r} in range: {start}-{end}");
                ret_val += r
            };
        }
    }
    Some(ret_val)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let parts: Vec<&str> = _input.split(",").collect();
    let mut ret_val = 0;

    for p in parts {
        let range: Vec<&str> = p.split("-").collect();
        let start = range[0].parse::<u64>().unwrap_or_default();
        let end = range[1].parse::<u64>().unwrap_or_default();

        for r in start..=end {
            if find_invalid_id_part_two(r) {
                // println!("Found invalid id: {r} in range: {start}-{end}");
                ret_val += r
            };
        }
    }
    Some(ret_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
