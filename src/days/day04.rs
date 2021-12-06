const ROWS: usize = 5;
const COLS: usize = 5;

#[derive(Debug)]
struct Board {
    data: Vec<u32>,
    zero_index: Option<usize>,
    row_sums: [u32; ROWS],
    col_sums: [u32; COLS],
}

impl Board {
    fn new(mut data: Vec<u32>) -> Self {
        assert!(data.len() == 25);

        let zero_index = data.iter().position(|&x| x == 0);
        if let Some(idx) = zero_index {
            data[idx] = 100;
        }

        let mut row_sums = [0; ROWS];
        for (row, row_sum) in data.chunks_exact(COLS).zip(row_sums.iter_mut()) {
            *row_sum = row.iter().sum();
        }

        let mut col_sums = [0; ROWS];
        for (idx, col_sum) in col_sums.iter_mut().enumerate() {
            *col_sum = data.iter().skip(idx).step_by(COLS).sum();
        }

        Self {
            data,
            zero_index,
            row_sums,
            col_sums,
        }
    }

    fn test_draw(&mut self, number: u32) -> Option<u32> {
        let number = if number == 0 { 100 } else { number };
        let r = self.data.iter().position(|&x| x == number)?;

        // Found
        self.data[r] = 0;
        self.row_sums[r / COLS] -= number;
        self.col_sums[r % COLS] -= number;
        if self.row_sums[r / COLS] == 0 || self.col_sums[r % COLS] == 0 {
            if let Some(idx) = self.zero_index {
                self.data[idx] = 0;
            }
            return Some(number * self.data.iter().sum::<u32>());
        }
        None
    }
}

fn find_answer(boards: &mut [Board], draws: &[u32]) -> Option<u32> {
    for draw in draws {
        for board in &mut *boards {
            if let Some(score) = board.test_draw(*draw) {
                return Some(score);
            }
        }
    }
    None
}

pub fn run() -> Option<u32> {
    let input = include_str!("data/day04.txt");
    let mut lines = input.split("\n\n");

    let draws = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let mut boards = lines
        .map(|board| {
            Board::new(
                board
                    .lines()
                    .flat_map(|line| {
                        line.split_whitespace()
                            .filter_map(|x| x.parse::<u32>().ok())
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<Board>>();

    find_answer(&mut boards, &draws)
}

#[test]
fn test_01() {
    let draws = vec![
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    let mut boards = vec![
        Board::new(vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ]),
        Board::new(vec![
            3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12,
            6,
        ]),
        Board::new(vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ]),
    ];

    assert_eq!(find_answer(&mut boards, &draws), Some(4512));
}
