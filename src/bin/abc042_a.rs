//! [link](https://atcoder.jp/contests/abc042/tasks/abc042_a)

use std::fmt;
use std::fs;
use std::io;
use std::path;
use std::time;

const PUZZLE_ID: &str = "abc042_a";
const TITLE: &str = "AtCoder Beginner Contest 042 : Task A - Iroha and Haiku (ABC Edition)";
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
    a: ItemType,
    b: ItemType,
    c: ItemType,
}

impl TryFrom<&Vec<String>> for ParsedInput {
    type Error = PuzzleError;

    fn try_from(input: &Vec<String>) -> Result<Self, Self::Error> {
        if input.len() != 1 {
            Err("input must be 1 line")?
        }
        let mut line1_iter = input[0].split_ascii_whitespace();
        let a = line1_iter
            .next()
            .ok_or("missing `a`")?
            .parse::<ItemType>()
            .map_err(|_| "`a` must be positive integer")?;
        let b = line1_iter
            .next()
            .ok_or("missing `b`")?
            .parse::<ItemType>()
            .map_err(|_| "`b` must be positive integer")?;
        let c = line1_iter
            .next()
            .ok_or("missing `c`")?
            .parse::<ItemType>()
            .map_err(|_| "`c` must be positive integer")?;
        Ok(ParsedInput { a, b, c })
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
    let line = match (input.a, input.b, input.c) {
        (5, 5, 7) | (5, 7, 5) | (7, 5, 5) => "YES",
        _ => "NO",
    };
    Ok(PuzzleOutput {
        line: line.to_string(),
    })
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let raw_input = vec!["5 5 7"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "YES");
    }

    #[test]
    fn example2() {
        let raw_input = vec!["7 7 5"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "NO");
    }

    #[test]
    fn example1_file() {
        let raw_input = read_file("1").unwrap();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "YES");
    }

    #[test]
    fn invalid_missing_file() {
        let raw_input = read_file("0");
        assert_eq!(raw_input, Err("error reading input from file"));
    }

    #[test]
    fn invalid_input_line_count() {
        let raw_input = vec!["5 5 7", "1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("input must be 1 line"));
    }

    #[test]
    fn invalid_input_missing_a() {
        let raw_input = vec![""].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `a`"));
    }

    #[test]
    fn invalid_input_missing_b() {
        let raw_input = vec!["5"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `b`"));
    }

    #[test]
    fn invalid_input_missing_c() {
        let raw_input = vec!["5 7"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `c`"));
    }

    #[test]
    fn invalid_input_integer_a() {
        let raw_input = vec!["a 5 7"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`a` must be positive integer"));
    }

    #[test]
    fn invalid_input_integer_b() {
        let raw_input = vec!["5 b 7"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`b` must be positive integer"));
    }

    #[test]
    fn invalid_input_integer_c() {
        let raw_input = vec!["5 7 c"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`c` must be positive integer"));
    }
}
