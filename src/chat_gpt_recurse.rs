use std::collections::HashSet;

fn sum_digits(n: i32) -> u32 {
    n.to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .sum::<u32>()
}

type Ant = (i32,i32);

fn available_moves(ant_pos: &Ant, visited: &HashSet<Ant>) -> [Ant;4] {

    const ARR_MOVES: [Ant;4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut arr_available_moves = [(0,0);4];

    /*let available_moves = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    .iter()
    .map(|(dx, dy)| (ant_pos.0 + dx, ant_pos.1 + dy))
    .filter(|(x, y)| sum_digits(*x) + sum_digits(*y) <= 25 && !visited.contains(&(*x, *y)))
    //.collect()
    ;*/
    let mut i = 0;
    for (dx, dy) in ARR_MOVES {
        let next_x = ant_pos.0 + dx;
        let next_y = ant_pos.1 + dy;
        if !visited.contains(&(next_x,next_y)) && sum_digits(next_x) + sum_digits(next_y) <= 25 {
            arr_available_moves[i] = (next_x,next_y);
            i += 1;
        }
    }

    arr_available_moves

}

pub fn next(ant_pos: &Ant, visited: &mut HashSet<Ant>) {

    if visited.contains(ant_pos) {
        return;
    }

    visited.insert(*ant_pos);
    //println!("{} {ant_pos:?}",visited.len());

    let available_moves = available_moves(ant_pos, visited);

    for ant_i in available_moves {
        if ant_i.0 == 0 {break;}
        next(&ant_i, visited);
    }
}



pub fn main() {
    let start_time = std::time::Instant::now();

    let mut visited = std::collections::HashSet::new();
    let ant_pos = (1000, 1000);
    next(&ant_pos, &mut visited);


    let elapsed_time = start_time.elapsed().as_secs_f32();
    println!("Муравей посетил {} клеток за {}", visited.len(), elapsed_time);
}