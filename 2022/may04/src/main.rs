fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> i64
{
    let tour: Vec<_> = input.lines()
        .map(|line| line.split(' ')
            .flat_map(|s| s.parse::<i64>())
            .collect::<Vec<_>>()
        )
        .collect();

    tour.windows(2)
        .map(|w| {
            let dx = w[0][0].abs_diff(w[1][0]).pow(2);
            let dy = w[0][1].abs_diff(w[1][1]).pow(2);
            let dz = w[0][2].abs_diff(w[1][2]).pow(2);
            ((dx + dy + dz) as f64).sqrt() as i64
        })
        .sum::<i64>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 64579603);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 85);
    }
}
