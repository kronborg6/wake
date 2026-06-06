fn main() {
    finder("hel", vec!["hello", "kronborg", "he", "helllo", "olleh"]);
}

#[derive(Debug, Clone)]
struct Res {
    name: String,
    match_project: u8,
}

// fn finder(target: &str, options: Vec<&str>) -> Result<Vec<Res>, ()> {
fn finder(target: &str, options: Vec<&str>) {
    let target_bytes: &[u8] = target.as_bytes();
    let target_len = target_bytes.len();
    let mut one_to_one = true;

    for name in options {
        let mut result = Res {
            name: name.to_string(),
            match_project: 100,
        };
        let option_byte: &[u8] = name.as_bytes();

        for (index, value) in option_byte.iter().enumerate() {
            if index < target_bytes.len() && target_bytes[index] == *value {
            } else if target_bytes.contains(value) {
                result.match_project -= 3;
            } else {
                result.match_project -= 5;
            }
        }

        if option_byte.len() < target_len {
            result.match_project -= (u8::try_from(target_len).unwrap_or(0)
                - u8::try_from(name.len()).unwrap_or(0))
                * 10;
        }
        println!(
            "name: {0:?} match: {1:?}",
            result.name, result.match_project
        );
    }
}
