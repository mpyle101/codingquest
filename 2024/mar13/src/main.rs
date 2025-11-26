use num_bigint::BigUint;

fn main()
{
    use std::time::Instant;

    let t = Instant::now();
    let result = doit::<856>(&[1, 2, 12, 40]);
    println!("{:?} ({:?})", result, t.elapsed());
}

fn doit<const N: usize>(r: &[usize]) -> BigUint
{
    let one = BigUint::new(vec![1]);
    let mut v = vec![BigUint::ZERO;N+1];
    r.iter().for_each(|&i| v[i] += one.clone());

    for i in 1..N {
        r.iter()
            .filter(|&j| i + j <= N)
            .for_each(|&j| {
                let n = v[i].clone();
                v[i+j] += n;
            })
    }

    //println!("{v:?}");
    
    v[N].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        assert_eq!(doit::<15>(&[1, 2, 12]), BigUint::new(vec![997]));
    }

    #[test]
    fn example_doit()
    {
        assert_eq!(doit::<5>(&[1,2,3]), BigUint::new(vec![13]));
    }
}
