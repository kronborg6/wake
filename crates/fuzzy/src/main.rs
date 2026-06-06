fn main() {
    // finder(
    //     "llllll",
    //     vec!["hello", "kronborg", "he", "alllll", "helllo", "olleh"],
    // );
    // finder("hel", vec!["hello", "kronborg", "he", "helllo", "olleh"]);
    let gg = seach_list("hel", vec!["hello", "kronborg", "he", "helllo", "olleh"]);

    println!("{:?}", gg);
}

// #[derive(Debug, Clone)]
// struct Res {
//     name: String,
//     match_project: u8,
// }

// fn seach_list<'a>(target: &str, options: Vec<&'a str>) -> Result<Vec<(&'a str, u8)>, ()> {
// fn finder(target: &str, options: Vec<&str>) {
//     let target_bytes: &[u8] = target.as_bytes();
//     let target_len = target_bytes.len();
//     let mut one_to_one = true;
//
//     for name in options {
//         let mut result = Res {
//             name: name.to_string(),
//             match_project: 100,
//         };
//         let option_byte: &[u8] = name.as_bytes();
//
//         for (index, value) in option_byte.iter().enumerate() {
//             if index < target_bytes.len() && target_bytes[index] == *value {
//             } else if target_bytes.contains(value) {
//                 result.match_project -= 3;
//             } else {
//                 result.match_project -= 5;
//             }
//         }
//
//         if option_byte.len() < target_len {
//             result.match_project -= (u8::try_from(target_len).unwrap_or(0)
//                 - u8::try_from(name.len()).unwrap_or(0))
//                 * 10;
//         }
//         println!(
//             "name: {0:?} match: {1:?}",
//             result.name, result.match_project
//         );
//     }
// }

// fn seach_list<'a>(target: &str, options: Vec<&'a str>) -> Result<Vec<(&'a str, u8)>, ()> {
fn seach_list<'a>(target: &str, options: Vec<&'a str>) -> Vec<(&'a str, u8)> {
    options
        .iter()
        .map(|option| compare(target, option))
        .collect()
}

pub fn compare<'a>(target: &str, option: &'a str) -> (&'a str, u8) {
    let target_bytes: &[u8] = target.as_bytes();
    let target_len = target.len();
    let mut one_to_one = true;
    let mut no_match = true;

    let mut match_score = 100;

    let option_bytes: &[u8] = option.as_bytes();

    for (index, value) in option_bytes.iter().enumerate() {
        if index < target_len {
            if target_bytes[index] == *value {
                no_match = false;
            } else if target_bytes.contains(value) {
                one_to_one = false;
                no_match = false;
                match_score -= 3;
            }
        } else {
            one_to_one = false;
            match_score -= 6;
        }
    }

    if no_match {
        match_score = 0;
    }
    if one_to_one {
        match_score = 100;
    }

    (option, match_score)
}

// need to add a match count
// change scroing system it need to check postin to
