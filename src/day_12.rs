use std::{ fs, collections::VecDeque };

use itertools::Itertools;

fn get_initial_state(
    contents: String,
    set_start: bool
) -> (Vec<Vec<[u64; 2]>>, VecDeque<(usize, usize)>, (usize, usize)) {
    let mut start = VecDeque::new();
    let mut end = None;
    let map = contents
        .trim()
        .split('\n')
        .enumerate()
        .map(|(i, row)|
            row
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start.push_back((i, j));
                        [0, 0]
                    } else if c == 'E' {
                        end = Some((i, j));
                        [(('z' as u8) - ('a' as u8)) as u64, u64::MAX]
                    } else if c == 'a' {
                        if set_start {
                            [0, u64::MAX]
                        } else {
                            start.push_back((i, j));
                            [0, 0]
                        }
                    } else {
                        [((c as u8) - ('a' as u8)) as u64, u64::MAX]
                    }
                })
                .collect_vec()
        )
        .collect_vec();
    (map, start, end.expect("Start should be found"))
}

pub fn day_12a() -> u64 {
    let contents = fs::read_to_string("src/data/day12.txt").expect("Should be able to open file");
    let (mut map, mut frontier, end) = get_initial_state(contents, true);
    while map[end.0][end.1][1] == u64::MAX {
        let top = frontier
            .pop_front()
            .expect("Frontier should never become empty before solution is found");
        let curr_level = map[top.0][top.1][0];
        let step_count = map[top.0][top.1][1] + 1;
        for dir in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
        ] {
            let new_pos = ((top.0 as i32) + dir.0, (top.1 as i32) + dir.1);
            if
                0 <= new_pos.0 &&
                new_pos.0 < (map.len() as i32) &&
                0 <= new_pos.1 &&
                new_pos.1 < (map[new_pos.0 as usize].len() as i32) &&
                map[new_pos.0 as usize][new_pos.1 as usize][0] <= curr_level + 1 &&
                map[new_pos.0 as usize][new_pos.1 as usize][1] > step_count
            {
                map[new_pos.0 as usize][new_pos.1 as usize][1] = step_count;
                frontier.push_back((new_pos.0 as usize, new_pos.1 as usize));
            }
        }
    }
    map[end.0][end.1][1]
}

pub fn day_12b() -> u64 {
    let contents = fs::read_to_string("src/data/day12.txt").expect("Should be able to open file");
    let (mut map, mut frontier, end) = get_initial_state(contents, false);
    while map[end.0][end.1][1] == u64::MAX {
        let top = frontier
            .pop_front()
            .expect("Frontier should never become empty before solution is found");
        let curr_level = map[top.0][top.1][0];
        let step_count = map[top.0][top.1][1] + 1;
        for dir in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
        ] {
            let new_pos = ((top.0 as i32) + dir.0, (top.1 as i32) + dir.1);
            if
                0 <= new_pos.0 &&
                new_pos.0 < (map.len() as i32) &&
                0 <= new_pos.1 &&
                new_pos.1 < (map[new_pos.0 as usize].len() as i32) &&
                map[new_pos.0 as usize][new_pos.1 as usize][0] <= curr_level + 1 &&
                map[new_pos.0 as usize][new_pos.1 as usize][1] > step_count
            {
                map[new_pos.0 as usize][new_pos.1 as usize][1] = step_count;
                frontier.push_back((new_pos.0 as usize, new_pos.1 as usize));
            }
        }
    }
    map[end.0][end.1][1]
}
