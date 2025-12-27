advent_of_code::solution!(5);

// Guesses: 798 (correct)
pub fn part_one(input: &str) -> Option<u64> {
    let ret_val;
    let mut ranges = vec![];
    let mut active_ingredients = vec![];
    let mut line_flipped = false;
    for line in input.lines() {
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

    let mut ret_ingredients: Vec<u64> = vec![];

    for range in ranges {
        let temp: Vec<&str> = range.split("-").collect();
        let lower_limit = temp[0].parse::<u64>().unwrap_or_default();
        let upper_limit = temp[1].parse::<u64>().unwrap_or_default();
        for i in active_ingredients.iter() {
            if lower_limit <= *i && upper_limit >= *i {
                if !ret_ingredients.contains(&i) {
                    ret_ingredients.push(*i);
                }
            }
        }
    }

    ret_val = ret_ingredients.len() as u64;

    Some(ret_val)
}

// Guesses: 387859640528018 (too high), 366181852921027 (correct)
pub fn part_two(input: &str) -> Option<u64> {
    let ret_val;
    let mut ranges = vec![];
    for line in input.lines() {
        if line == "" {
            break;
        }
        ranges.push(line);
    }

    let mut limits: Vec<Limit> = vec![];

    for r in ranges.iter().to_owned() {
        let ran: Vec<&str> = r.split("-").collect();
        let start = ran[0].parse::<u64>().unwrap_or_default();
        let end = ran[1].parse::<u64>().unwrap_or_default();
        limits.push(Limit { start, end });
    }

    limits.sort();

    let mut joined_limits = vec![];
    let mut last_limit = limits[0];
    for new_limit in limits.as_slice().iter().skip(1) {
        last_limit = if let Some(joined) = last_limit.overlap_join(new_limit) {
            joined
        } else {
            joined_limits.push(last_limit);
            *new_limit
        }
    }
    joined_limits.push(last_limit);

    ret_val = joined_limits
        .iter()
        .map(|l| l.end - l.start + 1)
        .sum::<u64>();

    Some(ret_val)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Limit {
    start: u64,
    end: u64,
}

impl Limit {
    // const fn contains(&self, value: u64) -> bool {
    //     self.start <= value && value <= self.end
    // }
    const fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
    fn overlap_join(&self, other: &Self) -> Option<Self> {
        self.overlaps(other).then(|| Self {
            start: std::cmp::min(self.start, other.start),
            end: std::cmp::max(self.end, other.end),
        })
    }
}

/*
Cases:

- Range does not overlap with any other range
- Lower end overlaps with another range
- Upper end overlaps with another range
- Entire range overlaps with another range
 */

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
