fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit::<12>(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit<const N:u32>(input: &str) -> u32
{
    use std::collections::HashMap;

    let distances = input.lines()
        .enumerate()
        .fold(HashMap::new(), |mut m, (a, line)| {
            line.split_whitespace()
                .enumerate()
                .filter(|(b, _)| *b != a)
                .for_each(|(b, s)| {
                    let key: u32 = 1 << a | 1 << b;
                    m.insert(key, s.parse::<u32>().unwrap());
                });
            m
        });

    let mut trips = HashMap::new();
    (1..N).for_each(|n| {
        let last: u32 = 1 << (15 + n);
        let trip: u32 = 0x01 | 1 << n;
        let dist = distances.get(&trip).unwrap();
        trips.insert(last | trip, *dist);
    });
    for _ in 2..N {
        trips = trips.iter()
            .fold(HashMap::new(), |mut m, (k, d)| {
                let last = k >> 15;
                let visited = k & 0xFFFF;

                (1..N).for_each(|n| {
                    let city = 1 << n;
                    if (city & visited) == 0 {
                        let trip = last | city;
                        let dist = *distances.get(&trip).unwrap();
                        let key  = (city << 15) | visited | city;
                        let dst  = d + dist;


                        if let Some(v) = m.get(&key) {
                            if dst < *v { m.insert(key, dst); }
                        } else {
                            m.insert(key, dst);
                        }
                    }
                });

                m
            });
    }

    trips.iter()
        .map(|(k, d)| {
            let trip = (k >> 15) | 0x01;
            d + distances.get(&trip).unwrap()
        })
        .min()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit::<12>(input), 7524);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit::<5>(input), 43);
    }
}
