use std::io;
use rand::prelude::*;

const MAX_ATTEMPTS: u8 = 5;

fn main() {
    let guess_list = ["apple", "mango", "banana", "orange", "litchi", "pineapple"];
    let mut rng = thread_rng();

    let index = rng.gen_range(0..guess_list.len());
    let random_fruit = guess_list[index];
    println!("🙏Welcome in the World of Fruit Guess game🙏");
    println!("!! Guess the fruit name 🤔 !!");
    println!("Type 'quit' or 'exit' to stop the game.");
    println!("You have {} attempts.\n", MAX_ATTEMPTS);

    let mut input = String::new();
    let mut attempts: u8 = 0;

    loop {
        if attempts >= MAX_ATTEMPTS {
            println!("❌ Game Over! The correct fruit was: {}", random_fruit);
            break;
        }

        input.clear();
        print!("Attempt {} → ", attempts + 1);
        flush_stdout();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_selected = input.trim().to_lowercase();

                if fruit_selected == "quit" || fruit_selected == "exit" {
                    println!("👋 Game exited by user.");
                    break;
                }

                if !guess_list.contains(&fruit_selected.as_str()) {
                    println!("This fruit is not in the guess list 🙂‍↔️");
                    continue;
                }

                attempts += 1;

                if guess_checker(fruit_selected.as_str(), random_fruit) {
                    println!("🎉 You are winner! Guessed in {} attempts.", attempts);
                    break;
                } else {
                    println!("❌ Wrong guess, try again 😊\n");
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

fn guess_checker(fruit_selected: &str, random_fruit: &str) -> bool {
    fruit_selected == random_fruit
}

fn flush_stdout() {
    use std::io::Write;
    io::stdout().flush().unwrap();
}
