/// Day 10: Pipe Maze
///
/// You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal
/// island. This island is surprisingly cold and there definitely aren't any thermals to glide on,
/// so you leave your hang glider behind.
///
/// You wander around for a while, but you don't find any people or animals. However, you do occasionally
/// find signposts labeled "Hot Springs" pointing in a seemingly consistent direction;
/// maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.
///
/// The landscape here is alien; even the flowers and trees are made of metal.
/// As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral
/// vision and jump into a big pipe! It didn't look like any animal you've ever seen;
/// if you want a better look, you'll need to get ahead of it.
///
/// Scanning the area, you discover that the entire field you're standing on is densely packed with pipes;
/// it was hard to tell at first because they're the same metallic silver color as the "ground".
/// You make a quick sketch of all of the surface pipes you can see (your puzzle input).
///
/// The pipes are arranged in a two-dimensional grid of tiles:
/// - `|` is a vertical pipe connecting north and south.
/// - `-` is a horizontal pipe connecting east and west.
/// - `L` is a 90-degree bend connecting north and east.
/// - `J` is a 90-degree bend connecting north and west.
/// - `7` is a 90-degree bend connecting south and west.
/// - `F` is a 90-degree bend connecting south and east.
/// - `.` is ground; there is no pipe in this tile.
/// - `S` is the starting position of the animal; there is a pipe on this tile,
///   but your sketch doesn't show what shape the pipe has.
///
/// Based on the acoustics of the animal's scurrying, you're confident the pipe
/// that contains the animal is one large, continuous loop.
///
/// For example, here is a square loop of pipe:
/// ```
/// .....
/// .F-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
/// If the animal had entered this loop in the northwest corner, the sketch would instead look like this:
/// ```
/// .....
/// .S-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
/// In the above diagram, the S tile is still a 90-degree F bend:
/// you can tell because of how the adjacent pipes connect to it.
///
/// Unfortunately, there are also many pipes that aren't connected to the loop!
/// This sketch shows the same loop as above:
/// ```
/// -L|F7
/// 7S-7|
/// L|7||
/// -L-J|
/// L|-JF
/// ```
/// In the above diagram, you can still figure out which pipes form the main loop:
/// they're the ones connected to S, pipes those pipes connect to, pipes those pipes connect to, and so on.
/// Every pipe in the main loop connects to its two neighbors (including S, which will have exactly
/// two pipes connecting to it, and which is assumed to connect back to those two pipes).
///
/// Here is a sketch that contains a slightly more complex main loop:
/// ```
/// ..F7.
/// .FJ|.
/// SJ.L7
/// |F--J
/// LJ...
/// ```
/// Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:
/// ```
/// 7-F7-
/// .FJ|7
/// SJLL7
/// |F--J
/// LJ.LJ
/// ```
/// If you want to get out ahead of the animal, you should find the tile in the loop that is farthest from
/// the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by
/// direct distance. Instead, you need to find the tile that would take the longest number of steps along
/// the loop to reach from the starting point - regardless of which way around the loop the animal went.
///
/// In the first example with the square loop:
/// ```
/// .....
/// .S-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
/// You can count the distance each tile in the loop is from the starting point like this:
/// ```
/// .....
/// .012.
/// .1.3.
/// .234.
/// .....
/// ```
/// In this example, the farthest point from the start is 4 steps away.
///
/// Here's the more complex loop again:
/// ```
/// ..F7.
/// .FJ|.
/// SJ.L7
/// |F--J
/// LJ...
/// ```
/// Here are the distances for each tile on that loop:
/// ```
/// ..45.
/// .236.
/// 01.78
/// 14567
/// 23...
/// ```
/// This loop's furthest tile is 8 steps away from the start.
///
/// Find the single giant loop starting at S. How many steps along the loop does it take
/// to get from the starting position to the point farthest from the starting position?
pub fn problem1(input: Vec<String>) -> usize {
    let input: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();

    let (start_i, start_j) = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &c)| c == 'S')
                .map(|(j, _)| (i, j))
        })
        .unwrap();

    use Direction::*;

    vec![U, D, L, R]
        .iter()
        .map(|d| {
            let mut i = start_i;
            let mut j = start_j;
            let mut d = *d;
            let mut steps = 0;
            loop {
                match d {
                    U => match i > 0 {
                        true => i -= 1,
                        _ => return 0,
                    },
                    D => i += 1,
                    L => match j > 0 {
                        true => j -= 1,
                        _ => return 0,
                    },
                    R => j += 1,
                }
                steps += 1;
                d = match (input[i][j], d) {
                    ('F', U) | ('L', D) => R,
                    ('7', U) | ('J', D) => L,

                    ('7', R) | ('F', L) => D,
                    ('J', R) | ('L', L) => U,

                    ('|', _) | ('-', _) => d,
                    ('S', _) | ('.', _) => return steps / 2,
                    (c, d) => panic!("unexpected char {c} for direction {d:?}"),
                };
            }
        })
        .max()
        .unwrap_or(0)
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[cfg(test)]
mod test {
    #[test]
    fn problem1_1() {
        assert_eq!(
            super::problem1(crate::lines_from_file("inputs/10-example1.txt")),
            4
        );
    }

    #[test]
    fn problem1_2() {
        assert_eq!(
            super::problem1(crate::lines_from_file("inputs/10-example2.txt")),
            8
        );
    }
}

#[cfg(test)]
mod solution {
    #[test]
    fn problem1() {
        let solution = super::problem1(crate::lines_from_file("inputs/10.txt"));
        println!("Solution for day 10 problem 1: {}", solution);
    }
}
