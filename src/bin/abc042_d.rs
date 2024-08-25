//! [link](https://atcoder.jp/contests/abc042/tasks/arc058_b)
//!
//! also: [link](https://atcoder.jp/contests/arc058/tasks/arc058_b)

use std::fmt;
use std::fs;
use std::io;
use std::path;
use std::time;

const PUZZLE_ID: &str = "abc042_d";
const TITLE: &str =
    "AtCoder Beginner Contest 042 / Regular Contest 058 : Task D - Iroha and a Grid";
const USE_STDIN: bool = false;
const MODULUS: usize = 1_000_000_007;

fn main() -> Result<(), PuzzleError> {
    let raw_input = if USE_STDIN {
        read_stdin(1)?
    } else {
        read_file("1")? // test case "4" also available, run takes ~6 sec.
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

type ItemType = usize;
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
    h: ItemType,
    w: ItemType,
    a: ItemType,
    b: ItemType,
}

impl TryFrom<&Vec<String>> for ParsedInput {
    type Error = PuzzleError;

    fn try_from(input: &Vec<String>) -> Result<Self, Self::Error> {
        if input.len() != 1 {
            Err("input must be 1 line")?
        }
        let mut line1_iter = input[0].split_ascii_whitespace();
        let h = line1_iter
            .next()
            .ok_or("missing `h`")?
            .parse::<ItemType>()
            .map_err(|_| "`h` must be positive integer")?;
        let w = line1_iter
            .next()
            .ok_or("missing `w`")?
            .parse::<ItemType>()
            .map_err(|_| "`w` must be positive integer")?;
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
        Ok(ParsedInput { h, w, a, b })
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
    let mut grid = [vec![1; input.w], vec![0; input.w]];
    let mut idx = 0;
    for _ in 1..(input.h - input.a) {
        idx = 1 - idx;
        grid[idx][0] = grid[1 - idx][0];
        for x in 1..input.w {
            grid[idx][x] = grid[1 - idx][x] + grid[idx][x - 1];
            if grid[idx][x] >= MODULUS {
                grid[idx][x] -= MODULUS;
            }
        }
    }
    for _ in (input.h - input.a)..input.h {
        idx = 1 - idx;
        grid[idx][input.b] = grid[1 - idx][input.b];
        for x in (input.b + 1)..input.w {
            grid[idx][x] = grid[1 - idx][x] + grid[idx][x - 1];
            if grid[idx][x] >= MODULUS {
                grid[idx][x] -= MODULUS;
            }
        }
    }
    Ok(PuzzleOutput {
        line: grid[idx][input.w - 1].to_string(),
    })
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let raw_input = vec!["2 3 1 1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "2");
    }

    #[test]
    fn example2() {
        let raw_input = vec!["10 7 3 4"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "3570");
    }

    #[test]
    fn example3() {
        let raw_input = vec!["100000 100000 99999 99999"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "1");
    }

    // too slow in debug mode
    #[test]
    #[ignore]
    fn example4() {
        let raw_input = vec!["100000 100000 44444 55555"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "738162020");
    }

    #[test]
    fn example1_file() {
        let raw_input = read_file("1").unwrap();
        let input = ParsedInput::try_from(&raw_input).unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(result.line, "2");
    }

    #[test]
    fn invalid_missing_file() {
        let raw_input = read_file("0");
        assert_eq!(raw_input, Err("error reading input from file"));
    }

    #[test]
    fn invalid_input_line_count() {
        let raw_input = vec!["2 3 1 1", "1"]
            .iter()
            .map(|&x| x.to_string())
            .collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("input must be 1 line"));
    }

    #[test]
    fn invalid_input_missing_h() {
        let raw_input = vec![""].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `h`"));
    }

    #[test]
    fn invalid_input_missing_w() {
        let raw_input = vec!["2"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `w`"));
    }

    #[test]
    fn invalid_input_missing_a() {
        let raw_input = vec!["2 3"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `a`"));
    }

    #[test]
    fn invalid_input_missing_b() {
        let raw_input = vec!["2 3 1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("missing `b`"));
    }

    #[test]
    fn invalid_input_integer_h() {
        let raw_input = vec!["h 3 1 1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`h` must be positive integer"));
    }

    #[test]
    fn invalid_input_integer_w() {
        let raw_input = vec!["2 w 1 1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`w` must be positive integer"));
    }

    #[test]
    fn invalid_input_integer_a() {
        let raw_input = vec!["2 3 a 1"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`a` must be positive integer"));
    }

    #[test]
    fn invalid_input_integer_b() {
        let raw_input = vec!["2 3 1 b"].iter().map(|&x| x.to_string()).collect();
        let input = ParsedInput::try_from(&raw_input);
        assert_eq!(input, Err("`b` must be positive integer"));
    }
}
