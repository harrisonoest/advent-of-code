advent_of_code::solution!(3);

// Guesses: 17589 (too high), 17316 (correct)
pub fn part_one(input: &str) -> Option<u64> {
    let mut ret_val = 0;
    for line in input.lines() {
        let mut line_chars: Vec<u32> = line
            .chars()
            .map(|l| l.to_digit(10).unwrap_or_default())
            .collect();

        let mut first_num: u32 = 0;
        let mut second_num: u32 = 0;
        let mut first_index = 0;
        let mut second_index;

        // Find first number
        for i in (1..10).rev() {
            first_index = line_chars
                .iter()
                .position(|&r| r == i)
                .unwrap_or_else(|| line_chars.len() - 1);
            if first_index == line_chars.len() - 1 {
                continue;
            } else {
                if first_num < line_chars[first_index] {
                    first_num = line_chars[first_index];
                    break;
                }
            }
        }

        let mut second_half = line_chars.split_off(first_index);

        // Throw away the first char
        second_half.remove(0);

        // Find second number
        for j in (1..10).rev() {
            second_index = second_half
                .iter()
                .position(|&r| r == j)
                .unwrap_or_else(|| 0);
            let temp_num = second_half[second_index];
            if second_num < temp_num {
                second_num = temp_num;
            }
        }
        ret_val += first_num * 10 + second_num;
    }
    Some(ret_val as u64)
}

// Guesses: 171741365473332 (correct)
pub fn part_two(input: &str) -> Option<u64> {
    let mut ret_val: u64 = 0;

    for line in input.lines() {
        let line_chars: Vec<u32> = line
            .chars()
            .map(|l| l.to_digit(10).unwrap_or_default())
            .collect();

        let mut stack: Vec<u32> = Vec::new();
        let mut remove = line_chars.len() - 12;

        for n in line_chars {
            while remove > 0 && !stack.is_empty() && *stack.last().unwrap() < n {
                stack.pop();
                remove -= 1;
            }
            stack.push(n);
        }

        stack.truncate(12);

        let base: u64 = 10;
        let temp: u64 = stack.iter().fold(0u64, |acc, &d| acc * base + d as u64);
        ret_val += temp as u64;
    }

    Some(ret_val as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
