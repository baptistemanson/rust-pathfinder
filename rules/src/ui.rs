use std::io::stdout;
use std::io::Write;

pub fn pause() {
    print!("\tEnter to continue");
    stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}

pub fn log(log_line: &str) {
    println!("{}", log_line);
}
