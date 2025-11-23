fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit::<20>(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit<const D: i32>(input: &str) -> u32
{
    let mut iter = input.lines();
    iter.next();    // fruit
    let mut fruit = iter.next()
        .map_or(vec![], |line|
            line.split_ascii_whitespace()
                .map(|s| {
                    let (s1, s2) = s.split_once(',').unwrap();
                    let x = s1.parse::<i32>().unwrap();
                    let y = s2.parse::<i32>().unwrap();
                    (x, y)
                })
                .rev()
                .collect::<Vec<_>>()
        );
    iter.next();    // moves
    let moves = iter.next().unwrap();

    let mut score = 0;
    let mut snake = vec![(0, 0)];

    for c in moves.chars() {
        let (x, y) = snake[0];
        let (dx, dy) = match c {
            'L' => (x - 1, y),
            'R' => (x + 1, y),
            'U' => (x, y - 1),
            'D' => (x, y + 1),
             _  => panic!()
        };

        if (0..D).contains(&dx) && (0..D).contains(&dy) && !snake.contains(&(dx, dy)) {
            score += 1;
            snake.insert(0, (dx, dy));
            if let Some(pos) = fruit.last() && *pos == (dx, dy) {
                score += 100;
                fruit.pop();
            } else {
                snake.pop();
            }
        } else {
            break;
        }
    }

    score
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit::<20>(input), 4240);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit::<8>(input), 320);
    }
}
