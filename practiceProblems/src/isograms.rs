use std::collections::HashMap;
use std::io;

fn main() {
    println!("Please enter the word you want to test for isogram: ");
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Unable to read from stdin.");
    check_isogram(word);
}

fn check_isogram(word: String) {
    let length = word.len();
    let mut frequencies = HashMap::new();
    let mut flag = false;

    for i in 0..length {
        let ch = word.chars().nth(i).unwrap();
        if frequencies.contains_key(&ch) {
            flag = true;
            break;
        }
        let prev = frequencies.get(&ch).unwrap_or(&0);
        frequencies.insert(ch, prev + 1);
    }

    if flag == true {
        println!("The word is not an isogram.");
        return;
    }
    println!("The word is an isogram.");
}
