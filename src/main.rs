//
// src/main.rs
//
use std::io;
use std::io::Write;

/// Representation of a 'move' of a disk from one peg to another
struct Move<'a> {
    /// Label for the source peg
    src: &'a str,
    /// Label for the destination peg
    dst: &'a str,
}

/// Solve the Tower of Hanoi problem
fn main() {
    let mut num_disks = 0;
    let mut lst_moves: Vec<Move> = Vec::new();

    get_num_disks(&mut num_disks);

    if num_disks >= 0 {
        println!("Number of disks = {}", num_disks);
        move_disk_set(num_disks, "A", "B", "C", &mut lst_moves);
        println!("Number of moves = {}", lst_moves.len());
        for (pos, m) in lst_moves.iter().enumerate() {
            println!("{:>10}: {} -> {}", pos + 1, m.src, m.dst);
        }
    } else {
        println!("Invalid number ({}) of disks!", &num_disks);
    }
}

/// Get the number of disks to solve the problem for
///
/// * `k` - Buffer to populate the result in
fn get_num_disks(k: &mut i64) {
    let mut buf = String::new();
    print!("Enter the number of disks: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input!");
    *k = buf
        .trim()
        .parse::<i64>()
        .expect("Failed to parse input as an integer!");
}

/// Move a tower of the given size from source to destination.
///
/// * `num_disks` - Number of disks in the tower
/// * `src`       - Label for the source peg
/// * `dst`       - Label for the destination peg
/// * `buf`       - Label for the peg acting as a buffer
/// * `lst_moves` - List of moves executed so far
fn move_disk_set<'a>(
    num_disks: i64,
    src: &'a str,
    dst: &'a str,
    buf: &'a str,
    lst_moves: &mut Vec<Move<'a>>,
) {
    match num_disks {
        0 => return,
        n => {
            move_disk_set(n - 1, src, buf, dst, lst_moves);
            move_disk(lst_moves, src, dst);
            move_disk_set(n - 1, buf, dst, src, lst_moves);
        }
    }
}

/// Move a single disk from source to destination.
///
/// * `lst_moves` - List of moves executed so far
/// * `src`       - Label for the source peg
/// * `dst`       - Label for the destination peg
fn move_disk<'a>(
    lst_moves: &mut Vec<Move<'a>>,
    src: &'a str,
    dst: &'a str,
) {
    lst_moves.push(Move { src: src, dst: dst });
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_nil() {
        test_disks(0, 0);
    }

    #[test]
    fn test_one() {
        test_disks(1, 1);
    }

    #[test]
    fn test_two() {
        test_disks(2, 3);
    }

    #[test]
    fn test_ten() {
        test_disks(10, 1023);
    }

    fn test_disks(num_disks: i64, exp_num_moves: i64) {
        let mut lst_moves: Vec<Move> = Vec::new();
        move_disk_set(num_disks, "A", "B", "C", &mut lst_moves);
        assert_eq!(lst_moves.len() as i64, exp_num_moves);
    }
}
