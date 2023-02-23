use std::{collections::HashSet, time::Instant};
type Int = usize;
enum Step {
    Start,
    Left,
    Down,
    Right,
    Up,
    Stop,
}
//#[derive(Hash)]
struct Cell {
    x: Int,
    y: Int,
    step: Step,
}
impl Cell {
    fn new(x: Int, y: Int) -> Self {
        Self { x, y, step: Step::Start }
    }
    
    fn next(&mut self, added_cells: &HashSet<(Int,Int)>) -> Option<(Int, Int)> {

        loop {
            let (step, x, y) = match self.step {
                Step::Start => (Step::Left, self.x-1, self.y),
                Step::Left =>  (Step::Down, self.x,   self.y-1),
                Step::Down => (Step::Right, self.x+1, self.y),
                Step::Right =>   (Step::Up, self.x,   self.y+1),
                Step::Up => { self.step = Step::Stop; return None },
                Step::Stop => { return None },
            };
    
            self.step = step;
            if added_cells.contains(&(x, y)) {
                continue;
            }
            let sum = sum_digits(x) + sum_digits(y);
            if sum <= 25 {
                return Some((x, y));
            };
        }
    }
}

pub fn sum_digits(x: Int) -> Int {
    let mut sum = 0;
    let mut quot = x;
    while quot > 0 {
        let rem = quot % 10;
        sum += rem;
        quot /= 10;
    }
    sum
}

struct Cells {
    cells: Vec<Cell>,
    pos: usize, //без Option<usize>
    added_cells: HashSet<(Int,Int)>,
}
impl Cells {
    fn new() -> Self {
        Self {  cells: Vec::<Cell>::new(), pos: 0, added_cells: HashSet::new() }
    }
    fn add(&mut self, x: Int, y: Int) {
        let cell = Cell::new(x, y);
        self.cells.push(cell);
        self.pos = self.cells.len();
        self.added_cells.insert((x, y));
    }
    fn run(&mut self, x: Int, y: Int) -> usize {
        self.add(x, y);

        loop {
            let cell = &mut self.cells[self.pos-1];

            if let Some((x, y)) = cell.next(&self.added_cells) {
                self.add(x, y);
            } else {
                if self.pos > 1 {
                    self.pos -= 1;
                } else {
                    break;
                }
            }
            
        }
        self.cells.len()
    }
    
}

mod hashmap_array_tuple_dx_dy;
mod chat_gpt_recursion;

fn main() {
    
    let start = Instant::now();
    let mut cells = Cells::new();
    let cells_cnt = cells.run(1000, 1000);
    let elapsed = start.elapsed().as_secs_f32();
    println!("cells: {cells_cnt}, elapsed: {elapsed:?} - data: vector_hashset + engine: enum_match");

    let elapsed2 = hashmap_array_tuple_dx_dy::main();
    println!("excellence: {:?}", elapsed2/elapsed);

    chat_gpt_recursion::main();
}

