# check and compile
cargo check
cargo fmt
cargo clippy
cargo test
cargo build --release
# run
Write-Host("==============================")
& "./target/release/atcoder-rust.exe"
Write-Host("==========")
$puzzles = @(
    "practice_a",
    "abc042_a",
    "abc042_b",
    "abc042_c",
    "abc042_d",
    "abc043_a",
    "abc043_b",
    "abc043_c"
)
foreach ($puzzle in $puzzles){
    & "./target/release/$puzzle.exe"
}
Write-Host("==========")
