fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> u32
{
    let v: Vec<_> = input.lines()
        .flat_map(|s| s.parse::<u32>())
        .collect();

    v.windows(60)
        .map(|w| w.iter().sum::<u32>() / w.len() as u32)
        .filter(|&n| !(1500..=1600).contains(&n))
        .count() as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 6248);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 110);
    }
}
