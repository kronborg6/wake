pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn seach_list<'a>(target: &str, options: Vec<&'a str>) -> Vec<(&'a str, u8)> {
    options
        .iter()
        .map(|option| compare(target, option))
        .collect()
}

pub fn compare<'a>(target: &str, option: &'a str) -> (&'a str, u8) {
    if target == option {
        return (option, 100);
    }

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
            } else {
                match_score -= 5;
                one_to_one = false;
            }
        } else {
            one_to_one = false;
            match_score -= 6;
        }
    }
    if option_bytes.len() < target_len {
        match_score -=
            (u8::try_from(target_len).unwrap_or(0) - u8::try_from(option.len()).unwrap_or(0)) * 10;
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
