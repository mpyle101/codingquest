fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input, "TYC", "EAR", 600);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str, start: &str, end: &str, delay: u32) -> u32
{
    use std::collections::HashMap;
    use pathfinding::prelude::dijkstra;

    let beacons: HashMap<_,_> = input.lines()
        .map(|line| {
            let (a, s) = line.split_once(" => ").unwrap();
            let v: Vec<_> = s.split_whitespace()
                .map(|s| {
                    let (b, n) = s.split_once(':').unwrap();
                    (b.to_string(), n.parse::<u32>().unwrap() + delay)
                })
                .collect();
            (a.to_string(), v)
        })
        .collect();

    let goal = end.to_string();
    let path = dijkstra(&start.to_string(),
        |b| beacons.get(b).unwrap().clone(),
        |b| *b == goal
    ).unwrap();

    path.1 - delay
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input, "TYC", "EAR", 600), 165127);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input, "AAA", "ZZZ", 10), 115);
    }
}
