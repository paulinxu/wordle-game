use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::collections::HashMap;

// struct GameData and Game used to store information in the required format
// for json storage

#[derive(Serialize, Deserialize, Debug)]
pub struct Game 
{
    answer: String,
    guesses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData 
{
    total_rounds: i32,
    games: Vec<Game>,
}

// reads data in a json file
pub fn read(filename: &str) -> Result<GameData> 
{
    let file = File::open(filename);
    match file 
    {
        Ok(f) => 
        {
            let reader = BufReader::new(f);
            let game_data = serde_json::from_reader(reader)?;
            return Ok(game_data)
        }
        Err(_) => panic!("read() error")
    }
}

// load data in the json file
pub fn load(filename: &str, results: &mut Vec<(crate::game::Outcome, i32)>, word_count: &mut HashMap<String, i32>, total_rounds: &mut i32)
{
    let mut file = match File::open(filename) 
    {
        Ok(file) => file,
        Err(err) => {
            print!("Failed to open file {}: {}", filename, err);
            return;
        }
    };

    // begin process to check if json file is empty
    let mut contents = String::new();

    if let Err(err) = file.read_to_string(&mut contents) 
    {
        print!("Failed to read file {}: {}", filename, err);
        return;
    }
    
    // checks that json file is not empty
    if contents.trim() == "{}" {
        return; 
    }

    let game_data: std::result::Result<GameData, _> = serde_json::from_str(&contents);

    // if the json file format is correct, the function pushes all past information
    // into results vector
    // function also uploads information about word counts
    match game_data
    {
        Ok(data) => 
        {
            *total_rounds = data.total_rounds;
    
            for game in &data.games
            {
                let total_guesses = game.guesses.len();
                if game.answer == game.guesses[total_guesses - 1]
                {
                    results.push((crate::game::Outcome::CORRECT, total_guesses as i32))
                }
                else 
                {
                    results.push((crate::game::Outcome::FAILED, total_guesses as i32))
                }
            }
            for game in &data.games
            {
                for word in &game.guesses
                {
                    let x = word_count.entry(word.clone()).or_insert(0);
                    *x += 1;
                }
            }
        }
        Err(_) => { panic!("Invalid file format") }
    }
}

// writes data into json file
pub fn write(filename: &str, game_data: &GameData) -> Result<()> 
{
    let file = File::create(filename);
    match file
    {
        Ok(f) =>
        {
            let writer = BufWriter::new(f);
            serde_json::to_writer_pretty(writer, game_data)?;
            return Ok(())
        }
        Err(_) => panic!("write() error")
    }
}

// adds individual round into data
pub fn update(filename: &str, gue: &Vec<String>, total_rounds: &mut i32, ans: String)
{
    let mut data = match read(filename) 
    {
        Ok(data) => data,
        Err(_) => GameData
        {
            games: Vec::new(),
            total_rounds: 0,
        },
    };
    
    let game: Game = Game {answer: ans, guesses: gue.clone()};
    data.games.push(game);

    data.total_rounds = *total_rounds;

    let _ = write(filename, &data);
    
}  