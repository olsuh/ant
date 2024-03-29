use crate::{sum_digits, Int};
use std::collections::HashMap;

type AntPos = (i32, i32);
const NULL_POS: AntPos = (0, 0);
#[derive(Default)]
struct Cells {
    visited: HashMap<AntPos, Cell>,
    ant_pos: AntPos,
}

impl Cells {
    fn new(ant_pos: AntPos) -> Self {
        let mut cells = Self {
            visited: Default::default(),
            ant_pos,
        };
        cells.add_cell(ant_pos, NULL_POS);
        cells
    }
    fn add_cell(&mut self, ant_pos: AntPos, prev_pos: AntPos) {
        let cell = Cell::new(ant_pos, prev_pos, &self.visited);
        self.visited.insert(ant_pos, cell);
        self.ant_pos = ant_pos;
    }
    fn run(&mut self) -> usize {
        while let Some(cell) = self.visited.get_mut(&self.ant_pos) {
            if let Some(next_pos) = cell.next() {
                self.add_cell(next_pos, self.ant_pos);
                continue;
            } else if cell.prev_pos == NULL_POS {
                break;
            };
            self.ant_pos = cell.prev_pos;
        }
        self.visited.len()
    }
}

fn available_moves(ant_pos: &AntPos, visited: &HashMap<AntPos, Cell>) -> Vec<AntPos> {
    const ARR_MOVES: [AntPos; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let available_moves: Vec<_> = ARR_MOVES
        .iter()
        .map(|(dx, dy)| (ant_pos.0 + dx, ant_pos.1 + dy))
        .filter(|(x, y)| {
            !visited.contains_key(&(*x, *y)) && sum_digits(*x as Int) + sum_digits(*y as Int) <= 25
        })
        .collect();
    available_moves

    /*
    let mut arr_available_moves = [(0,0);4];
    let mut i = 0;
    for (dx, dy) in ARR_MOVES {
        let next_x = ant_pos.0 + dx;
        let next_y = ant_pos.1 + dy;
        if !visited.contains(&(next_x,next_y)) && sum_digits(next_x as Int) + sum_digits(next_y as Int) <= 25 {
            arr_available_moves[i] = (next_x,next_y);
            i += 1;
        }
    }

    arr_available_moves
    */
}

struct Cell {
    available_moves: Vec<AntPos>,
    prev_pos: AntPos,
}

impl Cell {
    fn new(ant_pos: AntPos, prev_pos: AntPos, visited: &HashMap<AntPos, Cell>) -> Self {
        let available_moves = available_moves(&ant_pos, visited);
        Cell {
            available_moves,
            prev_pos,
        }
    }
    fn next(&mut self) -> Option<AntPos> {
        self.available_moves.pop()
    }
}

pub fn main() -> f32 {
    let start_time = std::time::Instant::now();

    let mut cells = Cells::new((1000, 1000));
    let cells_cnt = cells.run();

    let elapsed = start_time.elapsed().as_secs_f32();
    println!("cells: {cells_cnt}, elapsed: {elapsed:?} - data: hashmap + engine: array_tuple_dx_dy");
    elapsed
}
