use std::collections::BTreeMap;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn part_one(input: &str) -> Option<u32> {
    let commands: Vec<char> = input
        .lines()
        .next()
        .expect("Missing command line")
        .chars()
        .collect::<Vec<char>>();
    let mut map: BTreeMap<&str, Node> = BTreeMap::new();
    input.lines().skip(2).for_each(|l| {
        let (k, lr) = l.split_once('=').expect("Invalid node");
        let edges: Vec<&str> = lr
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();
        map.insert(
            k.trim(),
            Node {
                left: edges.first().unwrap().trim(),
                right: edges.get(1).unwrap().trim(),
            },
        );
    });

    let mut cmd_idx = 0u32;
    let mut accu = 0u32;
    let mut next_node = *map.keys().next().unwrap();
    loop {
        let cmd = commands[cmd_idx as usize];
        let node = map.get(next_node).unwrap();

        next_node = match (cmd, node.left, node.right) {
            ('L', a, _) => a,
            ('R', _, a) => a,
            _ => unreachable!("Abnormal command."),
        };

        // Modulo
        accu += 1;
        if next_node.cmp("ZZZ") == core::cmp::Ordering::Equal {
            break;
        }
        cmd_idx = (cmd_idx + 1) % commands.len() as u32;
    }

    Some(accu)
}

pub fn part_two(input: &str) -> Option<u64> {
    let commands: Vec<char> = input
        .lines()
        .next()
        .expect("Missing command line")
        .chars()
        .collect::<Vec<char>>();
    let mut map: BTreeMap<&str, Node> = BTreeMap::new();
    input.lines().skip(2).for_each(|l| {
        let (k, lr) = l.split_once('=').expect("Invalid node");
        let edges: Vec<&str> = lr
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();
        map.insert(
            k.trim(),
            Node {
                left: edges.first().unwrap().trim(),
                right: edges.get(1).unwrap().trim(),
            },
        );
    });
    
    let mut cmd_idx = 0u32;
    let mut accu = 0u64;
    let mut next_node:Vec<&str> = map.keys().clone().into_iter().filter(|&&s| s.ends_with('A')).map(|c| c.to_owned()).collect();
    // println!("{accu} {:#?}", next_node);
    loop {
        let cmd = commands[cmd_idx as usize];

        next_node = next_node.iter().map(|&c| {
            let node = map.get(c).unwrap();
            match (cmd, node.left, node.right) {
                ('L', a, _) => a,
                ('R', _, a) => a,
                _ => unreachable!("Abnormal command."),
            }
        }).collect();

        // Modulo
        accu += 1;
        if next_node.iter().all(|c| c.ends_with("Z")) {
            break;
        }
        if (accu % 10000000) == 0 {
            // println!("{accu} {:#?}", next_node);
            println!("{accu}");
        }
        cmd_idx = (cmd_idx + 1) % commands.len() as u32;
    }
    
    Some(accu)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_sub("examples", DAY, 1));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_sub("examples", DAY,2));
        assert_eq!(result, Some(6));
    }
}
