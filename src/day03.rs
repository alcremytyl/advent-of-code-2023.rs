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

ignore 114 & 58 as they're not adjacent to a symbol
sum 4361
*/

/*
* find first integer in line and if exists,
*   build a `Number` from chars until non-integer is found
* else
*   None for that line
*
* go to next line
*
* filter away all Nones
*
* return result
*/

#![allow(dead_code)]
#![allow(unused_mut)]

type Grid = Vec<Vec<Found>>;

#[derive(Debug, Clone)]
enum Found {
    Number {
        pos: (usize, usize),
        value: u16,
        length: usize,
        is_adjacent: bool,
    },
    Symbol {
        pos: (usize, usize),
        value: char,
    },
    Empty,
}
impl Found {
    fn set_found(&mut self) {
        match self {
            Self::Number { is_adjacent, .. } => *is_adjacent = true,
            _ => unreachable!(),
        }
    }
}

fn parse_input(grid: &mut Grid, input: &String) {
    for (col, line) in input.lines().enumerate() {
        let mut buffer = String::new();
        let mut col_vec = <Vec<Found>>::new();

        for (row, c) in line.char_indices() {
            if c.is_digit(10) {
                buffer.push(c);
            } else {
                if c == '.' {
                    col_vec.push(Found::Empty);
                } else {
                    col_vec.push(Found::Symbol {
                        pos: (row, col),
                        value: c,
                    });
                }
                if let Ok(value) = u16::from_str_radix(&buffer, 10) {
                    col_vec.push(Found::Number {
                        pos: (row, col),
                        length: buffer.len(),
                        is_adjacent: false,
                        value,
                    });
                    buffer.clear();
                }
            }
        }
        grid.push(col_vec.drain(0..).collect());
    }
}

pub fn part1(input: String) {
    let mut grid = Grid::new();

    parse_input(&mut grid, &input);

    for a in grid {
        for b in a {
            print!(
                "{}",
                match b {
                    Found::Empty => ".".to_string(),
                    Found::Symbol { value, .. } => value.to_string(),
                    Found::Number { value, .. } => value.to_string(),
                }
            );
        }
        println!();
    }
}
