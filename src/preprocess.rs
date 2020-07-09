use crate::*;

pub fn preprocess_rounds(n: usize, mut string: String) -> String {
    println!("Preprocessing {} rounds...", n);

    let processors = vec![0, 1, 2];

    let mut random = rand::thread_rng();
    let mut previous = 0;

    for _ in 0..n {
        let mut choice = previous;
        while choice == previous { choice = *processors.choose(&mut random).unwrap(); }
        previous = choice;

        match choice {
            0 => { print!("qratpre: "); string = run_qratpre(string); }
            1 => { print!("qxbf:    "); string = run_qxbf(string); }
            2 => { print!("bloqqer: "); string = run_bloqqer(string); }
            _ => {},
        }

        let header = string.lines().take(1).collect::<Vec<_>>()[0];
        println!("header after preprocessing: {}", header);
    }

    string
}

pub fn run_qratpre(string: String) -> String {
    let mut file = File::create("target/tmp.qdimacs").unwrap();
    file.write_all(string.as_bytes()).unwrap();

    let output = Command::new("qratpre+")
        .arg("--print-formula")
        .arg("target/tmp.qdimacs")
        .output().unwrap();

    from_utf8(&output.stdout).unwrap().to_string()
}

pub fn run_qxbf(string: String) -> String {
    let mut file = File::create("target/tmp.qdimacs").unwrap();
    file.write_all(string.as_bytes()).unwrap();

    let output = Command::new("qxbf")
        .arg("--bcp-prop-limit=9999999")
        .arg("target/tmp.qdimacs")
        .output().unwrap();

    from_utf8(&output.stdout).unwrap().to_string()
}

pub fn run_bloqqer(string: String) -> String {
    let mut file = File::create("target/tmp.qdimacs").unwrap();
    file.write_all(string.as_bytes()).unwrap();

    let output = Command::new("bloqqer")
        .arg("target/tmp.qdimacs")
        .output().unwrap();

    from_utf8(&output.stdout).unwrap().to_string()
}
