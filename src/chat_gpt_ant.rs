fn main() {
    let start_time = std::time::Instant::now(); // засекаем время начала работы

    let mut visited = std::collections::HashSet::new(); // множество посещенных клеток
    visited.insert((1000, 1000)); // добавляем начальную позицию муравья

    let mut ant_pos = (1000, 1000); // текущая позиция муравья
    let mut sum_digits = |n| { // функция для подсчета суммы цифр числа
        n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
    };

    loop {
        // перемещаем муравья в новую клетку, выбирая доступные направления
        let available_moves: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .map(|(dx, dy)| (ant_pos.0 + dx, ant_pos.1 + dy))
            .filter(|(x, y)| sum_digits(*x) + sum_digits(*y) <= 25 && !visited.contains(&(x, y)))
            .collect();

        if available_moves.is_empty() {
            break; // если нет доступных клеток, выходим из цикла
        }

        ant_pos = available_moves[0]; // перемещаем муравья в первую доступную клетку
        visited.insert(ant_pos); // добавляем ее в множество посещенных клеток
    }

    let elapsed_time = start_time.elapsed(); // измеряем время работы
    println!("Муравей посетил {} клеток за {:?}", visited.len(), elapsed_time);
}