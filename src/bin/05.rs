use std::collections::HashSet;

advent_of_code::solution!(5);

// Guesses: 798 (correct)
pub fn part_one(input: &str) -> Option<u64> {
    let ret_val;
    let mut ranges = vec![];
    let mut active_ingredients = vec![];
    let mut line_flipped = false;
    for line in input.lines() {
        // println!("line: {line}");
        if line == "" {
            line_flipped = true;
            continue;
        }
        if line_flipped {
            active_ingredients.push(line.parse::<u64>().unwrap_or_default());
        } else {
            ranges.push(line);
        }
    }

    // println!("ranges: {:?}", ranges);
    // println!("active: {:?}", active_ingredients);

    let mut ret_ingredients: Vec<u64> = vec![];

    for range in ranges {
        let temp: Vec<&str> = range.split("-").collect();
        let lower_limit = temp[0].parse::<u64>().unwrap_or_default();
        let upper_limit = temp[1].parse::<u64>().unwrap_or_default();
        for i in active_ingredients.iter() {
            if lower_limit <= *i && upper_limit >= *i {
                // println!("added ingredient {i} in range {lower_limit}-{upper_limit}");
                if !ret_ingredients.contains(&i) {
                    ret_ingredients.push(*i);
                }
            }
        }
    }

    ret_val = ret_ingredients.len() as u64;

    Some(ret_val)
}

// Guesses: 387859640528018 (too high)
pub fn part_two(input: &str) -> Option<u64> {
    let ret_val;
    let mut ranges = vec![];
    for line in input.lines() {
        if line == "" {
            break;
        }
        ranges.push(line);
    }

    // println!("ranges: {:?}", ranges);

    let mut ret_ingredients = vec![];
    // let lower_limits: Vec<u64> = vec![];
    // let upper_limits: Vec<u64> = vec![];
    let mut limits: Vec<(u64, u64)> = vec![];

    for r in ranges.iter().to_owned() {
        let ran: Vec<&str> = r.split("-").collect();
        let ll = ran[0].parse::<u64>().unwrap_or_default();
        let ul = ran[1].parse::<u64>().unwrap_or_default();
        limits.push((ll, ul));
    }

    limits.sort_by_key(|&(start, _)| start);

    for range in ranges {
        let temp: Vec<&str> = range.split("-").collect();
        let lower_limit = temp[0].parse::<u64>().unwrap_or_default();
        let upper_limit = temp[1].parse::<u64>().unwrap_or_default();
        let mut lower_offset: u64 = 0;
        let mut upper_offset: u64 = 0;

        for limit in limits.iter() {
            if upper_limit < limit.1 && upper_limit > limit.0 {
                let temp_high = limit.1 - upper_limit;
                upper_offset = temp_high;
            }
            if lower_limit > limit.0 && lower_limit < limit.1 {
                let temp_low = lower_limit - limit.0;
                lower_offset = temp_low;
            }
        }

        limits.push((lower_limit, upper_limit));

        println!("lower: {lower_limit}");
        println!("upper: {upper_limit}");
        println!("lower offset: {lower_offset}");
        println!("upper offset: {upper_offset}");

        if lower_offset > 0 {
            lower_offset -= 1;
        }

        if upper_offset > 0 {
            upper_offset -= 1;
        }

        let num = upper_limit - lower_limit - upper_offset - lower_offset;

        println!("num: {num}");

        ret_ingredients.push(num);
    }

    ret_val = ret_ingredients.iter().sum();

    Some(ret_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
