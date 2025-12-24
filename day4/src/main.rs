use utils::file_to_string;
use std::cmd::{max, min};

fn clamp_range(x: usize, y: usize, row_len: usize, col_len: usize) -> (usize, usize, usize, usize) {
    let r_min = max(0, x as i32-1) as usize;
    let r_max = min(col_len-1, x+1);
    let c_min = max(0, y as i32-1) as usize;
    let c_max = min(row_len-1, y+1);
    (r_min, r_max, c_min, c_max)
}

fn is_movable(r: &(usize, usize), mat: &Vec<Vec<bool>>, row_len: usize, col_len: usize) -> bool {
    let (r_min, r_max, c_min, c_max) = clamp_range(r.0, r.1, row_len, col_len);
    let mut sum = 0;
    for rdx in r_min..=r_max {
        for cdx in c_min..=c_max {
            if (rdx, cdx) != *r {
                if mat[rdx][cdx] {
                    sum += 1;
                }
            }
        }
    }
    if sum < 4 {
        return true;
    } else {
        return false;
    }
}

fn part1(mat: &Vec<Vec<bool>>) -> u32 {
    let row_len = mat[0].len();
    let col_len = mat.len();
    let mut roll_loc: Vec<(usize, usize)> = mat.iter()
        .enumerate()
        .flat_map(|(rdx, row)| {
            row.iter().enumerate()
                .filter_map(move |(cdx, &v)| {
                    v.then_some((rdx, cdx))
                })
        })
    .collect::<Vec<_>>();
    for roll in roll_loc.iter_mut() {
        is_movable(roll, mat, row_len, col_len);
    }
    let movable_rolls = roll_loc.iter().fold(0, |acc, r| if is_movable(r, mat, row_len, col_len) { acc + 1 } else { acc });
    movable_rolls
}

fn part2(mat: &mut Vec<Vec<bool>>) -> u32 {
    let row_len = mat[0].len();
    let col_len = mat.len();
    let mut moved_rolls: u32 = 0;
    let mut movable_rolls: Vec<(usize, usize)> = Vec::new();
    let mut immovable_rolls: Vec<(usize, usize)> = Vec::new();
    for (rdx, row) in mat.iter().enumerate() {
        for (cdx, &v) in row.iter().enumerate() {
            if v {
                match is_movable(&(rdx, cdx), mat, row_len, col_len) {
                    true => movable_rolls.push((rdx, cdx)),
                    false => immovable_rolls.push((rdx, cdx)),
                }
            }
        }
    }
    loop {
        while !movable_rolls.is_empty() {
            match movable_rolls.pop() {
                Some(r) => {
                    mat[r.0][r.1] = false;
                    moved_rolls += 1
                },
                None => {
                    break;
                }
            }
        }
        movable_rolls.clear();
        let mut movable_idx: Vec<usize> = Vec::new();
        for (idx, r) in immovable_rolls.iter().enumerate() {
            if is_movable(r, mat, row_len, col_len) {
                movable_rolls.push(*r);
                movable_idx.push(idx);
            }
        }
        while !movable_idx.is_empty() {
            match movable_idx.pop() {
                Some(idx) => immovable_rolls.remove(idx),
                None => break,
            };
        }
        if movable_rolls.is_empty() {
            break;
        }
    }
    moved_rolls
}
fn main() {
    let contents = file_to_string("input.txt");
    let mut mat: Vec<Vec<bool>> = contents.lines().map(|line| line.chars().map(|c| c == '@').collect()).collect();
    let result = part1(&mat);
    let result2 = part2(&mut mat);
    println!("Part 1: There are {} movable rolls", result);
    println!("Part 2: There are {} moved rolls", result2);
}
