/// Day 1: Trebuchet?!
///
/// Something is wrong with global snow production, and you've been selected to take a look.
/// The Elves have even given you a map; on it, they've used stars to mark
/// the top fifty locations that are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations,
/// you need to check all fifty stars by December 25th.
///
/// Collect stars by solving puzzles.
/// Two puzzles will be made available on each day in the Advent calendar;
/// the second puzzle is unlocked when you complete the first. Each puzzle grants one star.
/// Good luck!
///
/// You try to ask why they can't just use a weather machine ("not powerful enough")
/// and where they're even sending you ("the sky") and why your map looks mostly blank
/// ("you sure ask a lot of questions") and hang on did you just say the sky
/// ("of course, where do you think snow comes from") when you realize that the Elves
/// are already loading you into a trebuchet ("please hold still, we need to strap you in").
///
/// As they're making the final adjustments, they discover that their calibration document (your
/// puzzle input) has been amended by a very young Elf who was apparently just excited to show off
/// her art skills. Consequently, the Elves are having trouble reading the values on the document.
///
/// The newly-improved calibration document consists of lines of text; each line originally
/// contained a specific calibration value that the Elves now need to recover. On each line,
/// the calibration value can be found by combining the first digit and the last digit
/// (in that order) to form a single two-digit number.
///
/// For example:
/// ---
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
/// ---
/// In this example, the calibration values of these four lines are 12, 38, 15, and 77.
/// Adding these together produces 142.
///
/// Consider your entire calibration document. What is the sum of all of the calibration values?
pub fn day01_problem1(input: Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap_or(0);
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

#[test]
fn day01_problem1_test() {
    let input: Vec<String> = vec![
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string(),
    ];
    assert_eq!(day01_problem1(input), 142);
}

#[test]
fn day01_problem1_solution() {
    let solution = day01_problem1(crate::lines_from_file("inputs/01.txt"));
    println!("Solution for day 01 problem 1: {}", solution);
}

/// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with
/// letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and last digit on each line.
/// For example:
/// ---
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// ---
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
/// Adding these together produces 281.
pub fn day01_problem2(input: Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut digits = line
                .chars()
                .enumerate()
                .filter_map(|(i, c)| match c.to_digit(10) {
                    Some(d) => Some(d),
                    None => do_match(&line[i..]),
                });
            let first = digits.next().unwrap_or(0);
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn do_match(input: &str) -> Option<u32> {
    for (i, digit) in DIGITS.iter().enumerate() {
        if input.starts_with(digit) {
            return Some(i as u32 + 1);
        }
    }
    None
}

#[test]
fn day01_problem2_test() {
    let input: Vec<String> = vec![
        "two1nine".to_string(),
        "eightwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string(),
    ];
    assert_eq!(day01_problem2(input), 281);
}

#[test]
fn day01_problem2_solution() {
    let solution = day01_problem2(crate::lines_from_file("inputs/01.txt"));
    println!("Solution for day 01 problem 2: {}", solution);
}
