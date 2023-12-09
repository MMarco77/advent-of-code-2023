advent_of_code::solution!(9);

fn get_next_number(list: Vec<i32>) -> i32 {
    let mut find_not_zero = false;
    let res: Vec<i32> = list
        .windows(2)
        .map(|windows| {
            let left = windows[0];
            let right = windows[1];
            let diff = right.checked_sub(left).unwrap();
            find_not_zero |= diff != 0;
            diff
        })
        .collect();
    if find_not_zero {
        *list.last().unwrap() + get_next_number(res)
    } else {
        *list.last().unwrap()
    }
}

fn get_prev_number(list: Vec<i32>) -> i32 {
    let mut find_not_zero = false;
    let res: Vec<i32> = list
        .windows(2)
        .map(|windows| {
            let left = windows[0];
            let right = windows[1];
            let diff = right.checked_sub(left).unwrap();
            find_not_zero |= diff != 0;
            diff
        })
        .collect();
    if find_not_zero {
        *list.first().unwrap() - get_prev_number(res)
    } else {
        *list.first().unwrap()
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                let list: Vec<i32> = line
                    .split(' ')
                    .map(|x| {
                        x.trim()
                            .parse::<i32>()
                            .unwrap_or_else(|_| panic!("Invalid data for '{x}'"))
                    })
                    .collect::<Vec<i32>>();
                get_next_number(list)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                let list: Vec<i32> = line
                    .split(' ')
                    .map(|x| {
                        x.trim()
                            .parse::<i32>()
                            .unwrap_or_else(|_| panic!("Invalid data for '{x}'"))
                    })
                    .collect::<Vec<i32>>();
                get_prev_number(list)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result: Option<i32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
