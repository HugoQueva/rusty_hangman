use std::io;
use std::string::String;
use rand::prelude::*;

enum WordResult {
    NotFound,
    Found,
    AlreadyFound,
}

struct Letter {
    character: char,
    found: bool,
}

const WORDS: [&str; 30] = [
    "abruptly",
    "absurd",
    "abyss",
    "affix",
    "askew",
    "avenue",
    "awkward",
    "axiom",
    "azure",
    "bagpipes",
    "bandwagon",
    "banjo",
    "giaour",
    "gnarly",
    "oxidize",
    "triphthong",
    "topaz",
    "lucky",
    "pixel",
    "galvanize",
    "jackpot",
    "duplex",
    "cycle",
    "avenue",
    "exodus",
    "kiosk",
    "kiwifruit",
    "zigzagging",
    "woozy",
    "kazoo",
];

//|----RENDERING-----|\\

fn draw_screen(live: u8, word: &Vec<Letter>) {
    let mut word_display = String::new();

    for letter in word{
        if letter.found {
            word_display.push(letter.character);
        }else{
            word_display.push('_');
        }
    }

    print!("{}[2J", 27 as char); //Go to blank page
    println!
    ("\
    ===========================\
    Lives: {live} Word: {word_display}\
    ===========================\
    ");
}

fn draw_loose_screen() {
    print!("{}[2J", 27 as char); //Go to blank page
    println!
    ("
    ===========================\
    You lost\
    ===========================\
    ");
}

fn draw_won_screen() {
    print!("{}[2J", 27 as char); //Go to blank page
    println!
    ("
    ===========================\
    You won\
    ===========================\
    ");
}

//|----WORD-----|\\

fn has_found_all_word(letters: &Vec<Letter>) -> bool{
    for letter in letters {
        if !letter.found {
            return false;
        }
    }

    true
}

fn select_word() -> &'static str {
    let mut thread_rng = thread_rng();

    let word_index = thread_rng.gen_range(0..WORDS.len());

    WORDS[word_index]
}

fn initialize_word(selected_word: &str) -> Vec<Letter> {
    let mut word = Vec::new();

    for char in selected_word.chars() {
        word.push(Letter {
            character: char,
            found: false,
        });
    }

    word
}

fn handle_user_input(stdin: &io::Stdin) -> char {
    let mut input_buffer: String = String::new();

    let _ = stdin.read_line(&mut input_buffer);

    //⚠ represent an invalid character, we don't want the program to panic.
    input_buffer.chars().next().unwrap_or('⚠')
}

fn handle_word(input: &char, word: &mut Vec<Letter>, selected_word: &str) -> WordResult {
    let mut found_letters = false;

    if selected_word.chars().any(|c| c.to_ascii_lowercase() == input.to_ascii_lowercase()) {
        for letter in word {
            if letter.character.to_ascii_lowercase() == input.to_ascii_lowercase() {
                if letter.found {
                    return WordResult::AlreadyFound;
                }

                letter.found = true;
                found_letters = true;
            }
        }
    }

    if found_letters {
        WordResult::Found
    }else{
        WordResult::NotFound
    }
}

fn main() {
    let stdin_buffer = io::stdin();
    let selected_word = select_word();
    let mut blacklisted_letters: Vec<char> = Vec::new();
    let mut word = initialize_word(&selected_word);
    let mut live: u8 = 8;

    loop {
        draw_screen(live, &word);

        let input = handle_user_input(&stdin_buffer);

        if input == '⚠' {
            continue;
        }

        if blacklisted_letters.iter().any(|c| c.to_ascii_lowercase() == input.to_ascii_lowercase()) {
            continue;
        }

        let result = handle_word(&input, &mut word, &selected_word);

        match result {
            WordResult::Found => {
                if has_found_all_word(&word) {
                    break;
                }
            },
            WordResult::NotFound => {
                live -= 1;

                if live <= 0 {
                    break;
                }
            },
            WordResult::AlreadyFound => {
                continue;
            }
        };

        blacklisted_letters.push(input);
    }

    if live <= 0 {
        draw_loose_screen();
    } else{
        draw_won_screen();
    }
}
