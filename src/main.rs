use std::io::stdin;
use fastrand::usize;

fn main() {
    let secret_words: [&str; 5] = ["apple", "banana", "monkey", "house", "nepal"];
    let mut found_letter: Vec<char> = vec![];
    let i = usize(..secret_words.len());
    let secret_word = secret_words[i].to_owned();
    // println!("{}", secret_word);
    let mut try_count = 0;
    let mut user_guess = String::new();
    let mut user_guess_char: char;
    let possible_counts = secret_word.len() + 2;
    loop{
        //Giving extra tries
        try_count = try_count+1;
        if try_count > possible_counts {
            println!("Out of tries");
            break;
        }

        if secret_word.len() == found_letter.len(){
            println!("Congrats you did it!!");
            break;
        }

        user_guess = "".to_owned();
        stdin().read_line(&mut user_guess).ok().expect("Error reading the guess");
        user_guess_char = user_guess.chars().collect::<Vec<char>>()[0];

        if secret_word.contains(user_guess_char){
            found_letter.push(user_guess_char);
        }
        for j in secret_word.chars(){
            if found_letter.contains(&j){
                print!(" {} ", j);
            }else {
                print!(" _ ");
            }
        }
        println!();
    }

}
