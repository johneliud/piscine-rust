use std::io;

fn main() {
    let mut answer = String::new();

    riddle(answer);
}

fn riddle(answer: String::new()) {
    println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

    loop {
        io::stdin().read_line(&mut String).expect("Failed to read answer");

        if answer == "e" {
            break;
        }
    }
}