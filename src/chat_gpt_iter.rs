use crate::{sum_digits, Int};

pub fn main() {
    let start_time = std::time::Instant::now(); // засекаем время начала работы

    let mut visited = std::collections::HashSet::new(); // множество посещенных клеток
    visited.insert((1000, 1000)); // добавляем начальную позицию муравья

    let mut ant_pos = (1000, 1000); // текущая позиция муравья
    /*let sum_digits = |n:i32| { // функция для подсчета суммы цифр числа
        n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
    };*/
    let mut rem = Vec::new();

    loop {
        // перемещаем муравья в новую клетку, выбирая доступные направления
        let mut available_moves: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .map(|(dx, dy)| (ant_pos.0 + dx, ant_pos.1 + dy))
            .filter(|(x, y)| !visited.contains(&(*x, *y)) && sum_digits(*x as Int) + sum_digits(*y as Int) <= 25 )
            .collect();

        if available_moves.is_empty() {
            match rem.pop() {
                Some(a_m) => available_moves.push(a_m),
                None => break, // если нет доступных клеток, выходим из цикла
            }
        }

        ant_pos = available_moves.pop().unwrap(); // перемещаем муравья в первую доступную клетку
        rem.append(&mut available_moves);
        visited.insert(ant_pos); // добавляем ее в множество посещенных клеток
    }

    let elapsed_time = start_time.elapsed().as_secs_f32(); // измеряем время работы
    println!("Муравей посетил {} клеток за {:?}", visited.len(), elapsed_time);
}
