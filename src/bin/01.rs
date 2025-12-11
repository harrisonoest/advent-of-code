advent_of_code::solution!(1);

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

pub fn part_one(input: &str) -> Option<u64> {
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
    let turns = input.split_whitespace();
    let mut current_location = 50;
    let mut count: u64 = 0;
    let mut num;

    for t in turns {
        let num_only = remove_first_char(t);
        let first_letter = get_first_char(t);

        num = num_only.parse::<i32>().unwrap();

        if first_letter == Some('L') {
            current_location -= num;
        } else {
            current_location += num;
        }

        if current_location > 99 {
            current_location = current_location % 100;
        }

        if current_location.is_negative() {
            current_location = 100 - (current_location % 100).abs();
        }

        // println!("Rotating {t}, dial now at position: {current_location}");

        if (current_location % 100) == 0 {
            count += 1;
        }
    }
    // Return the count
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    /*
       TODO: Need to count the number of times the dial passes or lands on zero

       Brainstorm: - Get rid of negatives by adding a really large number.
                   - Mod the result and update the count when it is zero
    */
    let turns = input.split_whitespace();
    let mut count: u64 = 0;
    let mut current_location = 1000000000 + 50;

    for t in turns {
        let num_only = remove_first_char(t);
        let first_letter = get_first_char(t);
        let num = num_only.parse::<i32>().unwrap();

        for _n in 1..=num {
            match first_letter {
                Some('L') => current_location -= 1,
                _ => current_location += 1,
            }
            if current_location % 100 == 0 {
                count += 1;
                // println!("Updating count: {count}");
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let expected = Some(3);
        println!("Expected: {}", expected.unwrap_or_default());
        println!("Result: {}", result.unwrap_or_default());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        let expected = Some(1066);
        println!("Expected: {}", expected.unwrap_or_default());
        println!("Result: {}", result.unwrap_or_default());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let expected = Some(6);
        println!("Expected: {}", expected.unwrap_or_default());
        println!("Result: {}", result.unwrap_or_default());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        let expected = Some(6223);
        println!("Expected: {}", expected.unwrap_or_default());
        println!("Result: {}", result.unwrap_or_default());
        assert_eq!(result, expected);
    }
}
