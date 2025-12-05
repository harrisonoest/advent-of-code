advent_of_code::solution!(1);

fn solve_01(input: &str) -> Option<u64> {
    /*
    TODO:
    - Split file into data structure based on newline
        - The data structure could be a tuple of { Direction, Steps }
    - Loop over the ordered data structure, calculating the new position at each step
        - If the end value is zero, increment a tracking variable
    - Return the tracking variable

    Brainstorm:
    - Can we use positive and negative numbers instead of "L" and "R"?
    - Can we handle the addition/subtraction while reading the string?
    */
    fn get_first_char(s: &str) -> Option<char> {
        return s.chars().next();
    }

    fn remove_first_char(s: &str) -> &str {
        return s
            .char_indices()
            .nth(1)
            .and_then(|(i, _)| s.get(i..))
            .unwrap_or("");
    }

    let turns = input.split_whitespace();
    let mut current_location = 50;
    let mut count: u64 = 0;
    let mut num;

    for t in turns {
        let num_only = remove_first_char(t);

        num = num_only.parse::<i32>().unwrap();

        if get_first_char(t) == Some('L') {
            num *= -1;
        }

        // Check if the number is larger than 99
        // If it is, need to get the last two digits
        // e.g. 234 would go around twice, so 34 is the value to use
        if num > 99 {
            num = remove_first_char(&num.to_string()[..])
                .parse::<i32>()
                .unwrap();
        } else if num < -99 {
            num *= -1;
            num = remove_first_char(&num.to_string()[..])
                .parse::<i32>()
                .unwrap_or_default();
        }

        let mut sum = current_location + num;

        // Check for conditions that go past the range
        if sum < 0 {
            // take the remainder past 0 of the addition and subtract from 99
            sum += 100
        } else if sum > 99 {
            // take the remainder past 99 and add it to 0
            sum -= 99
        }

        // Check if our total is at 0 exactly
        if sum == 0 {
            count += 1;
        }

        // Always update the current location of the dial
        current_location = sum;
    }

    // Return the count
    Some(count)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve_01(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_01(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
