use std::io;

fn main() {
    let word = String::from("some string");
    let mut guessed: Vec<char> = vec![];

    let mut guess = String::new();
    let mut hidden_word = String::new();

    const MAX_GUESSES: u8 = 10;

    println!("This is your word: {}", "_".repeat(word.len()));

    for n_guesses in 0..MAX_GUESSES {
        guess.clear();
        hidden_word.clear();

        let guesses_left = MAX_GUESSES - n_guesses;
        println!("Take a guess ({guesses_left} guess{} left):", if guesses_left != 1 { "es" } else { "" });

        io::stdin().read_line(&mut guess).unwrap();
        guess.chars().next().and_then(|c| {
            guessed.push(c);
            Some(c)
        });

        let hidden_word: String = word
            .chars()
            .map(|c| if guessed.contains(&c) { c } else { '_' })
            .collect();

        let game_over = !hidden_word.contains('_');
        println!("{}", if game_over { "You win!" } else { hidden_word.as_str() });
        if game_over { break; };
    }
    
    println!("The word was: {word}");
}
