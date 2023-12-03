/*
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
 */
fn main() {
    let input: Vec<&str> = include_str!("./input.txt").lines().collect();
    let seats = transform_schematic_into_matrix(&input);
    let (symbols, part_numbers) = read_engine_schematic(&seats);

    let mut sum = 0;

    for part_number in part_numbers {
        if check_if_part_number_is_relevant(&symbols, &part_number) {
            sum += part_number.part_number;
        }
    }

    println!("sum: {}", sum);
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
}
#[derive(Debug, PartialEq)]
struct PartNumber {
    part_number: u32,
    row: usize,
    col: usize,
    adjacent_seats: Vec<char>,
}

impl PartNumber {
    fn new() -> Self {
        Self {
            part_number: 0,
            row: 0,
            col: 0,
            adjacent_seats: Vec::new(),
        }
    }
}

fn transform_schematic_into_matrix(input: &Vec<&str>) -> Vec<Vec<char>> {
    let mut seats: Vec<Vec<char>> = Vec::new();

    for line in input {
        let mut row: Vec<char> = Vec::new();

        for c in line.chars() {
            row.push(c);
        }

        seats.push(row);
    }

    seats
}

fn read_engine_schematic(seats: &Vec<Vec<char>>) -> (Vec<Symbol>, Vec<PartNumber>) {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut part_number: PartNumber;

    for (row, seat_row) in seats.iter().enumerate() {
        part_number = PartNumber::new();

        let mut lastcol = 0;
        for (col, seat) in seat_row.iter().enumerate() {
            if *seat == '.' {
                continue;
            }

            if seat.is_digit(10) {
                if col != 0 && lastcol == col - 1 {
                    part_number.part_number =
                        (part_number.part_number * 10) + seat.to_digit(10).unwrap();
                    lastcol = col;

                    part_number
                        .adjacent_seats
                        .extend(get_adjacent_seats_check_boundaries(seats, row, col));
                } else {
                    if part_number != PartNumber::new() {
                        part_numbers.push(part_number);
                    }

                    part_number = PartNumber::new();
                    part_number.part_number = seat.to_digit(10).unwrap();
                    part_number.adjacent_seats =
                        get_adjacent_seats_check_boundaries(seats, row, col);
                    part_number.col = col;
                    part_number.row = row;
                    lastcol = col;
                }
            } else {
                symbols.push(Symbol { symbol: *seat });
            }
        }

        part_numbers.push(part_number);
    }

    (symbols, part_numbers)
}

fn get_adjacent_seats_check_boundaries(
    seats: &Vec<Vec<char>>,
    row: usize,
    col: usize,
) -> Vec<char> {
    let mut adjacent_seats: Vec<char> = Vec::new();

    if row > 0 {
        adjacent_seats.push(seats[row - 1][col]);
    }

    if row < seats.len() - 1 {
        adjacent_seats.push(seats[row + 1][col]);
    }

    if col > 0 {
        adjacent_seats.push(seats[row][col - 1]);
    }

    if col < seats[row].len() - 1 {
        adjacent_seats.push(seats[row][col + 1]);
    }

    if row > 0 && col > 0 {
        adjacent_seats.push(seats[row - 1][col - 1]);
    }

    if row > 0 && col < seats[row].len() - 1 {
        adjacent_seats.push(seats[row - 1][col + 1]);
    }

    if row < seats.len() - 1 && col > 0 {
        adjacent_seats.push(seats[row + 1][col - 1]);
    }

    if row < seats.len() - 1 && col < seats[row].len() - 1 {
        adjacent_seats.push(seats[row + 1][col + 1]);
    }

    adjacent_seats
}

fn check_if_part_number_is_relevant(symbols: &Vec<Symbol>, part_number: &PartNumber) -> bool {
    for symbol in symbols {
        if part_number.adjacent_seats.contains(&symbol.symbol) {
            return true;
        }
    }

    false
}
