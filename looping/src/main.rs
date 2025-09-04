use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";

    let mut count = 0;

    loop {
        println!("{}", riddle);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        count += 1;

        if input.trim() == "The letter e" {
            println!("Number of trials: {}", count);
            break;
        }
    }
}
