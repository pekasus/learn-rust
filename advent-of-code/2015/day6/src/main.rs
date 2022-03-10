use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Could not read from file.")
        .trim()
        .to_owned();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn get_cells_in_range_without_collect<'a>(
    start_coord: Coord,
    end_coord: Coord,
) -> impl Iterator<Item = (usize, usize)> {
    (start_coord.y..=end_coord.y)
        .map(move |y| (start_coord.x..=end_coord.x).map(move |x| (x, y)))
        .flatten()
}

fn get_cells_in_range(start_coord: &Coord, end_coord: &Coord) -> Vec<(usize, usize)> {
    (start_coord.y..=end_coord.y)
        .map(|y| (start_coord.x..=end_coord.x).map(move |x| (x, y)))
        .flatten()
        .collect()
}

fn part1(input: &str) -> usize {
    let mut matrix = [[false; 1000]; 1000];
    input.lines().for_each(|line| {
        let line: Vec<&str> = line.split_ascii_whitespace().collect();
        let start_coord;
        let end_coord;
        let on_off;

        match line[0..2] {
            ["toggle", _] => {
                start_coord = Coord::from_str(line[1]).unwrap();
                end_coord = Coord::from_str(line[3]).unwrap();
                get_cells_in_range(&start_coord, &end_coord)
                    .into_iter()
                    .for_each(|(x, y)| matrix[y][x] = !matrix[y][x])
            }
            ["turn", _] => {
                start_coord = Coord::from_str(line[2]).unwrap();
                end_coord = Coord::from_str(line[4]).unwrap();
                on_off = match line[1] {
                    "on" => true,
                    "off" => false,
                    _ => unreachable!(),
                };
                get_cells_in_range(&start_coord, &end_coord)
                    .into_iter()
                    .for_each(|(x, y)| {
                        matrix[y][x] = on_off;
                    })
            }
            _ => unreachable!(),
        }
    });
    get_cells_in_range(&Coord { x: 0, y: 0 }, &Coord { x: 999, y: 999 })
        .iter()
        .filter(|(x, y)| matrix[*y][*x])
        .count()
}

fn part2(input: &str) -> usize {
    let mut matrix = [[0_u32; 1000]; 1000];
    input.lines().for_each(|line| {
        let line: Vec<&str> = line.split_ascii_whitespace().collect();
        let start_coord;
        let end_coord;

        match line[0..2] {
            ["toggle", _] => {
                start_coord = Coord::from_str(line[1]).unwrap();
                end_coord = Coord::from_str(line[3]).unwrap();
                get_cells_in_range_without_collect(start_coord, end_coord).for_each(|(x, y)| {
                    matrix[y][x] += 2;
                });
            }
            ["turn", _] => {
                start_coord = Coord::from_str(line[2]).unwrap();
                end_coord = Coord::from_str(line[4]).unwrap();
                match line[1] {
                    "on" => get_cells_in_range_without_collect(start_coord, end_coord).for_each(
                        |(x, y)| {
                            matrix[y][x] += 1;
                        },
                    ),
                    "off" => get_cells_in_range_without_collect(start_coord, end_coord).for_each(
                        |(x, y)| {
                            matrix[y][x] = matrix[y][x].saturating_sub(1);
                        },
                    ),
                    _ => unreachable!(),
                };
            }
            _ => unreachable!(),
        }
    });
    matrix.iter().flatten().map(|&x| x as usize).sum()
}

#[derive(Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl FromStr for Coord {
    type Err = String;
    fn from_str(coord: &str) -> Result<Coord, Self::Err> {
        let [x, y]: [usize; 2] = coord
            .split(",")
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| coord)?
            .try_into()
            .map_err(|_| coord)?;
        Ok(Coord { x, y })
    }
}
