/// Day 3: Gear Ratios
///
/// You and the Elf eventually reach a gondola lift station; he says the gondola lift will
/// take you up to the water source, but this is as far as he can bring you. You go inside.
///
/// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
///
/// "Aaah!"
///
/// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise.
/// "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now;
/// it'll still be a while before I can fix it." You offer to help.
///
/// The engineer explains that an engine part seems to be missing from the engine, but nobody
/// can figure out which one. If you can add up all the part numbers in the engine schematic,
/// it should be easy to work out which part is missing.
///
/// The engine schematic (your puzzle input) consists of a visual representation of the engine.
/// There are lots of numbers and symbols you don't really understand, but apparently any number
/// adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum.
/// (Periods (.) do not count as a symbol.)
///
/// Here is an example engine schematic:
/// ```
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
/// ```
/// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol:
/// 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol
/// and so is a part number; their sum is 4361.
///
/// Of course, the actual engine schematic is much larger.
/// What is the sum of all of the part numbers in the engine schematic?
pub fn day03_problem1(mut input: Vec<String>) -> u32 {
    input.iter_mut().for_each(|line| line.push('.')); // blink blink we avoid the edge case where the number is at the end of the line
    input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .fold((0, 0, 0), |(count, start, mut sum), (j, char)| match char {
                    '0'..='9' => (count * 10 + char.to_digit(10).unwrap(), start, sum),
                    _ => {
                        if count > 0
                            && (char != '.' || // right
                            (start > 0 && line.chars().nth(start - 1).is_some_and(|c| c != '.')) ||
                            i.checked_sub(1) // top index
                                .and_then(|x| input.get(x)) // top line
                                .is_some_and(|top| top
                                    .get(start.saturating_sub(1)..=j)
                                    .is_some_and(|s| s.chars().any(|c| !matches!(c, '0'..='9' | '.')))
                                ) ||
                            input
                                .get(i + 1) // bottom line
                                .is_some_and(|btm| btm
                                    .get(start.saturating_sub(1)..=j)
                                    .is_some_and(|s| s.chars().any(|c| !matches!(c, '0'..='9' | '.')))
                                )) {
                            sum += count;
                        }
                        (0, j + 1, sum)
                    }
                })
                .2
        })
        .sum()
}

#[test]
fn day03_problem1_test() {
    let input = vec![
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string(),
    ];
    assert_eq!(day03_problem1(input), 4361);
}

#[test]
fn day03_problem1_solution() {
    let solution = day03_problem1(crate::lines_from_file("inputs/03.txt"));
    println!("Solution for day 03 problem 1: {}", solution);
}
