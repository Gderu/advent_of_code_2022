use std::{ fs, collections::HashSet };

pub fn day_3a() -> u64 {
    let contents = fs::read_to_string("src/data/day3.txt").expect("Should be able to open file");
    contents
        .split('\n')
        .map(|line| {
            if line.len() == 0 {
                return 0;
            }
            let first: HashSet<char> = line[..line.len() / 2].chars().collect();
            let second = line[line.len() / 2..].chars();
            let combined = second
                .filter(|c| first.contains(c))
                .next()
                .expect("Should be a single char that fits the criteria");
            if 'a' <= combined && combined <= 'z' {
                (combined as u64) - ('a' as u64) + 1
            } else {
                (combined as u64) - ('A' as u64) + 27
            }
        })
        .sum()
}

pub fn day_3b() -> u64 {
    let contents = fs::read_to_string("src/data/day3.txt").expect("Should be able to open file");
    let mut cur = HashSet::new();
    let mut sum = 0;
    for (i, line) in contents.split('\n').enumerate() {
        if cur.len() == 0 {
            cur.extend(line.chars());
        } else {
            cur = cur.intersection(&line.chars().collect()).cloned().collect();
        }
        if (i + 1) % 3 == 0 {
            let c = *cur.iter().next().expect("Should be a single char that fits the criteria");
            sum += if 'a' <= c && c <= 'z' {
                (c as u64) - ('a' as u64) + 1
            } else {
                (c as u64) - ('A' as u64) + 27
            };
            cur.clear();
        }
    }
    sum
}