use std::io::{stdin, stdout, Read, Write};

pub fn generate_pauses(paragraph: &str) {
    let split = paragraph.split("_");
    for line in split {
        println!("{}", line);
        pause();
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
