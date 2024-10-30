use std::collections::HashMap;
use std::io::{self, Write};
use crate::progress;

#[derive(Debug)]
pub enum Error // used to represent errors: in this case, only INVALID is needed
{
    INVALID,
}

#[derive(Debug, PartialEq)]
pub enum Outcome // used to represent the possible outcomes of a round
{
    CORRECT,
    FAILED
}

// checks if the input satisfies requirements: 5 alphable characters, uppercase
fn valid_input(word: &String, is_final: bool, final_list: &Vec<&str>, acceptable_list: &Vec<&str>) -> bool
{
    
    if word.len() != 5 {return false;}
    for i in word.chars()
    {
        if !i.is_uppercase() {return false;}
    }
    for element in final_list
    {
        if element.to_string().to_uppercase() == *word
        {
            return true;
        }
    }

    // the following if statement is used to differenciate between final values
    // and acceptable values

    if !is_final // is_final = false then also accept values from acceptable
    {
        for element in acceptable_list
        {
            if element.to_string().to_uppercase() == *word
            {
                return true;
            }
        }
    }
    return false;
}

// gets the input from the user
fn get_input(is_final: bool, final_list: &Vec<&str>, acceptable_list: &Vec<&str>) -> Result<String, Error>
{
    let mut word = String::new();
    let _ = io::stdin().read_line(&mut word);
    let word = word.trim().to_string().to_uppercase();
    
    // If the input value does not satisfy the format or is not in its intended list,
    // then the function will return Error::INVALID to signal asking for input again

    match valid_input(&word, is_final, final_list, acceptable_list)
    {
        true => Ok(word),
        false => Err(Error::INVALID)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Status // used to represent the four colors a character can have
{
    G, Y, R, X
}

// check if a guess is correct by seeing if all its 5 characters are Status::G
fn check_correct(guess: [Status; 5]) -> bool
{
    for i in guess
    {
        if i != Status::G {return false;}
    }
    return true;
}

// prints status of the most recent guess
fn print_arr_5(arr: [Status; 5])
{
    for x in arr {print!("{:?}", x);}
}
// prints status of the entire alphabet
fn print_arr_26(arr: [Status; 26])
{
    for x in arr {print!("{:?}", x);}
}

// the compare function compares the guess and the answer and calculates
// the status of the guess and alphabet

fn compare(answer: &String, guess: String) -> ([Status; 5], [Status; 26]) 
{
    let mut result = [Status::X ; 5];
    let mut letter_status = [Status::X ; 26];

    // first, all elements in the guess status are set to red
    for (i, c) in guess.chars().enumerate()
    {
        result[i] = Status::R;
        letter_status[(c as usize) - ('A' as usize)] = Status::R;
    }
    // then, all the correct characters are set to green
    for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() 
    {
        if a == g
        {
            result[i] = Status::G;
            letter_status[(g as usize) - ('A' as usize)] = Status::G;
        }
    }
    // then, I use a HashMap to keep track of how freequently each character appears
    // this is used later to see which values should be yellow and which
    // should be red (those characters that already have a green status are ignored
    // because the net change in count is 0)

    let mut letter_count: HashMap<char, i32> = HashMap::new();
    for (i, ch) in answer.chars().enumerate() 
    {
        let x = letter_count.entry(ch).or_insert(0);
        if result[i] != Status::G
        {
            *x += 1;
        }
    }

    // finally, characters are assigned as yellow depending on how many more/less
    // times that character appears in the guess compared to the answer
    for (i, (_a, g)) in answer.chars().zip(guess.chars()).enumerate() 
    {
        if result[i] != Status::G
        {
            if let Some(x) = letter_count.get_mut(&g) {
                if *x > 0
                {
                    result[i] = Status::Y;
                    if letter_status[(g as usize) - ('A' as usize)] != Status::G
                    {
                        letter_status[(g as usize) - ('A' as usize)] = Status::Y;
                    }
                    *x -= 1;
                }
            }
        }
    }
    
    return (result, letter_status);
}

// function used to update information that each new guess provides to the alphabet
// it adds new information on top of the old information using priority: G>Y>R>X
fn merge(mut s1: [Status; 26], s2: [Status; 26]) -> [Status; 26]
{
    for i in 0..26
    {
        if s1[i] == Status::G || s2[i] == Status::G {s1[i] = Status::G ; continue;}
        else if s1[i] == Status::Y || s2[i] == Status::Y {s1[i] = Status::Y ; continue;}
        else if s1[i] == Status::R || s2[i] == Status::R {s1[i] = Status::R ; continue;}
        else {continue;}
    }
    return s1
}

// prints test-friendly result
fn print_no_tty(a: [Status; 5], b:[Status; 26])
{
    print_arr_5(a);
    print!(" ");
    print_arr_26(b);
    print!("\n");
}

// prints user-friendly characters
fn get_display(c: char, status: Status) -> console::StyledObject<char>
{
    if status == Status::G // each character is printed in the color of their status
    {
        return console::style(c).bold().blink().green();
    }
    else if status == Status::Y
    {
        return console::style(c).yellow();
    }
    else if status == Status::R
    {
        return console::style(c).red();
    }
    else {
        return console::style(c).white();
    }
}

// prints user-friendly result
fn print_tty(a: [Status; 5], b:[Status; 26], word: String)
{
    for (i, e) in word.chars().enumerate() 
    {
        let styled = get_display(e, a[i]);
        print!("{}", styled);
    }
    print!(" ");
    for i in 0..26
    {
        let styled = get_display((('A' as u8) + i) as char, b[i as usize]);
        print!("{}", styled);
    }
    print!("\n\n");
}

// keeps track of which characters have already been confirmed as green
// used in difficult mode
fn update_known_greens(mut greens: [Status; 5], p1: [Status; 5]) -> [Status; 5]
{
    for i in 0..5
    {
        if p1[i] == Status::G {greens[i] = Status::G;}
    }
    return greens;
}
// additional check for input values in the difficult mode
fn check_valid_difficult(guess: &String, answer: &String, greens: [Status; 5], alphabet: [Status; 26]) -> bool
{
    for i in 0..5 // checks that all green characters are fixed
    {
        if greens[i] == Status::G && guess.chars().nth(i) != answer.chars().nth(i)
        {
            return false;
        }
    }
    for i in 0..26 // checks that all yellow charecters appear
    {
        if alphabet[i] == Status::Y && !guess.contains((('A' as usize) + i) as u8 as char)
        {
            return false
        }
    }
    return true;
}

fn check_valid_hint(status: [Status; 5], e: &str, word: String) -> bool
{
    let (e_status, _) = compare(&e.to_string(), word);
    if status == e_status
    {
        return true;
    }
    return false
}


// // check if a word is a possible solution
// fn check_valid_hint(status: [Status; 5], e: &str, word: String) -> bool
// {
//     for (i, c) in word.chars().enumerate()
//     {
//         // ensures that the green characters are matching between word and correct answer
//         if status[i] == Status::G 
//         {
//             if c != e.chars().nth(i).unwrap()
//             {
                
//                 return false;
//             }
//         }
//         // ensures that yellow character are included in the word
//         if status[i] == Status::Y
//         {
//             if !e.contains(c)
//             {
//                 return false;
//             }
//             // ensures that a yellow character is not in that same position again
//             if c == e.chars().nth(i).unwrap()
//             {
//                 return false;
//             }
//         }
//         // ensures that a red character is not in that same position again
//         if status[i] == Status::R
//         {
//             if c == e.chars().nth(i).unwrap()
//             {
//                 return false;
//             }
//         }
//     }

//     // frequency of character in solution >= yellow + green
//     let mut letter_count: HashMap<char, i32> = HashMap::new(); 
//     for (i, ch) in word.clone().chars().enumerate() 
//     {
//         if status[i] == Status::Y || status[i] == Status::G
//         {
//             let x = letter_count.entry(ch).or_insert(0);
//             *x += 1;
//         }
//     }
//     // frequency of each character character in current string
//     let mut letter_count_2: HashMap<char, i32> = HashMap::new();
//     for (i, ch) in e.clone().chars().enumerate() 
//     {
//         let x = letter_count_2.entry(ch).or_insert(0);
//         *x += 1;
//     }
//     for (i, c) in word.clone().chars().enumerate()
//     {
//         if status[i] == Status::Y || status[i] == Status::G
//         {
//             // less appearences in current string than in the solution
//             if *letter_count.get_mut(&c).unwrap() > *letter_count_2.get_mut(&c).unwrap()
//             {
//                 return false;
//             }
//         }
//     }
    
//     return true;
// }

// game function starts a new wordle round
fn round(is_tty: bool, answer: &String, final_list: &Vec<&str>, acceptable_list: &Vec<&str>, is_difficult: bool,
     word_count: &mut HashMap<String, i32>, is_hint: bool) 
-> Result<(Outcome, i32, Vec<String>), Error> // Result<(correct/failed, #of tries)>
{
    
    // storing information about the current round
    let mut count: i32 = 0;
    let mut alphabet = [Status::X ; 26];
    let mut greens = [Status::X ; 5];
    let mut guesses: Vec<String> = Vec::new();
    let mut possible: Vec<&str> = Vec::new();

    if is_hint // used for hint mode
    {
        possible = acceptable_list.clone();
    }

    while count < 6 // count to keep track of how many guesses used
    {
        count += 1;
        let word: String;
        loop // loop used to get a valid input from user
        {
            if is_tty // all "if is_tty" are used for user friendly output 
            {
                println!("{}", console::style("Enter a guess: ").blue());
            }
             
            let guess: Result<String, Error> = get_input(false, &final_list, &acceptable_list);

            // if the input is not valid, then the user is asked for input again
            match guess 
            {
                Ok(x) => {
                    // DIFFICULT MODE START
                    if is_difficult && !check_valid_difficult(&x, &answer.clone(), greens, alphabet)
                    {
                        println!("{:?}", Error::INVALID);
                        continue;
                    }
                    // DIFFICULT MODE END

                    word = x;
                    break; 
                }
                Err(e) =>  {println!("{:?}", e);}
            }
        }

        // if the user guess is valid, then calculations begin now:

        guesses.push(word.clone()); // keeps track of all guesses

        // obtains results for printing
        let (p1, p2) = compare(&answer.clone(), word.clone());

        // updates alphabet information
        alphabet = merge(alphabet, p2);

        if is_tty
        {
            print_tty(p1, alphabet, word.clone());
        }
        else {
            print_no_tty(p1, alphabet);
        }

        // finds all words that are still possible solutions based on new result
        if is_hint
        {
            print!("Hint: possible words: \n");
            let mut temp: Vec<&str> = Vec::new();
            for e in &possible
            {
                if check_valid_hint(p1, &e.to_uppercase(), word.clone())
                {
                    temp.push(e);
                }
            }
            possible = temp;
            print!("{:?} \n", possible);
        }

        // DIFFICULT MODE START
        if is_difficult
        {
            greens = update_known_greens(greens, p1);
        }
        // DIFFICULT MODE END

        // STATS MODE START
        let x = word_count.entry(word.clone()).or_insert(0);
        *x += 1;
        // STATS MODE END

        if check_correct(p1) 
        { 
            return Ok((Outcome::CORRECT, count, guesses));
        }
    }
    
    return Ok((Outcome::FAILED, 6, guesses))
}

// asks user whether to continue to the next round
fn ask_continue() -> bool {
    loop {
        let mut input = String::new();

        io::stdout().flush().expect("Failed flush");

        let is_eof = io::stdin().read_line(&mut input).expect("Failed input")== 0;

        let input = input.trim().to_string();

        if input == "N" || is_eof {
            return false;
        } else if input == "Y" {
            return true;
        } else {
            println!("Invalid input. Please enter 'Y' or 'N'.");
        }
    }
}

// calculates prints the statistics in the mode --stats
fn print_stats(results: &Vec<(Outcome, i32)>, word_count: &HashMap<String, i32>, is_tty: bool)
{   
    // calculates wins, losses, and average guesses in wins
    let mut x = 0;
    let mut y = 0;
    let mut total_attempts: f32 = 0.0;
    for (outcome, attempts) in results
    {
        if *outcome == Outcome::CORRECT
        {
            x += 1;
            total_attempts += *attempts as f32;
        }
        else 
        {
            y += 1;
        }
    }
    let z: f32;
    if x == 0 { z = 0.0;}
    else { z = total_attempts/(x as f32); }

    // for user friendly version
    if is_tty
    {
        println!("\n{}\n", console::style("Statistics:").bold().blink().cyan());
        println!("{} {}", console::style("Wins:").cyan(), console::style(x).cyan());
        println!("{} {}", console::style("Losses:").cyan(), console::style(y).cyan());
        println!("{} {}", console::style("Avg. tries:").cyan(), console::style(z).cyan());
    }
    else {
        print!("{} {} {:.2}\n", x, y, z);
    }

    // calculates top 5 most frequently used words and their frequency
    let mut sorted_word_count: Vec<(&String, &i32)> = word_count.iter().collect();
    
    sorted_word_count.sort_by(|a, b| 
    {
        b.1.cmp(a.1).then_with(|| a.0.cmp(b.0))
    });

    if is_tty
    {
        println!("\n{}\n", console::style("Frequently used words:").bold().blink().cyan());
    }

    for i in 0..sorted_word_count.len()
    {
        if is_tty
        {
            println!("{} {} {} {}", console::style(sorted_word_count[i].0).cyan(),console::style("used"), 
            console::style(sorted_word_count[i].1).cyan(), console::style("time/s"));
        }
        else 
        {
            print!("{} {}", sorted_word_count[i].0, sorted_word_count[i].1);
            if i == 4 || i == sorted_word_count.len()-1
            {
                break;
            }
            print!(" ");
        }
    }
    print!("\n");
}


// function starts the actual game
pub fn start(is_tty: bool, final_list: &Vec<&str>, acceptable_list: &Vec<&str>, mut word_arg: Option<String>,
     is_difficult: bool, show_stats: bool, is_random: bool, mut day: i32, record_progress: bool, progress_file: String, is_hint: bool)
{
    
    if is_tty
    {
        println!("{}", console::style("Welcome to WORDLE!").bold().blink().blue());
    }

    // stores the results from each round
    let mut results: Vec<(Outcome, i32)> = Vec::new();
    let mut word_count: HashMap<String, i32> = HashMap::new();
    let mut total_rounds: i32 = 0;

    // READ PROGRESS FILE

    if record_progress
    {
        // print!("RUNNING LOAD()\n");
        progress::load(&progress_file, &mut results, &mut word_count, &mut total_rounds);
    }

    // END READ PROGRESS FILE

    loop // loop to ask if to play again
    {

        loop // loop to check for valid inut
        { 
            let mut word: Result<String, Error> = Err(Error::INVALID);
            if !is_random
            {
                if let Some(ref mut x) = word_arg
                {
                    word = Ok(x.to_string());
                }
                else 
                {
                    if is_tty
                    {
                        println!("{}", console::style("Enter the solution: ").blue());
                    }
                    word = get_input(true, &final_list, &acceptable_list);
                }
            }
            else
            {
                for (i, &element) in final_list.iter().enumerate() {
                    if (i == (day-1) as usize)
                    {
                        word = Ok(element.clone().to_string());
                        break;
                    }
                }
                day += 1; // for each round played, day count increases
            }
            
            let result: Result<(Outcome, i32, Vec<String>), Error>;
            // if the indicated word is not valid, then the used is asked for input again
            match word
            {
                Ok(mut x) => 
                {
                    x = x.to_uppercase();
                    result = round(is_tty, &x, final_list, acceptable_list, is_difficult, &mut word_count, is_hint);
                    match result
                    {
                        Ok((outcome, count, guesses)) => 
                        {
                            // round is finished, printing different types of outcomes
                            if outcome == Outcome::CORRECT
                            {
                                if is_tty
                                {
                                    println!("{} {} {}", 
                                    console::style("You won in").blue(), 
                                    console::style(count).bold().blink().blue(), 
                                    console::style("tries").blue());
                                }
                                else {
                                    println!("{:?} {}", outcome, count);
                                }
                            }
                            else
                            {
                                if is_tty
                                {
                                    println!("{} {}", 
                                    console::style("You lost. Correct answer: ").blue(), 
                                    console::style(count).bold().blink().blue()
                                    )
                                }
                                else {
                                    println!("{:?} {}", outcome, x);
                                }
                            }

                            results.push((outcome, count));
                            total_rounds += 1;

                            // WRITE TO JSON PROGRESS FILE

                            if record_progress
                            {
                                progress::update(&progress_file, &guesses , &mut total_rounds , x);
                            }

                            // END WRITE TO JSON PROGRESS FILE

                            break;
                        }
                        Err(e) => {println!("{:?}", e);}
                    }
                }
                Err(e) => {println!("{:?}", e);}
            }
        }

        // STATS MODE START
        if show_stats
        {
            print_stats(&results, &word_count, is_tty);
        }
        // STATS MODE END


        if is_tty
        {
            println!("{}", console::style("Type 'Y' if you wish to continue and 'N' if you wish to quit\n").blue());
        }

        // asks if the user wants to continue plauing
        if !ask_continue() 
        {
            if is_tty
            {
                println!("{}", console::style("Thank you for playing!\n").blue());
            }
            return;
        }
    }
}