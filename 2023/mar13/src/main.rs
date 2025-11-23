fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{result:?} ({:?})", t.elapsed());
}

fn doit(input: &str) -> (usize, usize)
{
    use pathfinding::matrix::Matrix;

    let asteroids = input.lines()
        .map(|line| line.split_whitespace())
        .map(|mut iter| {
            let px = iter.next().unwrap().parse::<i32>().unwrap();
            let py = iter.next().unwrap().parse::<i32>().unwrap();
            let dx = iter.next().unwrap().parse::<i32>().unwrap();
            let dy = iter.next().unwrap().parse::<i32>().unwrap();

            let x = px + (dx * 3600);
            let y = py + (dy * 3600);
            (x, y, dx, dy)
        })
        .collect::<Vec<_>>();

    let mut space = Matrix::new(100, 100, 0u8);
    let mut rocks = asteroids.iter()
        .inspect(|r| {
            let pos = (r.1 as usize, r.0 as usize);
            if r.2 == 0 && r.3 == 0 { *space.get_mut(pos).unwrap() = 1}
        })
        .filter(|r| r.2 != 0 || r.3 != 0 )
        .cloned()
        .collect::<Vec<_>>();

    (0..60).for_each(|_|
        rocks.iter_mut()
            .for_each(|(x, y, dx, dy)| {
                if (0..100).contains(x) && (0..100).contains(y) {
                    let pos = (*y as usize, *x as usize);
                    *space.get_mut(pos).unwrap() = 1;
                }

                *x += *dx;
                *y += *dy;
            })
    );

    let open = space.items()
        .filter(|(_, v)| **v == 0)
        .map(|(p, _)| p)
        .collect::<Vec<_>>();

    (open[0].1, open[0].0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), (5, 8));
    }
}
