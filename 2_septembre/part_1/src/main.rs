use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    // Specify the file's path
    let path: &Path = Path::new("src/puzzle.txt");

    // Open the file in read mode
    let file: File = File::open(&path)?;

    // Create a buffer reader to read the file line by line
    let reader: BufReader<File> = io::BufReader::new(file);

    // Initialize Regex objecs to be able to get all the number of cubes and the game id
    let re_game_id: Regex = Regex::new(r"Game (\d+)").unwrap();
    let re_cubes: Regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();

    let mut sum_of_game_ids: u32 = 0;

    // Use the method lines() to obtain an iterator on each line
    for line in reader.lines() {
        // Declare variables inside the loop to reset them for each line
        let mut game_id: u32 = 0;
        let mut is_valid_game: bool = true;

        // Manipulate each line here (line is of type Result<String, io::Error>)
        match line {
            Ok(content) => {
                // Use the captures_iter method to find all occurrences of the regular expression in the string
                for capture in re_game_id.captures_iter(&content) {
                    // Access the first capture which corresponds to the number
                    if let Some(number_match) = capture.get(1) {
                        // Extract the value of the capture
                        let number_str: &str = number_match.as_str();

                        // Convert the value to a number (u32 in this example)
                        if let Ok(number) = number_str.parse::<u32>() {
                            game_id = number;
                        } else {
                            // Handle conversion errors if necessary
                            println!("Error during number conversion");
                            is_valid_game = false;
                        }
                    }
                }

                for capture in re_cubes.captures_iter(&content) {
                    if let Some(number_match) = capture.get(1) {
                        let number_str: &str = number_match.as_str();

                        if let Ok(number) = number_str.parse::<u32>() {
                            let color = capture.get(2).unwrap().as_str();
                            let is_valid = match color {
                                "blue" => number <= 14,
                                "green" => number <= 13,
                                "red" => number <= 12,
                                _ => false,
                            };

                            if !is_valid {
                                is_valid_game = false;
                            }
                        } else {
                            println!("Error during number conversion");
                            is_valid_game = false;
                        }
                    }
                }

                if is_valid_game {
                    sum_of_game_ids += game_id;
                }
            }
            Err(err) => eprintln!("Error when reading the line : {err}"),
        }
    }

    println!("The sum of game ids that matched our criteria is {sum_of_game_ids}.");

    Ok(())
}