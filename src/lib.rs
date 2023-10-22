#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    True,
    False,
}

pub struct Picross {
    pub width: usize,
    pub height: usize,
    pub row_hints: Vec<Vec<usize>>,
    pub col_hints: Vec<Vec<usize>>,
    pub grid: Vec<Vec<Cell>>,
}

impl Picross {
    pub fn new(
        width: usize,
        height: usize,
        row_hints: Vec<Vec<usize>>,
        col_hints: Vec<Vec<usize>>,
    ) -> Picross {
        assert_eq!(row_hints.len(), height);
        assert_eq!(col_hints.len(), width);
        assert_eq!(
            row_hints
                .iter()
                .map(|v| v.iter().sum::<usize>())
                .sum::<usize>(),
            col_hints
                .iter()
                .map(|v| v.iter().sum::<usize>())
                .sum::<usize>()
        );
        Picross {
            width,
            height,
            row_hints,
            col_hints,
            grid: vec![vec![Cell::Empty; width]; height],
        }
    }

    pub fn solve(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..self.height {
                changed |= self.solve_row(i);
            }
            for i in 0..self.width {
                changed |= self.solve_col(i);
            }
        }
    }
    fn dfs(
        hints: &[usize],
        hint_idx: usize,
        start: usize,
        possibilities: &mut Vec<Vec<Cell>>,
        possibility: Vec<Cell>,
    ) {
        if hints.is_empty() {
            possibilities.push(vec![Cell::False; possibility.len()]);
            return;
        }
        if hint_idx == hints.len() || start == possibility.len() {
            possibilities.push(possibility);
            return;
        }
        let mut skip = if hint_idx == 0 { 0 } else { 1 };
        'outer: loop {
            let end = start + hints[hint_idx] + skip;
            if end > possibility.len() {
                return;
            }
            if end < possibility.len() && possibility[end] == Cell::True {
                skip += 1;
                continue;
            }
            let mut new_possibility = possibility.clone();
            for cell in new_possibility.iter_mut().skip(start).take(skip) {
                if *cell == Cell::True {
                    skip += 1;
                    continue 'outer;
                }
                *cell = Cell::False;
            }
            for cell in new_possibility.iter_mut().take(end).skip(start + skip) {
                if *cell == Cell::False {
                    skip += 1;
                    continue 'outer;
                }
                *cell = Cell::True;
            }
            Self::dfs(hints, hint_idx + 1, end, possibilities, new_possibility);
            skip += 1;
        }
    }

    fn solve_row(&mut self, row: usize) -> bool {
        let mut possibilities = vec![];

        Self::dfs(
            &self.row_hints[row],
            0,
            0,
            &mut possibilities,
            self.grid[row].clone(),
        );
        if let Some(possibility) = common_possibility(&possibilities) {
            let changed = possibility != self.grid[row];
            self.grid[row] = possibility;
            changed
        } else {
            false
        }
    }

    fn solve_col(&mut self, col: usize) -> bool {
        let mut possibilities = vec![];

        Self::dfs(
            &self.col_hints[col],
            0,
            0,
            &mut possibilities,
            take_col(&self.grid, col),
        );
        if let Some(possibility) = common_possibility(&possibilities) {
            let mut changed = false;
            for (row, cell) in possibility.iter().enumerate() {
                if self.grid[row][col] == *cell {
                    continue;
                }
                self.grid[row][col] = *cell;
                changed = true;
            }
            changed
        } else {
            false
        }
    }
}

impl ToString for Picross {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for row in self.grid.iter() {
            for &cell in row.iter() {
                match cell {
                    Cell::Empty => s.push('.'),
                    Cell::True => s.push('⬜'),
                    Cell::False => s.push('X'),
                }
            }
            s.push('\n');
        }
        s
    }
}

fn take_col<T: Copy>(v: &[Vec<T>], col: usize) -> Vec<T> {
    v.iter().map(|row| row[col]).collect::<Vec<_>>()
}

fn common_possibility(possibilities: &[Vec<Cell>]) -> Option<Vec<Cell>> {
    if possibilities.is_empty() {
        return None;
    }
    let mut possibility = vec![Cell::Empty; possibilities[0].len()];
    for (i, cell) in possibility.iter_mut().enumerate() {
        let first_cell = possibilities[0][i];
        if possibilities.iter().all(|p| p[i] == first_cell) {
            *cell = first_cell;
        }
    }
    Some(possibility)
}

#[cfg(test)]
mod test {
    use super::*;
    use Cell::*;
    #[test]
    fn test_take_col() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(take_col(&v, 0), vec![1, 4]);
        assert_eq!(take_col(&v, 1), vec![2, 5]);
        assert_eq!(take_col(&v, 2), vec![3, 6]);
    }
    #[test]
    fn test_common_possibility() {
        let possibilities = vec![
            vec![True, True, False, Empty],
            vec![True, False, False, Empty],
            vec![True, True, False, Empty],
        ];
        assert_eq!(
            common_possibility(&possibilities),
            Some(vec![True, Empty, False, Empty])
        );
        let possibilities = vec![vec![True, True, False]];
        assert_eq!(
            common_possibility(&possibilities),
            Some(vec![True, True, False])
        );
        let possibilities = vec![];
        assert_eq!(common_possibility(&possibilities), None);
    }
    #[test]
    fn test_solve_full_cell() {
        let width = 5;
        let height = 5;
        let row_hints = vec![vec![5], vec![5], vec![5], vec![5], vec![5]];
        let col_hints = vec![vec![5], vec![5], vec![5], vec![5], vec![5]];
        let mut picross = Picross::new(width, height, row_hints, col_hints);
        picross.solve();
        assert_eq!(picross.grid, vec![vec![True; 5]; 5]);
        assert_eq!(
            picross.to_string(),
            "⬜⬜⬜⬜⬜\n⬜⬜⬜⬜⬜\n⬜⬜⬜⬜⬜\n⬜⬜⬜⬜⬜\n⬜⬜⬜⬜⬜\n"
        );
    }
    #[test]
    fn test_solve_empty_cell() {
        let width = 5;
        let height = 6;
        let row_hints = vec![vec![]; 6];
        let col_hints = vec![vec![]; 5];
        let mut picross = Picross::new(width, height, row_hints, col_hints);
        picross.solve();
        assert_eq!(picross.grid, vec![vec![False; 5]; 6]);
        assert_eq!(
            picross.to_string(),
            "XXXXX\nXXXXX\nXXXXX\nXXXXX\nXXXXX\nXXXXX\n"
        );
    }
    #[test]
    fn test_sample() {
        let width = 5;
        let height = 5;
        let row_hints = vec![vec![4], vec![1, 1], vec![3], vec![2, 2], vec![1, 2]];
        let col_hints = vec![vec![1, 1], vec![1, 2], vec![3], vec![1, 3], vec![1, 3]];
        let mut picross = Picross::new(width, height, row_hints, col_hints);
        picross.solve();
        assert_eq!(
            picross.to_string(),
            "X⬜⬜⬜⬜\n⬜X⬜XX\nXX⬜⬜⬜\n⬜⬜X⬜⬜\nX⬜X⬜⬜\n"
        );
    }

    #[test]
    fn test_dfs() {
        let mut possibilities = vec![];
        Picross::dfs(
            &[3],
            0,
            0,
            &mut possibilities,
            vec![Empty, Empty, True, True, True],
        );
        assert_eq!(possibilities, vec![vec![False, False, True, True, True]]);
    }
}
