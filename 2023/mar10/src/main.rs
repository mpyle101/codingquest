fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> String
{
    use pathfinding::matrix::Matrix;

    let mut screen = Matrix::new(10, 50, false);
    input.lines()
        .map(|line| line.split_whitespace())
        .map(|mut iter| {
            let lt = iter.next().unwrap().parse::<usize>().unwrap();
            let tp = iter.next().unwrap().parse::<usize>().unwrap();
            let wd = iter.next().unwrap().parse::<usize>().unwrap();
            let ht = iter.next().unwrap().parse::<usize>().unwrap();

            // rows, cols
            (tp..tp + ht, lt..lt + wd)
        })
        .for_each(|(rows, cols)| {
            rows.for_each(|r|
                cols.clone().for_each(|c| {
                    let v = *screen.get((r, c)).unwrap();
                    *screen.get_mut((r, c)).unwrap() = !v;
                })
            );
        });

    screen.iter()
        .for_each(|row| {
            row.iter().for_each(|v| if *v { print!("#") } else { print!(".")});
            println!();
        });
    
    "Done".into()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), "Done".to_string());
    }
}
