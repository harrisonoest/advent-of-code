use std::cmp::Ordering;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut ret_val = 0;
    for line in input.lines() {
        let mut line_chars: Vec<u32> = line
            .chars()
            .map(|l| l.to_digit(10).unwrap_or_default())
            .collect();

        let first_num_index = line_chars
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap_or_default();

        // need to add logic for when the highest value is at the last index
        // of the vector. In this case, we need to find the second highest value and
        // then use the first value that we found that's at the end

        let first_num = line_chars[first_num_index];

        println!("hhh splitting at index: {first_num_index}");

        let second_list = line_chars.split_off(first_num_index);

        let max_second_num_index = second_list
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap_or_default();

        let second_num = match second_list.len() {
            0 => 0,
            _ => second_list[max_second_num_index],
        };

        println!("first: {first_num}");
        println!("second: {second_num}");
        ret_val += first_num * 10 + second_num;
    }
    Some(ret_val as u64)
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
