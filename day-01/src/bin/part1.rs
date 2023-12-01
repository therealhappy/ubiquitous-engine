/*
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
 */

fn main() {
    let input: Vec<&str> = include_str!("./input.txt").lines().collect();
    let out: Vec<i8> = build_string(input);

    let mut sum: i32 = 0;
    for i in out {
        sum += i as i32;
    }

    println!("Sum: {}", sum);
}

fn build_string(input: Vec<&str>) -> Vec<i8> {
    let mut res: Vec<i8> = Vec::new();

    for line in input {
        let mut num: String = String::new();
        for c in line.chars() {
            if c.is_digit(10) {
                num += &c.to_string();
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                num += &c.to_string();
                break;
            }
        }

        res.push(num.parse::<i8>().unwrap());
    }

    res
}
