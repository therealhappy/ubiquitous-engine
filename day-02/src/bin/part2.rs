/*
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
 */

struct Game {
    game_id: u32,
    valid: bool,
    r: u32,
    g: u32,
    b: u32,
}

impl Game {
    fn new(game_id: u32, valid: bool, r: u32, g: u32, b: u32) -> Self {
        Self {
            game_id,
            valid,
            r,
            g,
            b,
        }
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.valid
    }
}

impl Game {
    fn load_game_data_from_line(line: &str) -> Self {
        let mut count = 0;
        let mut game_id = 0;
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for game_detail in line.split(':') {
            if game_detail.starts_with("Game") {
                let res = game_detail
                    .chars()
                    .skip_while(|c| !c.is_digit(10))
                    .collect::<String>();
                println!("res: {}", res);
                game_id = res.parse::<u32>().unwrap();
                continue;
            }

            for word in game_detail.split_whitespace() {
                let mut chars = word.chars();

                if word.parse::<u32>().is_ok() {
                    count = word.parse::<u32>().unwrap();
                    continue;
                }

                let color = chars.next().unwrap();

                match color {
                    'b' => {
                        if count > min_blue {
                            min_blue = count;
                        }
                    }
                    'g' => {
                        if count > min_green {
                            min_green = count;
                        }
                    }
                    'r' => {
                        if count > min_red {
                            min_red = count;
                        }
                    }
                    _ => panic!("Invalid color"),
                }

                count = 0;
            }
        }

        Self::new(game_id, true, min_red, min_green, min_blue)
    }
}

fn main() {
    let input: Vec<&str> = include_str!("./input.txt").lines().collect();
    let mut sum = 0;

    for line in input {
        let game = Game::load_game_data_from_line(line);
        println!("game: {:?}", game.game_id);
        if game.is_valid() {
            let set_power = game.r * game.g * game.b;
            println!("set_power: {}", set_power);
            sum += set_power;
        }
    }

    println!("Sum: {}", sum);
}
