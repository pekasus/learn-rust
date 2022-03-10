use itertools::Itertools;
use std::{collections::BTreeSet, ops::Add};

struct UncountedBoard {
    dimensions: Coord,
    mines: BTreeSet<Coord>,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord { x, y }
    }
}

impl UncountedBoard {
    fn count_for(&self, cell: &Coord) -> u8 {
        let min_x = cell.x.saturating_sub(1);
        let max_x = cell.x.add(1).min(self.dimensions.x - 1);
        let range_x = min_x..=max_x;
        let min_y = cell.y.saturating_sub(1);
        let max_y = cell.y.add(1).min(self.dimensions.y - 1);
        let range_y = min_y..=max_y;
        let adjacent_positions = range_x.cartesian_product(range_y).map(Coord::from);

        let adjacent_mines = adjacent_positions.filter(|valid| self.mines.contains(valid));

        adjacent_mines.count() as u8
    }

    fn counted_board_cell_from(&self, coord: &Coord) -> CountedBoardCell {
        if self.mines.contains(coord) {
            CountedBoardCell::Mine
        } else {
            CountedBoardCell::Count(self.count_for(coord))
        }
    }
}

struct CountedBoard(Vec<Vec<CountedBoardCell>>);

enum CountedBoardCell {
    Count(u8),
    Mine,
}

impl TryFrom<&[&str]> for UncountedBoard {
    type Error = &'static str;

    fn try_from(minefield: &[&str]) -> Result<Self, Self::Error> {
        let height = minefield.len();
        let width = if height == 0 { 0 } else { minefield[0].len() };
        let dimensions = Coord {
            y: height,
            x: width,
        };

        if minefield.iter().any(|row| row.len() != dimensions.x) {
            return Err("Invalid row len");
        }

        let mines = minefield
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(x, cell)| match cell {
                        ' ' => None,
                        '*' => Some(Ok(Coord { x, y })),
                        _ => Some(Err("Invalid cell")),
                    })
            })
            .collect::<Result<BTreeSet<Coord>, &str>>()?;
        Ok(UncountedBoard { dimensions, mines })
    }
}

impl From<UncountedBoard> for CountedBoard {
    fn from(minefield: UncountedBoard) -> Self {
        CountedBoard(
            (0..minefield.dimensions.y)
                .into_iter()
                .map(|row| {
                    (0..minefield.dimensions.x)
                        .into_iter()
                        .map(|column| {
                            minefield.counted_board_cell_from(&(Coord { x: column, y: row }))
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

impl From<CountedBoard> for Vec<String> {
    fn from(minefield: CountedBoard) -> Self {
        minefield
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        CountedBoardCell::Mine => "*".to_string(),
                        CountedBoardCell::Count(0) => " ".to_string(),
                        CountedBoardCell::Count(n) => n.to_string(),
                    })
                    .collect()
            })
            .collect()
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let uncounted_board: UncountedBoard = minefield.try_into().unwrap();
    let counted_board: CountedBoard = uncounted_board.into();
    counted_board.into()
}
