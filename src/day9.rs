use std::collections::HashMap;

pub fn parse_day9(lines: &[String]) -> Vec<Option<u32>> {
    if lines.len() != 1 {
        panic!("expected only one line");
    }
    let mut file_id = 0;
    let mut is_file = true;
    let mut result = Vec::new();
    for ch in lines[0].chars() {
        let size = ch
            .to_string()
            .parse::<usize>()
            .expect("input should just be integers");
        if is_file {
            for _ in 0..size {
                result.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..size {
                result.push(None);
            }
        }
        is_file = !is_file;
    }

    result
}

pub fn defrag(filesystem: &[Option<u32>]) -> Vec<Option<u32>> {
    let mut defragged: Vec<Option<u32>> = filesystem.to_vec();
    for file_id in filesystem.iter().rev() {
        if let Some(id) = file_id {
            // Set the first empty cell with this id
            if let Some(slot) = defragged.iter_mut().find(|x| x.is_none()) {
                *slot = Some(*id);
            }
            // Set the last filled cell as empty
            if let Some(slot) = defragged.iter_mut().rev().find(|x| x.is_some()) {
                *slot = None;
            }
        }
    }
    defragged
}

pub fn defrag_stage2(filesystem: &[Option<u32>]) -> Vec<Option<u32>> {
    let mut files = Vec::new(); // Vec<(file_id, start, len)
    let mut i = 0;
    while i < filesystem.len() {
        if let Some(id) = filesystem[i] {
            let start = i;
            let mut len: usize = 0;
            while i < filesystem.len() && filesystem[i] == Some(id) {
                len += 1;
                i += 1;
            }
            files.push((id, start, len));
        } else {
            i += 1;
        }
    }

    // Sort by descending file_id so we move highest IDs first
    files.sort_by(|a, b| b.0.cmp(&a.0));

    let mut defrag = filesystem.to_vec();

    for &(file_id, start, file_len) in files.iter() {
        if let Some(new_start) = find_none_run(&defrag, file_len, start) {
            for j in 0..file_len {
                // Fill the gap
                defrag[new_start + j] = Some(file_id);
                // Clear the old file blocks
                defrag[start + j] = None;
            }
        }
    }
    defrag
}

fn find_none_run(fs: &[Option<u32>], needed: usize, limit: usize) -> Option<usize> {
    let mut count = 0;
    for i in 0..limit.min(fs.len()) {
        if fs[i].is_none() {
            count += 1;
            if count == needed {
                return Some(i + 1 - needed);
            }
        } else {
            count = 0;
        }
    }
    None
}

pub fn get_checksum(filesystem: &[Option<u32>]) -> u64 {
    filesystem
        .iter()
        .enumerate()
        .filter_map(|(idx, file_id)| {
            if let Some(id) = file_id {
                Some(idx as u64 * *id as u64)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod day9 {
    use super::*;

    fn get_lines() -> Vec<String> {
        vec![String::from("2333133121414131402")]
    }

    #[test]
    fn day9_parse() {
        let filesystem = parse_day9(&get_lines());
        assert_eq!(filesystem.len(), 42);
        let expected = vec![
            Some(0),
            Some(0),
            None,
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            Some(2),
            None,
            None,
            None,
            Some(3),
            Some(3),
            Some(3),
            None,
            Some(4),
            Some(4),
            None,
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(6),
            Some(6),
            Some(6),
            Some(6),
            None,
            Some(7),
            Some(7),
            Some(7),
            None,
            Some(8),
            Some(8),
            Some(8),
            Some(8),
            Some(9),
            Some(9),
        ];
        for (result, exp) in filesystem.iter().zip(expected.iter()) {
            assert_eq!(result, exp);
        }
    }

    #[test]
    fn day9_defrag() {
        let filesystem = parse_day9(&get_lines());
        let defragged = defrag(&filesystem);
        let expected = vec![
            Some(0),
            Some(0),
            Some(9),
            Some(9),
            Some(8),
            Some(1),
            Some(1),
            Some(1),
            Some(8),
            Some(8),
            Some(8),
            Some(2),
            Some(7),
            Some(7),
            Some(7),
            Some(3),
            Some(3),
            Some(3),
            Some(6),
            Some(4),
            Some(4),
            Some(6),
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            Some(6),
            Some(6),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        assert_eq!(defragged.len(), expected.len());
        for (result, exp) in defragged.iter().zip(expected.iter()) {
            assert_eq!(result, exp);
        }
    }

    #[test]
    fn day9_checksum_stage1() {
        let fs = parse_day9(&get_lines());
        let fs2 = defrag(&fs);
        let cs = get_checksum(&fs2);
        assert_eq!(cs, 1928);
    }

    #[test]
    fn day9_defrag_stage2() {
        let fs = parse_day9(&get_lines());
        let defragged = defrag_stage2(&fs);
        let expected = vec![
            Some(0),
            Some(0),
            Some(9),
            Some(9),
            Some(2),
            Some(1),
            Some(1),
            Some(1),
            Some(7),
            Some(7),
            Some(7),
            None,
            Some(4),
            Some(4),
            None,
            Some(3),
            Some(3),
            Some(3),
            None,
            None,
            None,
            None,
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(6),
            Some(6),
            Some(6),
            Some(6),
            None,
            None,
            None,
            None,
            None,
            Some(8),
            Some(8),
            Some(8),
            Some(8),
            None,
            None,
        ];
        assert_eq!(defragged.len(), expected.len());
        for (result, exp) in defragged.iter().zip(expected.iter()) {
            assert_eq!(result, exp);
        }
    }

    #[test]
    fn day9_checksum_stage2() {
        let fs = parse_day9(&get_lines());
        let fs2 = defrag_stage2(&fs);
        let cs = get_checksum(&fs2);
        assert_eq!(cs, 2858);
    }
}
