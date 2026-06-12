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
    let mut combo_count = 0;

    let target_bytes: &[u8] = target.as_bytes();
    let target_len = target.len();

    let option_bytes: &[u8] = option.as_bytes();
    let option_len = option.len();

    let mut match_score: u8 = 100;

    let mut multiplayer = 0.0;

    // let mut maxlen = 0;

    let _target_hmap = target_bytes.iter().fold(HashMap::new(), |mut acc, &byte| {
        *acc.entry(byte).or_insert(0) += 1;
        acc
    });

    let _option_hmap = option_bytes.iter().fold(HashMap::new(), |mut acc, &byte| {
        *acc.entry(byte).or_insert(0) += 1;
        acc
    });

    let mut match_array: Vec<bool> = Vec::new();
    for (index, value) in target_bytes.iter().enumerate() {
        if option_len > index && option_bytes[index] == *value {
            match_array.push(true);
        } else {
            match_array.push(false);
        }
    }

    let gg = match_array.iter().filter(|&x| *x).count() as f32 / target_len as f32;

    println!("{gg:?}");
    println!("{target_len:?}");
    println!("{gg:?}");

    // println!("{target_bytes:?}");
    // println!("{option_hmap:?}");
    // println!("{target_hmap:?}");
    // if target_len <= option_len {
    //     }
    //
    //     for (key, count) in &option_hmap {
    //         println!("[{key}]: {count}");
    //     }
    // } else {
    //     for (index, value) in option_bytes.iter().enumerate() {
    //         if target_bytes[index] == *value {
    //             match_array.push(true);
    //         } else {
    //             match_array.push(false);
    //         }
    //     }
    //
    //     for (key, count) in &target_hmap {
    //         println!("[{key}]: {count}");
    //     }

    for value in &match_array {
        if *value {
            if multiplayer >= 0.0 {
                multiplayer += 0.001;
                multiplayer *= multiplayer;
            }
        } else {
            if multiplayer >= 0.0 {
                // multiplayer -= 0.5;
                multiplayer -= 0.5 * multiplayer;
            }
        }
    }

    println!("{match_array:?}");
    println!("{multiplayer:?}");

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

pub fn finder(target: &str, option: &str) {
    let target_bytes: &[u8] = target.as_bytes();
    let target_len = target.len();

    let option_bytes: &[u8] = option.as_bytes();
    let option_len = option.len();

    let mut match_d2: Vec<Vec<u8>> = Vec::new();

    // need to ranger from last check not from the start agian good idea chatGPT
    let mut match_result: Vec<u8> = vec![0; option_len];
    for (index, value) in option_bytes.iter().enumerate() {
        if index < target_len && *value == target_bytes[index] {
            match_result[index] = *value;
            continue;
        }
    }

    println!("{target:?}");
    println!("{option:?}");
    println!("{match_result:?}");
    // let mut match_result: Vec<u8> = vec![0; option_len];
    // for (index, value) in option_bytes.iter().enumerate() {
    //     let mut match_result: Vec<u8> = vec![0; option_len];
    //     // let mut match_result: Vec<u8> = Vec::new();
    //     for x in target_bytes {
    //         if index < target_len && x == value {
    //             match_result[index] = *x;
    //             break;
    //         }
    //     }
    //     match_d2.push(match_result);
    // }
    //
    // println!("{target:?}");
    // println!("{option:?}");
    // // println!("{match_result:?}");
    // for res in match_d2 {
    //     println!("{res:?}");
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // finder("hewllo", "hello");
        // finder("hello", "hello");
        finder("hello", "hlieo");
        finder("hello", "hhewlo");
        finder("hello", "llll");
        finder("hello", "lellpo");
    }
}
