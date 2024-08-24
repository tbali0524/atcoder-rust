//! [link](https://atcoder.jp/contests/practice/tasks/practice_2)
//!
//! This is an interactive task, it cannot run without a judge.

use std::io;
use std::io::Write;

const PUZZLE_ID: &str = "practice_b";
const TITLE: &str = "practice contest, B - Interactive Sorting";

#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
enum SortAlgorithms {
    BubbleSort,
    QuickSort, // not working
    MergeSort, // not working
}

const SORT_ALGO: SortAlgorithms = SortAlgorithms::BubbleSort;

fn main() -> Result<(), PuzzleError> {
    let (n, _q) = read_init_input()?;
    let mut a = ('A'..='Z').take(n).collect::<Vec<_>>();
    match SORT_ALGO {
        SortAlgorithms::BubbleSort => bubble_sort(&mut a)?,
        SortAlgorithms::QuickSort => quick_sort(&mut a)?,
        SortAlgorithms::MergeSort => merge_sort(&mut a)?,
    }
    let result = a.iter().collect::<String>();
    println!("! {}", result);
    io::stdout()
        .flush()
        .map_err(|_| "error writing to `stdout`")?;
    eprintln!("=== {} : {}", PUZZLE_ID, TITLE);
    Ok(())
}

/// bubble_sort() is O(n^2), so it passes only test set #1
fn bubble_sort(a: &mut [char]) -> Result<(), PuzzleError> {
    let n = a.len();
    for _ in 0..n {
        for j in 0..(n - 1) {
            println!("? {} {}", a[j], a[j + 1]);
            io::stdout()
                .flush()
                .map_err(|_| "error writing to `stdout`")?;
            let ans = read_turn_input()?;
            if ans == '>' {
                a.swap(j, j + 1);
            }
        }
    }
    Ok(())
}

/// based on [<https://en.wikipedia.org/wiki/Quicksort>]
fn quick_sort(a: &mut [char]) -> Result<(), PuzzleError> {
    let hi = a.len() - 1;
    qs_iter(a, 0, hi)?;
    Ok(())
}

fn qs_iter(a: &mut [char], lo: usize, hi: usize) -> Result<(), PuzzleError> {
    if lo >= hi {
        return Ok(());
    }
    let p = qs_partition(a, lo, hi)?;
    qs_iter(a, lo, p - 1)?;
    qs_iter(a, p + 1, hi)?;
    Ok(())
}

fn qs_partition(a: &mut [char], lo: usize, hi: usize) -> Result<usize, PuzzleError> {
    let pivot = a[hi];
    let mut i = lo;
    for j in lo..hi {
        let mut ans = '<';
        if a[j] != pivot {
            println!("? {} {}", a[j], pivot);
            ans = read_turn_input()?;
            io::stdout()
                .flush()
                .map_err(|_| "error writing to `stdout`")?;
        }
        if ans == '<' {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, hi);
    Ok(i)
}

/// based on [<https://en.wikipedia.org/wiki/Merge_sort>]
fn merge_sort(a: &mut [char]) -> Result<(), PuzzleError> {
    let mut b = a.to_vec();
    let n = a.len();
    ms_top_down_split_merge(a, 0, n, &mut b)?;
    Ok(())
}

fn ms_top_down_split_merge(
    a: &mut [char],
    i_begin: usize,
    i_end: usize,
    b: &mut [char],
) -> Result<(), PuzzleError> {
    if i_end <= i_begin + 1 {
        return Ok(());
    }
    let i_middle = (i_end + i_begin) / 2;
    ms_top_down_split_merge(a, i_begin, i_middle, b)?;
    ms_top_down_split_merge(a, i_middle, i_end, b)?;
    ms_top_down_merge(b, i_begin, i_middle, i_end, a)?;
    Ok(())
}

#[allow(clippy::needless_range_loop)]
fn ms_top_down_merge(
    a: &mut [char],
    i_begin: usize,
    i_middle: usize,
    i_end: usize,
    b: &mut [char],
) -> Result<(), PuzzleError> {
    let mut i = i_begin;
    let mut j = i_middle;
    for k in i_begin..i_end {
        let mut is_i = true;
        if i >= i_middle {
            is_i = false;
        } else if j < i_end && i != j {
            println!("? {} {}", a[i], a[j]);
            io::stdout()
                .flush()
                .map_err(|_| "error writing to `stdout`")?;
            let ans = read_turn_input()?;
            is_i = ans == '<';
        }
        if is_i {
            b[k] = a[i];
            i += 1;
        } else {
            b[k] = a[j];
            j += 1;
        }
    }
    Ok(())
}

type ItemType = i32;
type PuzzleError = &'static str;

fn read_init_input() -> Result<(usize, ItemType), PuzzleError> {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .map_err(|_| "error reading input from `stdin`")?;
    let mut line1_iter = input_line.split_ascii_whitespace();
    let n = line1_iter
        .next()
        .ok_or("missing `n`")?
        .parse::<usize>()
        .map_err(|_| "`n` must be integer")?;
    let q = line1_iter
        .next()
        .ok_or("missing `q`")?
        .parse::<ItemType>()
        .map_err(|_| "`q` must be integer")?;
    Ok((n, q))
}

fn read_turn_input() -> Result<char, PuzzleError> {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .map_err(|_| "error reading input from `stdin`")?;
    let ans = input_line.chars().next().ok_or("missing `ans`")?;
    Ok(ans)
}
