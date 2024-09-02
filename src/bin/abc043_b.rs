//! [link](https://atcoder.jp/contests/abc043/tasks/abc043_b)

use std::fmt;
use std::fs;
use std::io;
use std::path;
use std::time;

const PUZZLE_ID: &str = "abc043_b";
const TITLE: &str = "AtCoder Beginner Contest 043 : Task B - Unhappy Hacking (ABC Edit)";
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
    s: String,
}

impl TryFrom<&Vec<String>> for ParsedInput {
    type Error = PuzzleError;

    fn try_from(input: &Vec<String>) -> Result<Self, Self::Error> {
        if input.len() != 1 {
            Err("input must be 1 line")?
        }
        Ok(ParsedInput {
            s: input[0].to_owned(),
        })
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
    let mut line = String::new();
    for c in input.s.chars() {
        match c {
            '0' | '1' => line.push(c),
            'B' => {
                if !line.is_empty() {
                    line.truncate(line.len() - 1)
                }
            }
            _ => Err("invalid character in input")?,
        }
    }
    Ok(PuzzleOutput { line })
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let raw_input = vec!["01B0"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "00");
    }

    #[test]
    fn example2() {
        let raw_input = vec!["0BB1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "1");
    }

    #[test]
    fn example1_file() {
        let raw_input = read_file("1").unwrap();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "00");
    }

    #[test]
    fn invalid_missing_file() {
        let raw_input = read_file("0");
        assert_eq!(raw_input, Err("error reading input from file"));
    }

    #[test]
    fn invalid_input_line_count() {
        let raw_input = vec!["0BB1", "1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("input must be 1 line"));
    }

    #[test]
    fn invalid_input_only_01b_allowed() {
        let raw_input = vec!["0BAB1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input);
        assert_eq!(result, Err("invalid character in input"));
    }
}
