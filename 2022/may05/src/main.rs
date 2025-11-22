use grid::Grid;

type Board = Grid<u8>;

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
    let mut wins = [0;3];

    input.lines()
        .flat_map(|line| play(line.as_bytes()))
        .for_each(|p| wins[p] += 1);
    wins.iter().map(|n| *n as u32).product()
}

fn play(moves: &[u8]) -> Option<usize>
{
    use grid::Order::ColumnMajor;

    let mut board = Grid::init_with_order(7, 7, ColumnMajor, b'.');

    let mut p = 0;
    for &m in moves.iter().take(9) {
        drop(&mut board, m - b'1', p);
        p = (p + 1) % 3;
    }
    for &m in moves.iter().skip(9) {
        drop(&mut board, m - b'1', p);
        p = (p + 1) % 3;
        if let Some(player) = winner(&board) {
            return Some(player);
        }
    }

    None
}

#[allow(dead_code)]
fn print(board: &Board)
{
    board.iter_rows()
        .for_each(|r| {
            r.for_each(|v| if *v == b'.' {
                print!(". ");
            } else {
                print!("{} ", *v)
            });
            println!();
        });
    println!();
}

fn drop(board: &mut Board, slot: u8, player: u8)
{
    for c in board.iter_col_mut(slot as usize).rev() {
        if *c == b'.' {
            *c = player;
            break;
        }
    }
}

fn winner(board: &Board) -> Option<usize>
{
    check_horz(board)
        .or(check_vert(board))
        .or(check_fwd(board))
        .or(check_bwd(board))
}

fn check_horz(board: &Board) -> Option<usize>
{
    for c in 0..7 {
        for r in 0..4 {
            let mut iter = board.iter_col(c).skip(r);
            let player = *iter.next().unwrap();
            if player != b'.' && iter.take(3).all(|p| *p == player) {
                return Some(player as usize)
            }
        }
    }

    None
}

fn check_vert(board: &Board) -> Option<usize>
{
    for r in 0..7 {
        for c in 0..4 {
            let mut iter = board.iter_row(r).skip(c);
            let player = *iter.next().unwrap();
            if player != b'.' && iter.take(3).all(|p| *p == player) {
                return Some(player as usize)
            }
        }
    }

    None
}

fn check_fwd(board: &Board) -> Option<usize>
{
    let mut slots = [b'.';4];

    for c in 0..4 {
        for r in (3..7).rev() {
            (0..4).for_each(|n| slots[n] = *board.get(r-n, c+n).unwrap());
            let player = slots[0];
            if player != b'.' && slots.iter().all(|p| *p == player) {
                return Some(player as usize)
            }
        }
    }

    None
}

fn check_bwd(board: &Board) -> Option<usize>
{
    let mut slots = [b'.';4];

    for c in (3..7).rev() {
        for r in (3..7).rev() {
            (0..4).for_each(|n| slots[n] = *board.get(r-n, c-n).unwrap());
            let player = slots[0];
            if player != b'.' && slots.iter().all(|p| *p == player) {
                return Some(player as usize)
            }
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 21630678);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 6);
    }
}
