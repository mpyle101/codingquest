fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

#[derive(Clone, Copy, Debug)]
struct Node {
    value: u64,
    left: Option<usize>,
    right: Option<usize>,
    depth: u32,
}

fn doit(input: &str) -> u32
{
    use std::collections::HashMap;

    let nums = input.lines()
        .flat_map(|line| u64::from_str_radix(line, 16))
        .collect::<Vec<_>>();

    let mut tree = Vec::with_capacity(nums.len());
    let root = Node { left: None, right: None, depth: 1, value: nums[0]};
    tree.push(root);
    nums.iter().skip(1).for_each(|n| insert(&mut tree, *n));

    let mut depth = 0;
    let widths = tree.iter()
        .fold(HashMap::new(), |mut m, n| {
            depth = n.depth.max(depth);
            *m.entry(n.depth).or_default() += 1;
            m
        });
    let width = widths.values().max().unwrap();

    depth * width
}

fn insert(tree: &mut Vec<Node>, n: u64)
{
    let mut i = 0;
    let mut slot = Some(0);

    while let Some(idx) = slot {
        i = idx;
        if n < tree[i].value {
            slot = tree[i].left
        } else {
            slot = tree[i].right;
        }
    }

    let node = Node { left: None, right: None, depth: tree[i].depth + 1, value: n };
    let idx = tree.len();
    tree.push(node);

    if n < tree[i].value {
        tree[i].left = Some(idx)
    } else {
        tree[i].right = Some(idx)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 30784);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 16);
    }
}
