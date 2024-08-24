//! [link](https://atcoder.jp/contests/practice/tasks/practice_1)

use std::fmt;
use std::fs;
use std::io;
use std::path;
use std::time;

const PUZZLE_ID: &str = "practice_a";
const TITLE: &str = "practice contest, A - Welcome to AtCoder";
const USE_STDIN: bool = false;

fn main() -> Result<(), PuzzleError> {
    let raw_input = if USE_STDIN {
        read_stdin(3)?
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
    a: ItemType,
    b: ItemType,
    c: ItemType,
    s: String,
}

impl TryFrom<&Vec<String>> for ParsedInput {
    type Error = PuzzleError;

    fn try_from(input: &Vec<String>) -> Result<Self, Self::Error> {
        if input.len() != 3 {
            Err("input must be 3 lines")?
        }
        let a = input[0]
            .parse::<ItemType>()
            .map_err(|_| "`a` must be integer")?;
        let mut line1_iter = input[1].split_ascii_whitespace();
        let b = line1_iter
            .next()
            .ok_or("missing `b`")?
            .parse::<ItemType>()
            .map_err(|_| "`b` must be integer")?;
        let c = line1_iter
            .next()
            .ok_or("missing `c`")?
            .parse::<ItemType>()
            .map_err(|_| "`c` must be integer")?;
        let s = input[2].to_string();
        Ok(ParsedInput { a, b, c, s })
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
    Ok(PuzzleOutput {
        line: format!("{} {}", input.a + input.b + input.c, input.s),
    })
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let raw_input = vec!["1", "2 3", "test"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "6 test");
    }

    #[test]
    fn example2() {
        let raw_input = vec!["72", "128 256", "myonmyon"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "456 myonmyon");
    }

    #[test]
    fn example1_file() {
        let raw_input = read_file("1").unwrap();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "6 test");
    }

    #[test]
    fn invalid_missing_file() {
        let raw_input = read_file("0");
        assert_eq!(raw_input, Err("error reading input from file"));
    }

    #[test]
    fn invalid_input_line_count() {
        let raw_input = vec!["1", "2 3"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("input must be 3 lines"));
    }

    #[test]
    fn invalid_input_integer_a() {
        let raw_input = vec!["a", "2 3", "test"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`a` must be integer"));
    }

    #[test]
    fn invalid_input_integer_b() {
        let raw_input = vec!["1", "b 3", "test"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`b` must be integer"));
    }

    #[test]
    fn invalid_input_integer_c() {
        let raw_input = vec!["1", "2 c", "test"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`c` must be integer"));
    }

    #[test]
    fn invalid_input_missing_b() {
        let raw_input = vec!["1", "", "test"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `b`"));
    }

    #[test]
    fn invalid_input_missing_c() {
        let raw_input = vec!["1", "2", "test"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `c`"));
    }
}
