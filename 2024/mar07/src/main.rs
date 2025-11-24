fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> f32
{
    use std::cmp::Ordering;

    let stars = input.lines()
        .skip(1)
        .map(|line| {
            let name = line[0..30].trim_end();
            let mut iter = line[31..].split_whitespace();
            iter.next();    // skip Dist
            let x = iter.next().unwrap().parse::<f32>().unwrap();
            let y = iter.next().unwrap().parse::<f32>().unwrap();
            let z = iter.next().unwrap().parse::<f32>().unwrap();
            (name, x, y, z)
        })
        .collect::<Vec<_>>();

    let mut dist = vec![];
    for i in 0..stars.len() - 1 {
        for j in i+1..stars.len() {
            let a = stars[i];
            let b = stars[j];

            let d = (a.1 - b.1).powi(2) + (a.2 - b.2).powi(2) + (a.3 - b.3).powi(2);
            dist.push(d.sqrt());
        }
    }
    
    *dist.iter()
        .min_by(|a, b| if a < b { Ordering::Less } else { Ordering::Greater })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 1.0993308);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 3.5078201);
    }
}
