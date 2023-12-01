use std::ascii::AsciiExt;
use std::sync::mpsc;
use std::thread;

fn task_one() {
    let lines = include_str!("t1.txt").lines();
    let lines_2 = lines.clone();

    let (in_1, out) = mpsc::channel();
    let in_2 = in_1.clone();

    let mut numbers: Vec<u32> = vec![0; 1000];

    thread::spawn(move || {
        for (index, line) in lines.enumerate() {
            for character in line.chars() {
                match character.to_digit(10) {
                    None => continue,
                    Some(digit) => {
                        in_1.send((index, digit * 10));
                        break;
                    }
                }
            }
        }
    });

    thread::spawn(move || {
        for line in lines_2 {
            for (index, character) in line.chars().rev().enumerate() {
                match character.to_digit(10) {
                    None => continue,
                    Some(digit) => {
                        in_2.send((index, digit));
                        break;
                    }
                }
            }
        }
    });

    for (index, number) in out {
        numbers[index] += number;
    }

    println!("{}", numbers.iter().sum::<u32>())
}

fn task_two(input: &'static str) {
    fn contains_letter_digit(substring: &str) -> Option<u32> {
        let word_digits = [
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
            (0, "zero"),
        ];

        for (digit, word) in word_digits {
            if substring.contains(word) {
                return Some(digit);
            }
        }

        None
    }

    let lines = input.lines();
    let lines2 = lines.clone();

    let (channel_1, out) = mpsc::channel();
    let channel_2 = channel_1.clone();

    thread::spawn(move || {
        for (index, line) in lines.enumerate() {
            let mut number_string = String::new();
            for character in line.chars() {
                match character.to_digit(10) {
                    Some(digit) => {
                        channel_1.send((index, digit * 10));
                        break;
                    }
                    None => {
                        number_string.push(character);
                        match contains_letter_digit(number_string.as_str()) {
                            None => continue,
                            Some(digit) => {
                                channel_1.send((index, digit * 10));
                                break;
                            }
                        }
                    }
                }
            }
        }
    });

    thread::spawn(move || {
        for (index, line) in lines2.enumerate() {
            let mut number_string = String::new();
            for character in line.chars().rev() {
                match character.to_digit(10) {
                    Some(digit) => {
                        channel_2.send((index, digit));
                        break;
                    }
                    None => {
                        number_string = format!("{}{}", character, number_string);
                        match contains_letter_digit(number_string.as_str()) {
                            None => continue,
                            Some(digit) => {
                                channel_2.send((index, digit));
                                break;
                            }
                        }
                    }
                }
            }
        }
    });

    let mut numbers = vec![0; 1000];

    for (index, number) in out {
        numbers[index] += number;
    }

    println!("{}", numbers.iter().sum::<u32>())
}

pub fn do_tasks() {
    let input = include_str!("t1.txt");
    task_one();
    task_two(input);
}
