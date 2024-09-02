//! [link](https://atcoder.jp/contests/abc043/tasks/arc059_a)
//!
//! also [link](https://atcoder.jp/contests/arc059/tasks/arc059_a)

use std::fmt;
use std::fs;
use std::io;
use std::path;
use std::time;

const PUZZLE_ID: &str = "abc043_c";
const TITLE: &str = "AtCoder Beginner Contest 043 / Regular Contest 059 : Task C - Be Together";
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
    eprintln!(
        "=== {} [time: {} ms] : {}",
        PUZZLE_ID,
        elapsed.as_millis(),
        TITLE
    );
    println!("{}", result);
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
    n: usize,
    a: Vec<ItemType>,
}

impl TryFrom<&Vec<String>> for ParsedInput {
    type Error = PuzzleError;

    fn try_from(input: &Vec<String>) -> Result<Self, Self::Error> {
        if input.len() != 2 {
            Err("input must be 2 lines")?
        }
        let n = input[0]
            .trim()
            .parse::<usize>()
            .map_err(|_| "`n` must be positive integer")?;
        let a = input[1]
            .split_ascii_whitespace()
            .map(|x| {
                x.parse::<ItemType>()
                    .map_err(|_| "list items must be integers")
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ParsedInput { n, a })
    }
}

struct PuzzleOutput {
    line: String,
}

impl fmt::Display for PuzzleOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

fn solve(input: &ParsedInput) -> Result<PuzzleOutput, PuzzleError> {
    let mut best_cost = ItemType::MAX;
    for c in -100..=100 {
        let cost = input.a.iter().map(|&x| (x - c) * (x - c)).sum::<ItemType>();
        if cost < best_cost {
            best_cost = cost;
        }
    }
    Ok(PuzzleOutput {
        line: best_cost.to_string(),
    })
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let raw_input = vec!["2", "4 8"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "8");
    }

    #[test]
    fn example2() {
        let raw_input = vec!["3", "1 1 3"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "3");
    }

    #[test]
    fn example3() {
        let raw_input = vec!["3", "4 2 5"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "5");
    }

    #[test]
    fn example4() {
        let raw_input = vec!["4", "-100 -100 -100 -100"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "0");
    }

    #[test]
    fn example1_file() {
        let raw_input = read_file("1").unwrap();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "8");
    }

    #[test]
    fn invalid_missing_file() {
        let raw_input = read_file("0");
        assert_eq!(raw_input, Err("error reading input from file"));
    }

    #[test]
    fn invalid_input_line_count() {
        let raw_input = vec!["2"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("input must be 2 lines"));
    }

    #[test]
    fn invalid_input_integer_n() {
        let raw_input = vec!["a", "4 8"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`n` must be positive integer"));
    }

    #[test]
    fn invalid_input_integer_list_item() {
        let raw_input = vec!["2", "4 a"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("list items must be integers"));
    }
}
