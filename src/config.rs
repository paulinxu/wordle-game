use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Read};

#[derive(Serialize, Deserialize, Debug)] // to help organize arguments
pub struct Config 
{
    random: Option<bool>,
    difficult: Option<bool>,
    stats: Option<bool>,
    day: Option<i32>,
    seed: Option<u64>,
    final_set: Option<String>,
    acceptable_set: Option<String>,
    state: Option<String>,
    word: Option<String>,
}

fn string_to_option(s: Option<String>) -> Option<String> 
{
    s.filter(|s| !s.is_empty())
}

// loads information from config file
pub fn load(filename: &str, args: &mut crate::Arguments)
{
    let mut file = match File::open(filename)
    {
        Ok(file) => file,
        Err(err) => 
        {
            println!("Failed to open file {}: {}", filename, err);
            return;
        }
    };

    // checks that the json file is not empty
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents)
    {
        println!("Failed to read file {}: {}", filename, err);
        return;
    }
    if contents.trim() == "{}"
    {
        return;
    }

    let temp: std::result::Result<Config, _> = serde_json::from_str(&contents);

    // makes sure that info from config file does not overwrite information
    match temp 
    {
        Ok(config) => 
        {
            if let Some(word) = config.word 
            {
                if args.word == None
                {
                    args.word = string_to_option(Some(word));
                }
            }
            if let Some(difficult) = config.difficult 
            {
                args.difficult = difficult || args.difficult;
            }
            if let Some(random) = config.random 
            {
                args.random = random || args.random;
            }
            if let Some(stats) = config.stats 
            {
                args.stats = stats || args.stats;
            }
            if let Some(day) = config.day 
            {
                if args.day == None
                {
                    args.day = Some(day);
                }
            }
            if let Some(seed) = config.seed 
            {
                if args.seed == None
                {
                    args.seed = Some(seed);
                }
            }
            if let Some(final_set) = config.final_set 
            {
                if args.final_set == None
                {
                    args.final_set = string_to_option(Some(final_set));
                }
            }
            if let Some(acceptable_set) = config.acceptable_set 
            {
                if args.acceptable_set == None
                {
                    args.acceptable_set = string_to_option(Some(acceptable_set));
                }
            }
            if let Some(state) = config.state 
            {
                if args.state == None
                {
                    args.state = string_to_option(Some(state));
                }
            }
        }
        Err(err) => 
        {
            println!("Failed to parse JSON: {}", err);
        }
    }
}