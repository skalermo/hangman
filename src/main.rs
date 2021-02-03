use std::fs;
use rand::Rng;
use std::io;


static HANGMAN_PICS: [&str; 7] = ["
        +---+
        |   |
        |
        |
        |
        |
    =========", "
        +---+
        |   |
        |   O
        |
        |
        |
    =========", "
        +---+
        |   |
        |   O
        |   |
        |
        |
    =========", "
        +----+
        |   |
        |   O
        |  /|
        |
        |
    =========", "
        +---+
        |   |
        |   O
        |  /|\\
        |
        |
    =========", "
        +---+
        |   |
        |   O
        |  /|\\
        |  /
        |
    =========", "
        +---+
        |   |
        |   O
        |  /|\\
        |  / \\
        |
    ========="];

fn print_hangman(wrong_tries: usize) {
    println!("{}", HANGMAN_PICS[wrong_tries])
}

fn print_stats(points: u32, wrong_tries: u32) {
    println!("Your points: {}", points);
    println!("You entered wrong {}/{}", wrong_tries, HANGMAN_PICS.len()-1);
}

fn print_partial_word(word_to_guess: &str, mask: &Vec<bool>) {
    print!("Word to guess: ");
    for (i, char) in word_to_guess.chars().enumerate() {
        if mask[i] {
            print!("{}", char);
        }
        else {
            print!("*");
        }
    }
    println!();

}

fn sample_word<'a>(words: &Vec<&'a str>) -> &'a str {
    let rand_index = rand::thread_rng().gen_range(0, words.len());
    &words[rand_index]
}

fn let_user_guess() -> String {
    let mut guess = "".to_string();
    println!("New guess:");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}

fn process_guess(word_to_guess: &str,
                 user_guess: &str,
                 mask: &mut Vec<bool>,
                 used_letters: &mut Vec<char>,
                 matches_overall: &mut u32,
                 wrong_tries: &mut u32) -> bool {

    let input_letter: char = user_guess.chars().next().unwrap_or('0');
    if input_letter == '\n' {
        return false;
    }
    if used_letters.iter().any(|&i| i==input_letter) {
        println!("Letter {} was already used", input_letter);
        return false;
    }
    used_letters.push(input_letter);
    let mut matches = 0;
    for (i, letter) in word_to_guess.chars().enumerate() {
        if input_letter == letter {
            mask[i] = true;
            matches += 1;
        }
    }
    *matches_overall += matches;
    if *matches_overall == word_to_guess.len() as u32 {
        return true;
    }
    else if matches == 0 {
        *wrong_tries += 1;
        if *wrong_tries == HANGMAN_PICS.len() as u32 -1 {
            return true;
        }
    }
    false
}

fn finalize_results(matches: u32, wrong_tries: u32) {
    let mut score = matches;
    if wrong_tries == HANGMAN_PICS.len() as u32 -1 {
        print_hangman(HANGMAN_PICS.len() - 1);
        println!("You lose!");
        println!("Your score: {}", score)
    } else {
        score += matches - wrong_tries;
        println!("You win!");
        println!("Your score: {}", score)
    }
}

fn main() {
    let mut matches = 0;
    let mut wrong_tries = 0;

    let words_string = fs::read_to_string("hangman_wordbank.txt").unwrap();
    let words: Vec<&str> = words_string.split(", ").collect();

    let word_to_guess: &str = sample_word(&words);
    let mut mask = vec![false; word_to_guess.len()];
    let mut used_letters = vec!['0'; 0];

    let mut running = true;
    while running {
        print_hangman(wrong_tries as usize);
        print_partial_word(&word_to_guess, &mask);
        println!("Used letters: {:?}", &used_letters[..]);
        print_stats(matches, wrong_tries);
        let user_guess = let_user_guess();
        running = !process_guess(
            word_to_guess,
            &user_guess,
            &mut mask,
            &mut used_letters,
            &mut matches,
            &mut wrong_tries
        );
    }

    finalize_results(matches, wrong_tries);
}
