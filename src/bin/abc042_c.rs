//! [link](https://atcoder.jp/contests/abc042/tasks/arc058_a)
//!
//! also [link](https://atcoder.jp/contests/arc058/tasks/arc058_a)

use std::fmt;
use std::fs;
use std::io;
use std::path;
use std::time;

const PUZZLE_ID: &str = "abc042_c";
const TITLE: &str =
    "AtCoder Beginner Contest 042 / Regular Contest 058 : Task C - Iroha's Obsession";
const USE_STDIN: bool = false;

fn main() -> Result<(), PuzzleError> {
    let raw_input = if USE_STDIN {
        read_stdin(2)?
    } else {
        read_file("1")?
    };
    let now = time::Instant::now();
    let input = ParsedInput::try_from(&raw_input)?;
    let result = solve(&input)?;
    let elapsed = now.elapsed();
    println!("{}", result);
    eprintln!(
        "=== {} [time: {} ms] : {}",
        PUZZLE_ID,
        elapsed.as_millis(),
        TITLE
    );
    Ok(())
}

type ItemType = i32;
type PuzzleError = &'static str;

fn read_stdin(count_lines: usize) -> Result<Vec<String>, PuzzleError> {
    io::stdin()
        .lines()
        .take(count_lines)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "error reading input from `stdin`")
}

fn read_file(test_case: &str) -> Result<Vec<String>, PuzzleError> {
    let input_path = format!("./input/{}_{}.txt", PUZZLE_ID, test_case);
    Ok(fs::read_to_string(path::Path::new(&input_path))
        .map_err(|_| "error reading input from file")?
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<_>>())
}

#[derive(Debug, PartialEq)]
struct ParsedInput {
    n: ItemType,
    k: usize,
    d: Vec<u8>,
}

impl TryFrom<&Vec<String>> for ParsedInput {
    type Error = PuzzleError;

    fn try_from(input: &Vec<String>) -> Result<Self, Self::Error> {
        if input.len() != 2 {
            Err("input must be 2 lines")?
        }
        let mut line1_iter = input[0].split_ascii_whitespace();
        let n = line1_iter
            .next()
            .ok_or("missing `n`")?
            .parse::<ItemType>()
            .map_err(|_| "`n` must be positive integer")?;
        let k = line1_iter
            .next()
            .ok_or("missing `k`")?
            .parse::<usize>()
            .map_err(|_| "`k` must be positive integer")?;
        let d = input[1]
            .split_ascii_whitespace()
            .map(|x| {
                x.parse::<u8>()
                    .map_err(|_| "`d` must be list of positive integers")
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ParsedInput { n, k, d })
    }
}

#[derive(Debug, PartialEq)]
struct PuzzleOutput {
    line: String,
}

impl fmt::Display for PuzzleOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

fn solve(input: &ParsedInput) -> Result<PuzzleOutput, PuzzleError> {
    let mut digits = [false; 10];
    for &digit in &input.d {
        if !(0..=9).contains(&digit) {
            Err("`d` must be list of decimal digits")?;
        }
        digits[digit as usize] = true;
    }
    let mut ans = input.n;
    loop {
        if ans
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .all(|x| !digits[x as usize])
        {
            break;
        }
        ans += 1;
    }
    let line = ans.to_string();
    Ok(PuzzleOutput { line })
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let raw_input = vec!["1000 8", "1 3 4 5 6 7 8 9"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "2000");
    }

    #[test]
    fn example1_file() {
        let raw_input = read_file("1").unwrap();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "2000");
    }

    #[test]
    fn invalid_missing_file() {
        let raw_input = read_file("0");
        assert_eq!(raw_input, Err("error reading input from file"));
    }

    #[test]
    fn invalid_input_line_count() {
        let raw_input = vec!["1000 8", "1 3 4 5 6 7 8 9", "1"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("input must be 2 lines"));
    }

    #[test]
    fn invalid_input_missing_n() {
        let raw_input = vec!["", "1 3 4 5 6 7 8 9"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `n`"));
    }

    #[test]
    fn invalid_input_missing_k() {
        let raw_input = vec!["1000", "1 3 4 5 6 7 8 9"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `k`"));
    }

    #[test]
    fn invalid_input_integer_n() {
        let raw_input = vec!["n 8", "1 3 4 5 6 7 8 9"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`n` must be positive integer"));
    }

    #[test]
    fn invalid_input_integer_k() {
        let raw_input = vec!["1000 k", "1 3 4 5 6 7 8 9"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`k` must be positive integer"));
    }

    #[test]
    fn invalid_input_integer_list_d() {
        let raw_input = vec!["1000 8", "1 a 4 5 6 7 8 9"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`d` must be list of positive integers"));
    }

    #[test]
    fn invalid_input_decimal_digit_list_d() {
        let raw_input = vec!["1000 8", "1 31 4 5 6 7 8 9"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input);
        assert_eq!(result, Err("`d` must be list of decimal digits"));
    }
}
