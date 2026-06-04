fn main() {
    finder("hello", vec!["kronborg", "he", "hello", "olleh"]);
}

#[derive(Debug)]
struct Res {
    name: String,
    match_project: u8,
}

// fn finder(target: &str, options: Vec<&str>) -> Result<Vec<Res>, ()> {
fn finder(target: &str, options: Vec<&str>) {
    let target_bytes: &[u8] = target.as_bytes();

    for name in options {
        let mut gg = Res {
            name: name.to_string(),
            match_project: 0,
        };
        let option_byte: &[u8] = name.as_bytes();

        for byte in option_byte {
            if target_bytes.contains(byte) {
                gg.match_project += 1;
            }
        }
        println!("fist: {:?}", gg);
    }
}
