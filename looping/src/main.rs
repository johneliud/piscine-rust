use std::io;

fn main() {
    let mut answer = String::new();
    let mut tries = 0;

    loop {
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );

        answer.clear();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read answer");
        let trimmed_answer = answer.trim();

        tries += 1;

        if trimmed_answer == "e" {
            println!("You are correct! It took you {} tries.", tries);
            break;
        } else {
            println!("You are incorrect!");
        }
    }
}

