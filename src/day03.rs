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
pub fn problem1(mut input: Vec<String>) -> u32 {
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

/// The engineer finds the missing part and installs it in the engine!
/// As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.
///
/// You don't seem to be going very fast, though. Maybe something is still wrong?
/// Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.
///
/// Before you can explain the situation, she suggests that you look out the window.
/// There stands the engineer, holding a phone in one hand and waving with the other.
/// You're going so slowly that you haven't even left the station. You exit the gondola.
///
/// The missing part wasn't the only issue - one of the gears in the engine is wrong.
/// A gear is any * symbol that is adjacent to exactly two part numbers.
/// Its gear ratio is the result of multiplying those two numbers together.
///
/// This time, you need to find the gear ratio of every gear and add them all up
/// so that the engineer can figure out which gear needs to be replaced.
///
/// Consider the same engine schematic again:
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
/// In this schematic, there are two gears.
/// The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345.
/// The second gear is in the lower right; its gear ratio is 451490.
/// (The * adjacent to 617 is not a gear because it is only adjacent to one part number.)
/// Adding up all of the gear ratios produces 467835.
///
/// What is the sum of all of the gear ratios in your engine schematic?
pub fn problem2(input: Vec<String>) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                // filter_map gives random errors with lifetimes
                .map(|(j, char)| match char {
                    '*' => {
                        [
                            get_num(&line.chars().collect(), j),
                            i.checked_sub(1)
                                .and_then(|idx| {
                                    input.get(idx).map(|v| get_num(&v.chars().collect(), j))
                                })
                                .unwrap_or_default(),
                            input
                                .get(i + 1)
                                .map(|btm| get_num(&btm.chars().collect(), j))
                                .unwrap_or_default(),
                        ]
                        .into_iter()
                        .flatten()
                        .enumerate()
                        .fold((0, 0), |acc, (idx, num)| match idx {
                            0 => (num, 0),
                            1 => (0, acc.0 * num),
                            _ => (0, 0),
                        })
                        .1
                    }

                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum()
}

fn get_num(ch: &Vec<char>, at: usize) -> Vec<u32> {
    if !matches!(ch.get(at), Some(c) if c.is_ascii_digit()) {
        let mut res = vec![];
        if at > 0 && matches!(ch.get(at - 1), Some(c) if c.is_ascii_digit()) {
            res.extend(get_num(ch, at - 1));
        }
        if at < ch.len() - 1 && matches!(ch.get(at + 1), Some(c) if c.is_ascii_digit()) {
            res.extend(get_num(ch, at + 1));
        }
        return res;
    }

    let l = (0..at)
        .rev()
        .find(|&i| !matches!(ch.get(i), Some(c) if c.is_ascii_digit()))
        .map(|i| i + 1);
    let mut res = 0;
    for i in l.unwrap_or_default().. {
        match ch.get(i) {
            Some(c) if c.is_ascii_digit() => res = res * 10 + c.to_digit(10).unwrap(),
            _ => break,
        }
    }
    vec![res]
}

#[cfg(test)]
mod test {
    #[test]
    fn problem1() {
        assert_eq!(
            super::problem1(crate::lines_from_file("inputs/03-example.txt")),
            4361
        );
    }

    #[test]
    fn problem2() {
        assert_eq!(
            super::problem2(crate::lines_from_file("inputs/03-example.txt")),
            467835
        );
    }
}

#[cfg(test)]
mod solution {
    #[test]
    fn problem1() {
        let solution = super::problem1(crate::lines_from_file("inputs/03.txt"));
        println!("Solution for day 03 problem 1: {}", solution);
    }

    #[test]
    fn problem2() {
        let solution = super::problem2(crate::lines_from_file("inputs/03.txt"));
        println!("Solution for day 03 problem 2: {}", solution);
    }
}
