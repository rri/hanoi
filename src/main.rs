//
// src/main.rs
//
use std::io;
use std::io::Write;

/// Solve the Tower of Hanoi problem
fn main() {
    let mut num_disks = 0;
    let mut num_moves = 0;

    get_num_disks(&mut num_disks);

    if num_disks >= 0 {
        println!("Number of disks = {}", num_disks);
        move_disk_set(num_disks, "A", "B", "C", &mut num_moves);
        println!("Number of moves = {}", &num_moves);
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
/// * `num_moves` - Number of moves executed so far
fn move_disk_set(
    num_disks: i64,
    src: &str,
    dst: &str,
    buf: &str,
    num_moves: &mut i64,
) {
    match num_disks {
        0 => return,
        n => {
            move_disk_set(n - 1, src, buf, dst, num_moves);
            move_disk(num_moves, src, dst);
            move_disk_set(n - 1, buf, dst, src, num_moves);
        }
    }
}

/// Move a single disk from source to destination.
///
/// * `num_moves` - Number of moves executed so far
fn move_disk(num_moves: &mut i64, src: &str, dst: &str) {
    *num_moves = *num_moves + 1;
    println!("{:>10}: {} -> {}", num_moves, src, dst);
}
