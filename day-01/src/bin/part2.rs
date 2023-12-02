/*
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
 */

// 54506 -- Wrong
// 54298 -- Wrong
use indexmap::indexmap;

fn main() {
    let input: Vec<&str> = include_str!("./input.txt").lines().collect();
    let out: Vec<i32> = build_string(input);

    let mut sum: i32 = 0;
    for i in out {
        sum += i;
    }

    println!("Sum: {}", sum);
}

fn build_string(input: Vec<&str>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    for line in input {
        let data: String = replace_text_digits(line);

        let mut num: String = String::new();
        for c in data.chars() {
            if c.is_digit(10) {
                num += &c.to_string();
                break;
            }
        }

        for c in data.chars().rev() {
            if c.is_digit(10) {
                num += &c.to_string();
                break;
            }
        }

        println!("num: {}", num);
        let a = num.parse::<i32>().unwrap();
        if a < 10 || a > 99 {
            panic!("Invalid number: {}", a)
        }
        res.push(a);
    }

    res
}

fn replace_text_digits(input: &str) -> String {
    let numbers = indexmap! {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9"
    };

    let mut buffer: String = String::new();
    let mut result: String = String::new();

    input.chars().for_each(|c| {
        buffer.push(c);
        println!("buffer: {}", buffer);

        if c.is_digit(10) {
            result.push_str(&buffer);
            buffer.clear();
        } else {
            for (key, value) in numbers.iter() {
                if buffer.contains(key) {
                    result.push_str(&buffer.replace(key, &value.to_string()));
                    buffer.drain(0..buffer.len() - 1);
                    break;
                }
            }
        }

        result.push_str(&buffer);
    });

    println!("{} => {}", input, result);
    return result.to_string();
}
