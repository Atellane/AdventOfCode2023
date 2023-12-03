use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    // Specify the file's path
    let path: &Path = Path::new("src/puzzle.txt");

    // Open the file in read mode
    let file: File = File::open(&path)?;

    let sum_of_game_power: u32 = part_2(file);

    println!("The sum of game power is {sum_of_game_power}.");

    Ok(())
}

fn part_2(file: File) -> u32 {
    // Create a buffer reader to read the file line by line
    let reader: BufReader<File> = io::BufReader::new(file);

    // Initialize Regex objecs to be able to get all the number of cubes
    let re_cubes: Regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();

    let mut sum_of_game_power: u32 = 0;
 

    // Use the method lines() to obtain an iterator on each line
    for line in reader.lines() {
        let mut minimal_amount_of_blue_cubes: u32 = 0;
        let mut minimal_amount_of_red_cubes: u32 = 0;
        let mut minimal_amount_of_green_cubes: u32 = 0;

        // Manipulate each line here (line is of type Result<String, io::Error>)
        match line {
            Ok(content) => {
                // Use the captures_iter method to find all occurrences of the regular expression in the string
                for capture in re_cubes.captures_iter(&content) {
                    // Access the first capture which corresponds to the number
                    if let Some(number_match) = capture.get(1) {
                        // Extract the value of the capture
                        let number_str: &str = number_match.as_str();

                        // Convert the value to a number (u32 in this example)
                        if let Ok(number) = number_str.parse::<u32>() {
                            let color = capture.get(2).unwrap().as_str();
                            match color {
                                "blue" => {
                                    if number > minimal_amount_of_blue_cubes {
                                        minimal_amount_of_blue_cubes = number;
                                    }
                                },
                                "green" => {
                                    if number > minimal_amount_of_green_cubes {
                                        minimal_amount_of_green_cubes = number;
                                    }
                                },
                                "red" => {
                                    if number > minimal_amount_of_red_cubes {
                                        minimal_amount_of_red_cubes = number;
                                    }
                                },
                                _ => eprintln!("identify a thing who's not a color as a color. color : {color}"),
                            };
                        } else {
                            // Handle conversion errors if necessary
                            println!("Error during number conversion");
                        }
                    }
                }
                let game_power: u32 = minimal_amount_of_blue_cubes * minimal_amount_of_green_cubes * minimal_amount_of_red_cubes;
                sum_of_game_power += game_power;
            }
            Err(err) => eprintln!("Error when reading the line : {err}"),
        }
    }

    return sum_of_game_power;
}