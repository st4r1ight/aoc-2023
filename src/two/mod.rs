fn task_one(input: &'static str) {
    let lines = input.lines();
    let mut numbers: Vec<u32> = vec![];

    for (index, line) in lines.enumerate() {
        let mut game_possible = true;
        let game_record = line
            .strip_prefix(format!("Game {}: ", index + 1).as_str())
            .unwrap();
        for round in game_record.split("; ") {
            for block in round.split(", ") {
                let mut round_possible = true;
                let number_and_colour = block.split_whitespace().collect::<Vec<&str>>();
                let number = number_and_colour[0].parse::<i32>().unwrap();
                let colour = number_and_colour[1];

                match colour {
                    "red" => round_possible = number <= 12,
                    "green" => round_possible = number <= 13,
                    "blue" => round_possible = number <= 14,
                    _ => panic!(),
                }
                if !round_possible {
                    game_possible = false;
                }
            }
        }

        if game_possible {
            println!("{}", index + 1);
            numbers.push((index as u32) + 1);
        }
    }

    println!(
        "Sum of possible game IDs: {}",
        numbers.into_iter().sum::<u32>()
    );
}

fn task_two(input: &'static str) {
    let mut powers = vec![];

    let lines = input.lines();
    for (index, line) in lines.enumerate() {
        let mut min_red = 1;
        let mut min_green = 1;
        let mut min_blue = 1;

        let game_record = line
            .strip_prefix(format!("Game {}: ", index + 1).as_str())
            .unwrap();

        for round in game_record.split("; ") {
            for block in round.split(", ") {
                let number_and_colour = block.split_whitespace().collect::<Vec<&str>>();
                let number = number_and_colour[0].parse::<i32>().unwrap();
                let colour = number_and_colour[1];

                match colour {
                    "red" => {
                        if number > min_red {
                            min_red = number;
                        }
                    }
                    "blue" => {
                        if number > min_blue {
                            min_blue = number;
                        }
                    }
                    "green" => {
                        if number > min_green {
                            min_green = number;
                        }
                    }
                    _ => panic!(),
                }
            }
        }

        powers.push(min_red * min_green * min_blue)
    }

    println!("Sum of the powers is {}", powers.iter().sum::<i32>())
}

pub fn do_tasks() {
    let input = include_str!("input.txt");
    task_one(input);
    task_two(input);
}
