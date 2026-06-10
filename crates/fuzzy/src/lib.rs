use std::{collections::HashMap, ops::SubAssign};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn seach_list<'a>(target: &str, options: Vec<&'a str>) -> Vec<(&'a str, u8)> {
    options
        .iter()
        .map(|option| compare(target, option))
        .collect()
}

// need to change it so you get more points if a a..c can compare
// target /compose-postres-1
// option compose
// at the moment this give zero becus target is so long need to change that to
// but this need to be scroed heiger

pub fn compare_old<'a>(target: &str, option: &'a str) -> (&'a str, u8) {
    if target == option {
        return (option, 100);
    }

    let target_bytes: &[u8] = target.as_bytes();
    let target_len = target.len();
    let mut no_match = true;

    let mut match_score: u8 = 100;

    let option_bytes: &[u8] = option.as_bytes();

    for (index, value) in option_bytes.iter().enumerate() {
        if index < target_len {
            if target_bytes[index] == *value {
                match_score = match_score.saturating_add(10);
                no_match = false;
            } else if target_bytes.contains(value) {
                no_match = false;
                match_score = match_score.saturating_sub(3);
            } else {
                match_score = match_score.saturating_sub(5);
            }
        } else {
            match_score -= match_score.saturating_sub(6);
        }
    }

    // need to make this dymaci

    if option_bytes.len() < target_len {
        let multipler: f32 = 10.2 / option_bytes.len() as f32;
        // let mut multipler = option_bytes.len() / 10;

        // if multipler < 2 {
        //     multipler = 3;
        // }
        let diff_penalty = target_len.abs_diff(option.len()) as f32 * multipler;
        match_score = match_score.saturating_sub(diff_penalty as u8);
    }

    if no_match {
        match_score = 0;
    }

    (option, match_score)
}

pub fn compare<'a>(target: &str, option: &'a str) -> (&'a str, u8) {
    if target == option {
        return (option, 100);
    }

    let mut score = 0.0;
    let mut no_match = true;

    let target_bytes: &[u8] = target.as_bytes();
    let target_len = target.len();

    let option_bytes: &[u8] = option.as_bytes();
    let option_len = option.len();

    let mut match_score: u8 = 100;

    let mut multiplayer = 0.0;

    let mut combo = true;

    // let mut maxlen = 0;

    let target_hmap = target_bytes.iter().fold(HashMap::new(), |mut acc, &byte| {
        *acc.entry(byte).or_insert(0) += 1;
        acc
    });

    let option_hmap = option_bytes.iter().fold(HashMap::new(), |mut acc, &byte| {
        *acc.entry(byte).or_insert(0) += 1;
        acc
    });
    // println!("{target_bytes:?}");
    println!("{option_hmap:?}");
    println!("{target_hmap:?}");
    if target_len <= option_len {
        for (key, count) in &option_hmap {
            println!("[{key}]: {count}");
        }
    } else {
        for (key, count) in &target_hmap {
            println!("[{key}]: {count}");
        }
    }

    for (index, value) in option_bytes.iter().enumerate() {
        if target_bytes[index] == *value {
            if combo {
                multiplayer += 0.1;
            }
            combo = true;
        } else {
            combo = false;
            if multiplayer > 0.2 {
                score *= multiplayer;
            }

            // if
        }
    }

    // need to make this dymaci

    if option_bytes.len() < target_len {
        let multipler: f32 = 10.2 / option_bytes.len() as f32;
        // let mut multipler = option_bytes.len() / 10;

        // if multipler < 2 {
        //     multipler = 3;
        // }
        let diff_penalty = target_len.abs_diff(option.len()) as f32 * multipler;
        match_score = match_score.saturating_sub(diff_penalty as u8);
    }

    if no_match {
        match_score = 0;
    }

    (option, match_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
