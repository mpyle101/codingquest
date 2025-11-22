fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> usize
{
    use pathfinding::matrix::Matrix;
    use pathfinding::prelude::dijkstra;

    let maze = Matrix::from_rows(input.lines().map(|l| l.as_bytes())).unwrap();

    let row = maze.iter().next().unwrap();
    let col = row.iter().position(|c| **c == b' ').unwrap();
    let start = (0, col);

    let row = maze.iter().next_back().unwrap();
    let col = row.iter().position(|c| **c == b' ').unwrap();
    let goal = (maze.rows - 1, col);

    let path = dijkstra(&start,
        |pos| maze.neighbours(*pos, false).map(|p| (p, cost(maze.get(p)))),
        |pos| *pos == goal
    ).unwrap();

    path.0.len()
}

fn cost(item: Option<&&u8>) -> u32
{
    if **item.unwrap() == b' ' { 1 } else { 10000 }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 6723);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 71);
    }
}
