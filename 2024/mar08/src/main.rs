fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{:?} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> u32
{
    use std::collections::HashMap;

    let (outposts, routes) = input.split_once("\n\n").unwrap();
    let mut iter = outposts.lines();
    let names = iter.next().unwrap().split_whitespace().collect::<Vec<_>>();

    let distances = iter.fold(HashMap::new(), |mut m, line| {
        let mut it = line.split_whitespace();
        let outpost = it.next().unwrap();

        names.iter().zip(it)
            .for_each(|(n, s)| {
                let d = s.parse::<u32>().unwrap();
                m.insert((outpost, *n), d);
            });
        m
    });

    let routes = routes.lines()
        .map(|line| {
            let (_, s) = line.split_once(": ").unwrap();
            s.split(" -> ").collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    routes.iter()
        .map(|r| {
            r.windows(2)
                .map(|w| distances.get(&(w[0], w[1])).unwrap())
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 6979629);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 353480);
    }
}
