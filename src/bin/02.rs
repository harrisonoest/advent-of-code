advent_of_code::solution!(2);

fn get_digit_at(n: u32, idx: usize) -> Option<char> {
    Some(n.to_string().chars().nth(idx)?)
}

fn find_sub_strings(num: u32, start: usize, size: usize) -> String {
    let mut ret: String = String::from("");
    for i in start..=start + size {
        ret.push(get_digit_at(num, i).unwrap_or_default());
    }
    ret
}

fn find_largest_match(r: u32) -> u32 {
    // println!("hhh r: {r}");
    // Take the first two digits of the number
    // Compare them to the next two digits in the number
    // Set the value if they match
    // Compare the first three digits of the number
    // Compare them against the next three digits of the number
    // Set the value if they match
    // Loop on this until the number is larger than half of its parent
    let mut temp_ret_val = 0;
    let mut r_start: usize = 0;
    let r_size: usize = 2;
    let mut r_sub = String::new();
    let mut r_sec_sub = String::new();
    let r_half_size: usize = r.to_string().len() / 2;

    while (r_size + r_start) <= r_half_size {
        r_sub = find_sub_strings(r, r_start, r_size);
        r_sec_sub = find_sub_strings(r, r_start + r_size, r_start + r_size + r_size);

        if r_sub == r_sec_sub {
            temp_ret_val = r_sub.parse::<u32>().unwrap();
        }
        r_start += 1;
    }
    println!("hhh r_sub: {r_sub}");
    println!("hhh r_sec_sub: {r_sec_sub}");
    println!("hhh temp_ret_val: {temp_ret_val}");

    temp_ret_val
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
        let start = range[0].parse::<u32>().unwrap_or_default();
        let end = range[1].parse::<u32>().unwrap_or_default();

        for r in start..=end {
            ret_val += find_largest_match(r);
        }

        println!("hhh ret_val: {ret_val}");
    }

    None
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
