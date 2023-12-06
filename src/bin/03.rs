use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(3);

fn compute(line: &str, line_counter: u32, xy_values: &HashMap<(u32, u32), u32>) -> u32 {
    let mut result = 0u32;

    // Build vector for scanning
    let vector: Vec<char> = line.chars().collect();
    for (idx, letter) in vector.iter().enumerate() {
        if letter.eq(&'*') {
            /*
             * | -1,-1 | 0, -1 | +1, -1 |
             * | -1,0  |   '*' | +1, 0  |
             * | -1,+1 | 0, +1 | +1, +1 |
             */
            let mut set: HashSet<_> = HashSet::new();
            let list_number_upper: Vec<&u32> = [
                xy_values.get(&((idx as u32) - 1, line_counter - 1)),
                xy_values.get(&((idx as u32), line_counter - 1)),
                xy_values.get(&((idx as u32) + 1, line_counter - 1)),
            ]
            .into_iter()
            .flatten()
            .filter(|x| set.insert(*x))
            .collect();

            let list_number = [
                xy_values.get(&((idx as u32) - 1, line_counter)),
                xy_values.get(&((idx as u32) + 1, line_counter)),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

            set.clear();
            let list_number_lower = [
                xy_values.get(&((idx as u32) - 1, line_counter + 1)),
                xy_values.get(&((idx as u32), line_counter + 1)),
                xy_values.get(&((idx as u32) + 1, line_counter + 1)),
            ]
            .into_iter()
            .flatten()
            .filter(|x| set.insert(*x))
            .collect::<Vec<_>>();

            let mut multi = 1u32;
            if (list_number_upper.len() + list_number.len() + list_number_lower.len()) == 2 {
                // println!("{:#?}", list_number);
                multi *= list_number_upper.into_iter().product::<u32>();
                multi *= list_number.into_iter().product::<u32>();
                multi *= list_number_lower.into_iter().product::<u32>();
                result += multi;
            }
        }
    }
    result
}

fn is_valid(items: Vec<char>) -> bool {
    items.iter().any(|&x| x != '.' && !x.is_ascii_digit())
}

fn find_valid_number(bloc: &VecDeque<&str>, line: u32) -> u32 {
    let mut result = 0u32;
    let max_y = bloc[0].len() - 1;

    // Build vector for scanning
    let vector: Vec<char> = bloc[line as usize].chars().collect();
    let ref_vector: Vec<char> = if line == 0 {
        bloc[1].chars().collect()
    } else {
        bloc[0].chars().collect()
    };
    let ref_vector2: Vec<char> = if bloc.len() == 3 {
        bloc[2].chars().collect()
    } else {
        vec!['.'; max_y + 1]
    };

    let mut current_number: Option<u32> = None;
    let mut valid = false;
    for (idx, letter) in vector.iter().enumerate() {
        // Meet other than number
        if !letter.is_ascii_digit() {
            if !valid {
                current_number = None;
            } else if let Some(v) = current_number {
                result += v;
                current_number = None;
                valid = false;
            }
            continue;
        }

        // Collect new number
        current_number = Some(match current_number {
            None => letter.to_digit(10).expect("Invalid digit"),
            Some(v) => v * 10 + letter.to_digit(10).expect("Invalid digit"),
        });

        // Compute validity
        if valid {
            continue;
        }
        valid = is_valid(vec![ref_vector[idx], ref_vector2[idx]])
            || idx < max_y
                && is_valid(vec![
                    ref_vector[idx + 1],
                    vector[idx + 1],
                    ref_vector2[idx + 1],
                ])
            || idx > 0
                && is_valid(vec![
                    ref_vector[idx - 1],
                    vector[idx - 1],
                    ref_vector2[idx - 1],
                ]);
    }

    //
    if let (Some(v), true) = (current_number, valid) {
        v + result
    } else {
        result
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut blocs: VecDeque<&str> = VecDeque::new();
    let result = input.lines().fold(0, |acc, line| match blocs.len() {
        0 => {
            blocs.push_back(line);
            acc
        }
        1 => {
            blocs.push_back(line);
            acc + find_valid_number(&blocs, 0)
        }
        2 => {
            blocs.push_back(line);
            acc + find_valid_number(&blocs, 1)
        }
        _ => {
            blocs.pop_front();
            blocs.push_back(line);
            acc + find_valid_number(&blocs, 1)
        }
    });

    blocs.pop_front();
    Some(result + find_valid_number(&blocs, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    // Compute coordinate of all numbers
    let mut xy_value: HashMap<(u32, u32), u32> = HashMap::new();
    let mut current_number: Option<u32> = None;
    let mut x_drift = 0u32;

    for (y, line) in input.lines().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if !letter.is_ascii_digit() {
                if let Some(v) = current_number {
                    for shift in 1..=x_drift {
                        xy_value.insert((x as u32 - shift, y as u32), v);
                    }
                }
                current_number = None;
                x_drift = 0;
                continue;
            }

            // Collect new number
            current_number = Some(match current_number {
                None => letter.to_digit(10).expect("Invalid digit"),
                Some(v) => v * 10 + letter.to_digit(10).expect("Invalid digit"),
            });
            x_drift += 1;
        }
    }

    let result = input
        .lines()
        .enumerate()
        .fold(0, |acc, (y, line)| acc + compute(line, y as u32, &xy_value));
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
        // 27743353
        // 30222892
        // 82824352
    }
}
