use fuzzy::seach_list;

fn main() {
    // finder(
    //     "llllll",
    //     vec!["hello", "kronborg", "he", "alllll", "helllo", "olleh"],
    // );
    // finder("hel", vec!["hello", "kronborg", "he", "helllo", "olleh"]);

    // let target = "olldh";
    let target = "/compose-postgres-1";
    let gg = seach_list(
        target,
        vec![
            // "he", "hello", "compose", "postgres", "dough db", "kronborg", "he", "helllo",
            "ollmh",
            "postgres", // "he", "hello", "db", "dough", "dough db", "kronborg", "he", "helllo", "olleh",
        ],
    );

    println!("target: {target}");
    for fuz in &gg {
        println!("option: {}: {}%", fuz.0, fuz.1);
    }

    println!("{gg:?}");
}
