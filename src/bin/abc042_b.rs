//! [link](https://atcoder.jp/contests/abc042/tasks/abc042_b)

use std::fmt;
use std::fs;
use std::io;
use std::path;
use std::time;

const PUZZLE_ID: &str = "abc042_b";
const TITLE: &str = "AtCoder Beginner Contest 042, B - Iroha Loves Strings (ABC Edition)";
const USE_STDIN: bool = false;

fn main() -> Result<(), PuzzleError> {
    let raw_input = if USE_STDIN {
        read_stdin()?
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

type PuzzleError = &'static str;

fn read_stdin() -> Result<Vec<String>, PuzzleError> {
    let mut input = Vec::new();
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .map_err(|_| "error reading input from `stdin`")?;
    let n = input_line
        .split_ascii_whitespace()
        .next()
        .ok_or("missing `n`")?
        .parse::<usize>()
        .map_err(|_| "`n` must be positive integer")?;
    input.push(input_line.clone());
    for _ in 1..=n {
        input_line.clear();
        io::stdin()
            .read_line(&mut input_line)
            .map_err(|_| "error reading input from `stdin`")?;
        input.push(input_line.trim_end().to_string());
    }
    Ok(input)
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
    l: usize,
    s: Vec<String>,
}

impl TryFrom<&Vec<String>> for ParsedInput {
    type Error = PuzzleError;

    fn try_from(input: &Vec<String>) -> Result<Self, Self::Error> {
        if input.is_empty() {
            Err("input must be (n + 1) lines")?
        }
        let mut line1_iter = input[0].split_ascii_whitespace();
        let n = line1_iter
            .next()
            .ok_or("missing `n`")?
            .parse::<usize>()
            .map_err(|_| "`n` must be positive integer")?;
        let l = line1_iter
            .next()
            .ok_or("missing `l`")?
            .parse::<usize>()
            .map_err(|_| "`l` must be positive integer")?;
        if input.len() != n + 1 {
            Err("input must be (n + 1) lines")?
        }
        let s = input.iter().skip(1).cloned().collect::<Vec<_>>();
        Ok(ParsedInput { n, l, s })
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
    let mut fragments = input.s.to_vec();
    fragments.sort();
    let line = fragments.join("");
    Ok(PuzzleOutput { line })
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let raw_input = vec!["3 3", "dxx", "axx", "cxx"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "axxcxxdxx");
    }

    #[test]
    fn example1_file() {
        let raw_input = read_file("1").unwrap();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "axxcxxdxx");
    }

    #[test]
    fn invalid_missing_file() {
        let raw_input = read_file("0");
        assert_eq!(raw_input, Err("error reading input from file"));
    }

    #[test]
    fn invalid_input_line_count() {
        let raw_input = vec!["3 3", "dxx"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("input must be (n + 1) lines"));
    }

    #[test]
    fn invalid_input_missing_n() {
        let raw_input = vec![""].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `n`"));
    }

    #[test]
    fn invalid_input_missing_l() {
        let raw_input = vec!["3"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `l`"));
    }

    #[test]
    fn invalid_input_integer_n() {
        let raw_input = vec!["a 3"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`n` must be positive integer"));
    }

    #[test]
    fn invalid_input_integer_l() {
        let raw_input = vec!["3 l", "dxx", "axx", "cxx"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`l` must be positive integer"));
    }
}
