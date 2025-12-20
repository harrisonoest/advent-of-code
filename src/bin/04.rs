advent_of_code::solution!(4);

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn parse_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> u64 {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let mut count = 0;

    for (dx, dy) in DIRECTIONS {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && nx < width && ny >= 0 && ny < height {
            if grid[ny as usize][nx as usize] {
                count += 1;
            }
        }
    }
    count
}

fn count_all_neighbors(grid: &Vec<Vec<bool>>) -> u64 {
    let mut count = 0;
    let height = grid.len();
    let width = grid[0].len();
    let mut result_grid = vec![vec![0u8; width]; height];
    const LIMIT: u64 = 4;

    // hhh debug printing
    // for x in 0..height {
    //     for y in 0..width {
    //         print!("{:?} ", grid[y][x]);
    //     }
    //     println!("");
    // }

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] {
                let temp_count = count_neighbors(&grid, x, y);
                result_grid[y][x] = temp_count as u8;
                count += (temp_count < LIMIT) as u64;
            }
        }
        // println!("{:?}", result_grid[y]);
    }

    count
}

fn count_all_neighbors_two(grid: &mut Vec<Vec<bool>>) -> (u64, bool) {
    let mut changed = false;
    let mut count: u64 = 0;
    let height = grid.len();
    let width = grid[0].len();
    const LIMIT: u64 = 4;
    let mut rolls_to_remove = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] {
                let neighbors = count_neighbors(&grid, x, y);
                if neighbors < LIMIT {
                    rolls_to_remove[y][x] = true;
                    count += 1;
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if rolls_to_remove[y][x] {
                grid[y][x] = false;
                changed = true;
            }
        }
    }

    (count, changed)
}

// Guesses: 1460 (correct)
pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);

    Some(count_all_neighbors(&grid))
}

// Guesses: 9243 (correct)
pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);
    let mut ret_val = 0;

    loop {
        let (changed_count, has_change) = count_all_neighbors_two(&mut grid);
        ret_val += changed_count;

        if !has_change {
            break;
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
