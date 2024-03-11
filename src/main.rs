use colored::Colorize;
use rand::seq::SliceRandom;
use std::fs::read;
use std::io;

fn get_words() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut words: Vec<String> = Vec::new();
    let words_file = String::from_utf8(read("words.txt")?)?;
    for word in words_file.split("\n") {
        words.push(word.to_string());
    }

    Ok(words)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let words = get_words()?;
    let word = words.choose(&mut rand::thread_rng()).unwrap();

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("couldnt read stdin.");

        user_input = user_input.replace("\n", "");

        if user_input.len() != 5 {
            println!("Word must be 5 letters long.");
            continue;
        }
        if words.contains(&user_input) == false {
            println!("That word is not in the word list.");
            continue;
        }

        for i in 0..word.len() {
            let word_letter = word[i..=i].to_string();
            let user_letter = user_input[i..=i].to_string();

            if user_letter == word_letter {
                print!("{}", user_letter.green());
            } else if word.contains(user_letter.as_str()) {
                print!("{}", user_letter.yellow());
            } else {
                print!("{}", user_letter);
            }
        }

        print!("\n");

        if &user_input == word {
            break;
        }
    }
    Ok(())
}
