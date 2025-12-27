use std::u64;

advent_of_code::solution!(6);

fn sum_column(matrix: &[Vec<&str>], index: usize) -> Result<u64, std::num::ParseIntError> {
    matrix
        .iter()
        .map(|row| row[index].parse::<u64>())
        .try_fold(0u64, |acc, x| Ok(acc + x?))
}

fn multiply_column(matrix: &[Vec<&str>], index: usize) -> Result<u64, std::num::ParseIntError> {
    matrix
        .iter()
        .map(|row| row[index].parse::<u64>())
        .try_fold(1u64, |acc, x| Ok(acc * x?))
}

// Guesses: 6503327062445 (correct)
pub fn part_one(input: &str) -> Option<u64> {
    let mut ret_val: u64 = 0;
    let mut problems: Vec<Vec<&str>> = vec![];
    let mut operations: Vec<&str> = vec![];

    for line in input.lines() {
        if line.chars().nth(0)? == '*' || line.chars().nth(0)? == '+' {
            operations = line.split_whitespace().collect();
        } else {
            problems.push(line.split_whitespace().collect());
        }
    }

    for i in 0..=problems[0].len() - 1 {
        println!("i: {i}");
        if operations[i] == "+" {
            ret_val += sum_column(&problems, i).unwrap_or_default();
        } else if operations[i] == "*" {
            ret_val += multiply_column(&problems, i).unwrap_or_default();
        }
    }

    Some(ret_val)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
