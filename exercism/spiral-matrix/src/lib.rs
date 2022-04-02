use std::{collections::HashMap, iter};

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; size]; size];
    if size == 1 {
        matrix[0][0] = 1;
        return matrix;
    }
    let mut cursor = Cursor(0, 0, Direction::Right);
    let mut directions = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ]
    .iter()
    .cycle();
    directions.next();
    (1..=(size * size)).for_each(|n| {
        matrix[cursor.0 as usize][cursor.1 as usize] = n as u32;

        // advance if we can: if valid
        let mut peek_cursor = cursor.peek(&matrix);

        if peek_cursor.is_none()
            || matrix[peek_cursor.clone().unwrap().0 as usize]
                [peek_cursor.clone().unwrap().1 as usize]
                != 0
        {
            // change directon
            cursor.2 = *directions.next().unwrap();
            peek_cursor = cursor.peek(&matrix);
        }
        let peek_cursor = peek_cursor.unwrap();
        cursor.0 = peek_cursor.0;
        cursor.1 = peek_cursor.1;
    });
    matrix
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Cursor(u32, u32, Direction);

impl Cursor {
    fn peek(&self, matrix: &Vec<Vec<u32>>) -> Option<Cursor> {
        use Direction::*;
        let mut new = self.clone();
        match new.2 {
            Up => {
                if new.0 > 0 {
                    new.0 -= 1;
                    return Some(new);
                }
            }
            Down => {
                if (new.0 as usize) < matrix.len() - 1 {
                    new.0 += 1;
                    return Some(new);
                }
            }
            Left => {
                if new.1 > 0 {
                    new.1 -= 1;
                    return Some(new);
                }
            }
            Right => {
                if (new.1 as usize) < matrix.len() - 1 {
                    new.1 += 1;
                    return Some(new);
                }
            }
        }
        None
    }

    // fn change_direction(&self, current_direction: Direction) {

    // }
}

// OLD CODE /////////////////////////////////////////////////////////////////////////////

// Choice of which function to use.
// Another way could be to re-export the function under a different name
// #[allow(non_upper_case_globals)]
// pub const spiral_matrix: &dyn Fn(u32) -> Vec<Vec<u32>> = /*&spiral_matrix_by_lines;*/
//     &spiral_matrix_step_by_step;

// pub fn spiral_matrix_by_lines(size: u32) -> Vec<Vec<u32>> {
//     let mut coords: HashMap<(u8, u8), u32> = HashMap::new();
//     let mut row_limits: (u32, u32) = (0, size - 1);
//     let mut column_limits: (u32, u32) = (0, size - 1);
//     let mut current_number: u32 = 0;
//     let mut current_coord: (u8, u8) = (0, 0);

//     while (coords.len() as u32) < size.pow(2) {
//         while current_coord.0 as u32 <= column_limits.1 {
//             coords.insert(current_coord, current_number);
//             current_number += 1;
//             current_coord.0 += 1;
//         }
//         row_limits.0 = row_limits.0 + 1;
//         current_coord.0 = row_limits.0 as u8;

//         while current_coord.1 as u32 <= row_limits.1 {
//             coords.insert(current_coord, current_number);
//             current_number += 1;
//             current_coord.1 += 1;
//         }
//         //column_limits.1 =
//     }
//     todo!();
// }

// // Like indices to Vec<Vec<u32>> for result of spiral_matrix(u32), but both are isize
// type Position = (isize, isize);

// /// Only one of the two axis is non-zero, and either -1 or 1.
// #[derive(PartialEq, Eq)]
// struct Direction {
//     /// row delta
//     row: i8,
//     /// column delta
//     col: i8,
// }

// impl Direction {
//     const fn new(row: i8, col: i8) -> Self {
//         Self { row, col }
//     }

//     fn turn_right(&self) -> Self {
//         match self {
//             &RIGHT => DOWN,
//             &DOWN => LEFT,
//             &LEFT => UP,
//             &UP => RIGHT,
//             _ => unreachable!(),
//         }
//     }

//     /// Advance one step in this direction. It may get into negative.
//     fn advance(&self, pos: Position) -> Position {
//         (pos.0 + self.row as isize, pos.1 + self.col as isize)
//     }
// }

// fn within_square(pos: Position, square_size: usize) -> bool {
//     pos.0 >= 0 && pos.0 < (square_size as isize) && pos.1 >= 0 && pos.1 < (square_size as isize)
// }

// fn cell(vec: &mut Vec<Vec<u32>>, pos: Position) -> &mut u32 {
//     &mut vec[pos.0 as usize][pos.1 as usize]
// }

// const RIGHT: Direction = Direction::new(0, 1);
// const DOWN: Direction = Direction::new(1, 0);
// const LEFT: Direction = Direction::new(0, -1);
// const UP: Direction = Direction::new(-1, 0);

// #[cfg(test)]
// mod local_tests {
//     use super::{DOWN, LEFT, RIGHT, UP};
//     #[test]
//     fn advance() {
//         assert_eq!(RIGHT.advance((0, 0)), (0, 1));
//         assert_eq!(DOWN.advance((0, 0)), (1, 0));
//         assert_eq!(LEFT.advance((0, 0)), (0, -1));
//         assert_eq!(UP.advance((0, 0)), (-1, 0));
//     }
// }

// pub fn spiral_matrix_step_by_step(size: u32) -> Vec<Vec<u32>> {
//     let size = size as usize;
//     let one_row_of_zeros = iter::repeat(0).take(size).collect::<Vec<u32>>(); // Without the following, debugger showed strange numbers
//     assert!(one_row_of_zeros.iter().all(|&value| value == 0));
//     let mut spiral = iter::repeat(one_row_of_zeros)
//         .take(size)
//         .collect::<Vec<_>>();

//     if size == 0 {
//         return spiral;
//     }

//     let mut pos: Position = (0, 0);
//     let mut dir = RIGHT;
//     let mut number = 1;
//     loop {
//         // First, put the number at the current place
//         *cell(&mut spiral, pos) = number;
//         number += 1;

//         if size == 1 {
//             // Nowhere to advance - we're done.
//             return spiral;
//         }

//         // If we can, continue in the current direction.
//         let pos_straight = dir.advance(pos);

//         pos = if within_square(pos_straight, size) && {
//             // Filled in already?
//             let pos_straight_cell = *cell(&mut spiral, pos_straight);
//             pos_straight_cell == 0
//         } {
//             pos_straight
//         } else {
//             // Otherwise turn.
//             dir = dir.turn_right();
//             let pos_turn = dir.advance(pos);
//             assert!(within_square(pos_turn, size));
//             pos_turn
//         };
//         if *cell(&mut spiral, pos) != 0 {
//             // All done.
//             break;
//         }
//     }

//     spiral
// }
