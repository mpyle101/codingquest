fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit::<20000, 100000>(input);
    println!("{} ({:?})", result, t.elapsed());
}

type Rect = (u32, u32, u32, u32);

fn doit<const W: u32, const H: u32>(input: &str) -> u32
{
    let mut shields = input.lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let l = iter.next().unwrap().parse::<u32>().unwrap();
            let t = iter.next().unwrap().parse::<u32>().unwrap();
            let w = iter.next().unwrap().parse::<u32>().unwrap();
            let h = iter.next().unwrap().parse::<u32>().unwrap();

            // top, left, right, bottom
            (t, l, l + w - 1, t + h - 1)
        })
        .collect::<Vec<_>>();
    shields.sort();

    let mut area = 0;
    let mut active: Vec<Rect> = vec![];
    let mut y = shields[0].0;
    while y < H {
        active = active.iter()
            .filter(|sh| y <= sh.3)
            .copied()
            .collect();
        while !shields.is_empty() && shields[0].0 == y {
            active.push(shields.remove(0))
        }
        active.sort_by_key(|sh| sh.1);

        if !active.is_empty() {
            let (_, l, r, _) = active[0];
            let mut merged = vec![(l, r)];
            active.iter()
                .skip(1)
                .for_each(|&(_, l, r, _)| {
                    let last = merged.pop().unwrap();
                    if l > last.1 {
                        merged.push(last);
                        merged.push((l, r));
                    } else {
                        merged.push((last.0, last.1.max(r)));
                    }
                });
            area += merged.iter()
                .map(|(l, r)| r - l + 1)
                .sum::<u32>();
            y += 1;
        } else if !shields.is_empty() {
            y = shields[0].0;
        } else {
            break;
        }
    }

    W * H - area
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit::<20000, 100000>(input), 21630678);
    }

    #[test]
    fn example1_doit()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(doit::<10, 10>(input), 12);
    }

    #[test]
    fn example2_doit()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(doit::<100, 100>(input), 2061);
    }
}
