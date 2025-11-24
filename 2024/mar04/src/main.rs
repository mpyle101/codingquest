fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> i32
{
    use std::collections::HashMap;

    let cost = input.lines()
        .fold(HashMap::<&str, i32>::new(), |mut m, line| {
            let (company, s) = line.split_once(": ").unwrap();
            let (category, s) = s.split_once(' ').unwrap();
            let n = s.parse::<i32>().unwrap();
            let v = if category == "Discount" || category == "Rebate" { -n } else { n };

            *m.entry(company).or_default() += v;
            m
        });

    *cost.values().min().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 191274);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 2533);
    }
}
