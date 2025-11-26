use pathfinding::matrix::Matrix;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{:?} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> usize
{
    use pathfinding::prelude::dijkstra;

    let (s1, s2) = input.split_once("\n\n").unwrap();
    let top = Matrix::from_rows(s1.lines().map(|l| l.chars())).unwrap();
    let bot = Matrix::from_rows(s2.lines().map(|l| l.chars())).unwrap();

    let start = (1, 0, 1);
    let goal  = (top.rows - 2, top.columns - 1, 1);

    let path = dijkstra(&start,
        |pos| neighbours(pos, &top, &bot),
        |pos| *pos == goal
    ).unwrap();

    //println!("{path:?}");

    path.1
}

// row, col, layer
type Pos = (usize, usize, usize);

fn neighbours(pos: &Pos, top: &Matrix<char>, bot: &Matrix<char>) -> Vec<(Pos, usize)>
{
    let layer = if pos.2 == 1 { top } else { bot };
    let mut v = layer.neighbours((pos.0, pos.1), false)
        .filter(|&p| {
            let c = *layer.get(p).unwrap();
            c == '.' || c == '$'
        })
        .map(|p| ((p.0, p.1, pos.2), 1))
        .collect::<Vec<_>>();

    if *layer.get((pos.0, pos.1)).unwrap() == '$' {
        v.push(((pos.0, pos.1, (pos.2 + 1) % 2), 0));
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 250);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 52);
    }
}
