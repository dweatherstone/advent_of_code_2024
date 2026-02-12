pub type Key = Vec<u8>;
pub type Lock = Vec<u8>;

pub fn parse_day25(lines: &[String]) -> (Vec<Key>, Vec<Lock>) {
    let mut keys: Vec<Key> = Vec::new();
    let mut locks: Vec<Lock> = Vec::new();
    for chunk in lines.chunks(8) {
        let text = chunk.join("\n");
        match text.trim().chars().next().unwrap() {
            '.' => keys.push(keylock_from_text(&text)),
            '#' => locks.push(keylock_from_text(&text)),
            _ => panic!("unknown character"),
        }
    }

    (keys, locks)
}

pub fn result_day25_stage1(keys: &[Key], locks: &[Lock]) -> usize {
    let mut sum = 0;
    for key in keys {
        for lock in locks {
            if key_fits_lock(key, lock) {
                sum += 1;
            }
        }
    }

    sum
}

fn keylock_from_text(text: &str) -> Vec<u8> {
    let mut heights: Vec<u8> = vec![0; 5];
    for line in text.trim().lines() {
        for (i, pin) in line.char_indices() {
            if pin == '#' {
                heights[i] += 1;
            }
        }
    }
    heights.iter().map(|h| *h - 1).collect()
}

fn key_fits_lock(key: &Key, lock: &Lock) -> bool {
    for (k, l) in key.iter().zip(lock.iter()) {
        if k + l > 5 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod day25 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("#####"),
            String::from(".####"),
            String::from(".####"),
            String::from(".####"),
            String::from(".#.#."),
            String::from(".#..."),
            String::from("....."),
            String::from(""),
            String::from("#####"),
            String::from("##.##"),
            String::from(".#.##"),
            String::from("...##"),
            String::from("...#."),
            String::from("...#."),
            String::from("....."),
            String::from(""),
            String::from("....."),
            String::from("#...."),
            String::from("#...."),
            String::from("#...#"),
            String::from("#.#.#"),
            String::from("#.###"),
            String::from("#####"),
            String::from(""),
            String::from("....."),
            String::from("....."),
            String::from("#.#.."),
            String::from("###.."),
            String::from("###.#"),
            String::from("###.#"),
            String::from("#####"),
            String::from(""),
            String::from("....."),
            String::from("....."),
            String::from("....."),
            String::from("#...."),
            String::from("#.#.."),
            String::from("#.#.#"),
            String::from("#####"),
        ]
    }

    #[test]
    fn parse() {
        let (keys, locks) = parse_day25(&get_example());
        let expected_keys = vec![
            vec![5, 0, 2, 1, 3],
            vec![4, 3, 4, 0, 2],
            vec![3, 0, 2, 0, 1],
        ];
        let expected_locks = vec![vec![0, 5, 3, 4, 3], vec![1, 2, 0, 5, 3]];
        assert_eq!(keys, expected_keys, "Keys are wrong");
        assert_eq!(locks, expected_locks, "Locks are wrong");
    }

    #[test]
    fn stage1() {
        let (keys, locks) = parse_day25(&get_example());
        let result = result_day25_stage1(&keys, &locks);
        assert_eq!(result, 3);
    }
}
