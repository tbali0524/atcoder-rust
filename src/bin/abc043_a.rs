//! [link](https://atcoder.jp/contests/abc043/tasks/abc043_a)

use std::fmt;
use std::fs;
use std::io;
use std::path;
use std::time;

const PUZZLE_ID: &str = "abc043_a";
const TITLE: &str = "AtCoder Beginner Contest 043 : Task A - Children and Candies (ABC Edit)";
const USE_STDIN: bool = false;

fn main() -> Result<(), PuzzleError> {
    let raw_input = if USE_STDIN {
        read_stdin(1)?
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
    n: ItemType,
}

impl TryFrom<&Vec<String>> for ParsedInput {
    type Error = PuzzleError;

    fn try_from(input: &Vec<String>) -> Result<Self, Self::Error> {
        if input.len() != 1 {
            Err("input must be 1 line")?
        }
        let mut line1_iter = input[0].split_ascii_whitespace();
        let n = line1_iter
            .next()
            .ok_or("missing `n`")?
            .parse::<ItemType>()
            .map_err(|_| "`n` must be positive integer")?;
        Ok(ParsedInput { n })
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
    let ans = (input.n * (input.n + 1)) / 2;
    Ok(PuzzleOutput {
        line: ans.to_string(),
    })
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let raw_input = vec!["3"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "6");
    }

    #[test]
    fn example2() {
        let raw_input = vec!["10"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "55");
    }

    #[test]
    fn example3() {
        let raw_input = vec!["1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "1");
    }

    #[test]
    fn example1_file() {
        let raw_input = read_file("1").unwrap();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "6");
    }

    #[test]
    fn invalid_missing_file() {
        let raw_input = read_file("0");
        assert_eq!(raw_input, Err("error reading input from file"));
    }

    #[test]
    fn invalid_input_line_count() {
        let raw_input = vec!["3", "1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("input must be 1 line"));
    }

    #[test]
    fn invalid_input_missing_n() {
        let raw_input = vec![""].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `n`"));
    }

    #[test]
    fn invalid_input_integer_n() {
        let raw_input = vec!["a 5 7"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`n` must be positive integer"));
    }
}
