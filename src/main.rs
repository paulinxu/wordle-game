mod game;
mod builtin_words;
mod arguments;
mod readfilemode;
mod progress;
mod config;
use clap::Parser;
use rand::rngs::StdRng;
use rand::prelude::*;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct Arguments // help organize arguments
{
    word: Option<String>,
    random: bool,
    difficult: bool, 
    stats: bool,
    seed: Option<u64>,
    day: Option<i32>,
    final_set: Option<String>,
    acceptable_set: Option<String>,
    state: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> 
{    
    let cli = arguments::Cli::parse();

    let mut args = Arguments
    {
        word : cli.word, 
        random: cli.random,
        difficult: cli.difficult, 
        stats: cli.stats,
        seed: cli.seed,
        day: cli.day,
        final_set: cli.final_set,
        acceptable_set: cli.acceptable_set,
        state: cli.state,
    };

    // HANDLE CONFIG

    // If config file is provided then it will load the arguments that have not been 
    // provided earlier

    if let Some(x) = cli.config
    {
        config::load(&x, &mut args);
    }
    
    // END HANDLE CONFIG


    // HANDLE CONFLICTS
    // Program will exit if there are conflicting argument uses
    if args.random
    {
        if let Some(_x) = args.word.clone()
        {
            return Err("Cannot use -w in random mode".into());
        }
    }
    else
    {
        if let Some(_x) = args.seed.clone()
        {
            return Err("Cannot use -s in non-random mode".into());
        }
        if let Some(_x) = args.day.clone()
        {
            return Err("Cannot use -d in non-random mode".into());
        }
    }
    // END HANDLE CONFLICTS

    // HANDLE EXTERNAL FILES

    // switching to vector for easier manipulation
    let mut final_list: Vec<&str> = builtin_words::FINAL.to_vec(); 
    let mut acceptable_list: Vec<&str> = builtin_words::ACCEPTABLE.to_vec();

    // checks if non-default final_set has been provided
    let mut str_f: String = String::new();
    if let Some(filename) = args.final_set
    {
        let mut f = File::open(filename)?;
        f.read_to_string(&mut str_f)?;

        let new_final_list: Vec<&str> = str_f.lines().collect();

        // checking if the new list satisfies requirements
        // if it is, then the original list is replaced with the new one

        if readfilemode::check_valid_list(&new_final_list, &final_list)
        {
            final_list = new_final_list;
        }
        else {
            return Err("Invalid final-set".into());
        }
        final_list.sort();
    }

    // checks if non-default acceptable_set has been provided
    let mut str_a = String::new();
    if let Some(filename) = args.acceptable_set
    {
        let mut f = File::open(filename)?;
        f.read_to_string(&mut str_a)?;

        let new_acceptable_list: Vec<&str> = str_a.lines().collect();

        // checking if the new list satisfies requirements
        // if it is, then the original list is replaced with the new one

        if readfilemode::check_valid_list(&new_acceptable_list, &acceptable_list)
        {
            acceptable_list = new_acceptable_list;
        }
        else {
            return Err("Invalid acceptable-set".into());
        }
        acceptable_list.sort();
    }

    // END HANDLE EXTERNAL FILES

    // HANDLE PROGESS IN JSON

    // Checks if state file exists
    // if it does, then the filename is recorded and the start() function is 
    // signaled using a boolean value

    let mut record_progress = false;
    let mut filename: String = "".to_string();

    if let Some(x) = args.state
    {
        filename = x;
        // print!("filename: {}\n", filename);
        record_progress = true;
    }

    // END HANDLE PROGESS IN JSON

    // HANDLE RANDOM

    // Obtains seed and day data

    let seed = args.seed.unwrap_or(1);
    let day = args.day.unwrap_or(1);

    // if -r option selected, then the lists will be shuffled using the provided seed
    if args.random  
    {
        let mut rng = StdRng::seed_from_u64(seed);
        final_list.shuffle(&mut rng);
        acceptable_list.shuffle(&mut rng);
    }

    // END HANDLE RANDOM

    // determines which output version: 
    // test friendly for is_tty=false and user friendly for is_tty=true
    let is_tty = atty::is(atty::Stream::Stdout); 
    
    // essential information is passed into the game
    game::start(is_tty, &final_list, &acceptable_list, args.word,
        args.difficult, args.stats, args.random, day,
        record_progress, filename, cli.hint);

    Ok(())
}
