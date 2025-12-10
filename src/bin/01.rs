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
    let mut remainder;

    for t in turns {
        let num_only = remove_first_char(t);
        let first_letter = get_first_char(t);

        num = num_only.parse::<i32>().unwrap();

        remainder = num % 100;

        println!("hhh remainder: {remainder}");

        if first_letter == Some('L') {
            num = current_location - remainder;
        } else {
            num = current_location + remainder;
        }

        println!("hhh num: {num}");

        let sec_rem = num % 100;

        println!("hhh sec_rem: {sec_rem} ");

        // Check if our total is at 0 exactly
        if sec_rem == 0 {
            count += 1;
        }

        // Always update the current location of the dial
        if first_letter == Some('L') {
            current_location = num;
        }
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    //     assert_eq!(result, Some(42));
    // }
}
