use std::io;
use std::io::Write;
use std::process;
use rand::Rng;

const MAX_FAILS: u8 = 7;

fn main() 
{
    let mut fails = 0;
    let mut flagged = [false; 26];
    let words = ["screen", "banana", "mouse", "computer", "lemon", "bruh"];
    let error = "You may only suggest letters from the alphabet.\n";
    
    let chosen_word = words[rand::thread_rng().gen_range(0, words.len())];
    println!("The word to guess is {}", chosen_word);
    loop
    {
        print_word(chosen_word, &mut flagged);
        print!("Please, guess a letter: ");
        match io::stdout().flush() {_ => ()};

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(error);

        let guess: char = match guess.trim().parse()
        {
            Ok(num) if num >= 'a' && num <= 'z' => num,
            _ => {println!("{}", error); continue;},
        };

        if update_flagged(guess, chosen_word, &mut flagged)
            {println!("Congrats! You found letter {}.\n", guess);}
        else
        {
            fails += 1;
            println!("Oops! You failed {} time(s) ({} attempt(s) left). There is no {} in that word.\n", fails, MAX_FAILS-fails, guess);
        }
        check_win(chosen_word, &mut flagged, fails);
    }
}

fn check_win(word: &str, flagged: &mut [bool], fails: u8)
{
    if fails >= MAX_FAILS
        {println!("Crap! :( You lost. The word to guess was {}.", word);}
    else
    {
        for e in word.chars()
        {
            if !flagged[(e as usize - 97) as usize]
                {return;}
        }
        println!("Congrats! :d You won. The word to guess was {}.", word);
    }
    process::exit(1);
}

fn update_flagged(guess: char, word: &str, flagged: &mut [bool]) -> bool
{
    if word.contains(guess)
    {
        flagged[(guess as usize - 97) as usize] = true;
        return true;
    }
    return false;
}

fn print_word(word: &str, flagged: &mut [bool])
{
    for e in word.chars()
    {
        if flagged[(e as usize - 97) as usize]
            {print!("{} ", e);}
        else
            {print!("_ ");}
    }
    print!("\n");
}