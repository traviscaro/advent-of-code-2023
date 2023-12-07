use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part_1();
    part_2();
}

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

struct Set {
    cubes: Vec<Cube>,
}

struct Cube {
    color: String,
    number: u32,
}

/**
--- Day 2: Cube Conundrum ---
You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
*/
fn part_1() {
    // There are 4 structs: Bag, Game, Set, and Cube.
    // The Bag struct will contain the number of cubes of each color.
    // The Game struct will contain the game ID and a vector of Sets.
    // The Set struct will contain a vector of Cubes.
    // The Cube struct will contain the number of cubes and the color of the cubes.

    // Loop through lines of file. Each line represents a Game.
    // The line will start with "Game <number>: " and then be followed by a list of cubes.
    // The cubes will be in the format "<number> <color>".
    // The objective is to sum the game IDs of the games that would be possible with the given cubes.
    // The game ID is the number after "Game " in the line.
    // The game is possible if the number of cubes of each color is less than or equal to the number of cubes of that color in the bag.
    // The bag contains 12 red cubes, 13 green cubes, and 14 blue cubes.
    // The game is impossible if the number of cubes of any color is greater than the number of cubes of that color in the bag.
    // I will loop through each line, parse the game ID.
    // Then I will loop through the sets of cubes in the games.
    // Each set is separated by a semicolon.
    // Each set will include a number of cubes and the color of the cubes.

    // Then I will write a function that returns a list of possible game IDs.
    // The function will take a Game and a Bag.
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut possible_game_ids: Vec<u32> = Vec::new();

    let lines = load_file("input.txt");
    for line in lines {
        let line = line.unwrap();
        let game_id = parse_game_id(&line);
        let sets = parse_sets(&line);
        let game = Game { id: game_id, sets };

        if is_game_possible(&game, &bag) {
            possible_game_ids.push(game.id);
        }
    }

    // Sum the possible game IDs.
    let sum: u32 = possible_game_ids.iter().sum();

    // Print the sum.
    println!("PART 1 ANSWER: {}", sum);
}

/**
--- Part Two ---
The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
Game 4 required at least 14 red, 3 green, and 15 blue cubes.
Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
 */
fn part_2() {
    // Print the sum.
    println!("PART 2 ANSWER");
}

fn load_file(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn parse_game_id(line: &str) -> u32 {
    // Find the index of "Game " and ":"
    let game_str = "Game ";
    let start = line.find(game_str).unwrap();
    let end = line.find(":").unwrap();

    // Return the substring between "Game " and ":"
    return line[(start + game_str.len())..end].parse::<u32>().unwrap();
}

fn parse_sets(line: &str) -> Vec<Set> {
    let prefix_str = ": ";
    let game_prefix_start = line.find(prefix_str).unwrap();
    let set_input_str = &line[(game_prefix_start + prefix_str.len())..line.len()];

    // Split the line by "; "
    let set_strs = set_input_str.split("; ");

    // Loop through the sets
    let mut sets: Vec<Set> = Vec::new();
    for set_str in set_strs {
        let cubes = parse_cubes(set_str);
        let set = Set { cubes };
        sets.push(set);
    }

    return sets;
}

fn parse_cubes(set_str: &str) -> Vec<Cube> {
    // Split the set by ", "
    let cube_strs = set_str.split(", ");

    // Loop through the cubes
    let mut cubes: Vec<Cube> = Vec::new();
    for cube_str in cube_strs {
        let cube = parse_cube(cube_str);
        cubes.push(cube);
    }

    return cubes;
}

fn parse_cube(cube_str: &str) -> Cube {
    // Split the cube by " "
    let cube_strs = cube_str.split(" ");

    // Get the number and color
    let mut number: u32 = 0;
    let mut color: String = String::from("");
    for (i, cube_str) in cube_strs.enumerate() {
        if i == 0 {
            number = cube_str.parse::<u32>().unwrap();
        } else {
            color = String::from(cube_str);
        }
    }

    return Cube { color, number };
}

fn is_game_possible(game: &Game, bag: &Bag) -> bool {
    // Loop through the sets
    for set in game.sets.iter() {
        // Loop through the cubes
        for cube in set.cubes.iter() {
            // Check if the number of cubes of the color is greater than the number of cubes of that color in the bag
            if cube.color == "red" && cube.number > bag.red {
                return false;
            } else if cube.color == "green" && cube.number > bag.green {
                return false;
            } else if cube.color == "blue" && cube.number > bag.blue {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use crate::{parse_game_id, parse_sets, Set};

    #[test]
    fn test_parse_game_id() {
        assert_eq!(parse_game_id("Game 1: 3 blue, 4 red"), 1);
    }

    fn test_parse_sets() {
        // assert_eq!(parse_sets());
    }

    fn test_parse_cubes() {
        // assert_eq!(parse_cubes());
    }

    fn test_parse_cube() {
        // assert_eq!(parse_cube());
    }

    fn test_is_game_possible() {
        // assert_eq!(is_game_possible());
    }
}
