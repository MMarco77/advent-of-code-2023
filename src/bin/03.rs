use std::collections::VecDeque;

advent_of_code::solution!(3);

fn scan_for_nbr(_vect: &[char], _idx: usize) -> u32 {
    todo!()
}

fn find_number(vect: &Vec<char>, idx: usize) -> Option<Vec<u32>> {
    let mut res: Option<Vec<u32>> = None;
    let mut neighbor: Vec<char> = vec![vect[idx]];

    //No number
    match (idx, vect.len() - 1) {
        (0, _) => neighbor.push(vect[idx + 1]),
        (i, s) if i == s => neighbor.push(vect[idx - 1]),
        (_, _) => {}
    }
    if neighbor.iter().all(|&x| x != '.') {
        return None;
    }

    let digit_count = neighbor
        .iter()
        .fold(0, |a, c| if c.is_ascii_digit() { a + 1 } else { a });

    // Find number
    match digit_count {
        0 => res = Some(vec![scan_for_nbr(vect, idx)]),
        1 => {
            match (vect[idx - 1], vect[idx], vect[idx + 1]) {
                ('.', _, _) => Some(vec![scan_for_nbr(vect, idx)]),
                (_, _, '.') => Some(vec![scan_for_nbr(vect, idx)]),
                (_, '.', _) => Some(vec![
                    scan_for_nbr(vect, idx - 1),
                    scan_for_nbr(vect, idx + 1),
                ]),
                (_, _, _) => panic!("Boom wall2!!!"),
            };
        }
        2 => {
            let v = match (vect[idx - 1], vect[idx], vect[idx + 1]) {
                ('.', '.', _) => scan_for_nbr(vect, idx + 1),
                ('.', v, '.') => v.to_digit(10).expect("Not a number"),
                (_, '.', '.') => scan_for_nbr(vect, idx - 1),
                (_, _, _) => panic!("Boom wall!!!"),
            };
            res = Some(vec![v])
        }
        _ => {}
    };

    res
}

fn get_ratio(bloc: &VecDeque<&str>, line: u32) -> u32 {
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

    for (idx, letter) in vector.iter().enumerate() {
        if letter.eq(&'*') {
            let num1 = find_number(&ref_vector, idx);
            let num2 = find_number(&vector, idx);
            let num3 = find_number(&ref_vector2, idx);
            match (num1, num2, num3) {
                (None, Some(a), Some(b)) if a.len() == 1 && b.len() == 1 => result += a[0] * b[0],
                (Some(a), None, Some(b)) if a.len() == 1 && b.len() == 1 => result += a[0] * b[0],
                (Some(a), Some(b), None) if a.len() == 1 && b.len() == 1 => result += a[0] * b[0],
                (None, None, Some(a)) if a.len() == 2 => result += a[0] * a[1],
                (None, Some(a), None) if a.len() == 2 => result += a[0] * a[1],
                (Some(a), None, None) if a.len() == 2 => result += a[0] * a[1],
                (_, _, _) => todo!(),
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
    let mut blocs: VecDeque<&str> = VecDeque::new();
    let result = input.lines().fold(0, |acc, line| match blocs.len() {
        0 => {
            blocs.push_back(line);
            acc
        }
        1 => {
            blocs.push_back(line);
            acc + get_ratio(&blocs, 0)
        }
        2 => {
            blocs.push_back(line);
            acc + get_ratio(&blocs, 1)
        }
        _ => {
            blocs.pop_front();
            blocs.push_back(line);
            acc + get_ratio(&blocs, 1)
        }
    });

    blocs.pop_front();
    Some(result + get_ratio(&blocs, 1))
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
    }
}
