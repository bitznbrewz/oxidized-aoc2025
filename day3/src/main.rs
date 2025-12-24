use utils::file_to_string();

fn part1(contents: &String) -> u64 {
    let mut sum: u64 = 0;
    for line in contents.lines() {
        let nums: Vec<u64> = line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
        let mut tens: (usize, u64) = (0, 0);
        let mut ones: (usize, u64) = (0, 0);
        for (i, &n) in nums[..nums.len()-1].iter().enumerate() {
            if n > tens.1 {
                if n == 9u64 {
                    tens.0 = i;
                    tens.1 = n;
                    break;
                } else {
                    tens.0 = i;
                    tens.1 = n;
                }
            }
        }
        for (i, &n) in nums[tens.0+1..nums.len()].iter().enumerate() {
            if n > ones.1 {
                if n == 9u64 {
                    ones.0 = i;
                    ones.1 = n;
                    break;
                } else {
                    ones.0 = i;
                    ones.1 = n;
                }
            }
        }
    }
    sum
}

fn recur(size: usize, nums: &Vec<u64>, fin: &mut [u64;12], idx: usize, start: usize) {
    let og_offset = start;
    let mut new_offset = 0;
    for (i, &n) in nums[og_offset..nums.len()-size+idx+1].iter().enumerate() {
        if n > fin[idx] {
            if n == 9u64 {
                fin[idx] = n;
                new_offset = og_offset + i + 1;
            }
        } else {
            fin[idx] = n;
            new_offset = og_offset + i + 1;
        }
    }
    if idx + 1 < size {
        recur(size, nums, inf, idx+1, new_offset);
    }
}

fn part2(contents: &String) -> u64 {
    let mut sum: u64 = 0;
    const LEN: usize = 12;
    let idx = 0;
    for line in contents.lines() {
        let nums: Vec<u64> = line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
        let mut fin: [u64; LEN] = [0; LEN];
        let start = 0;
        recur(LEN, &nums, &mut fin, idx, start);
        let temp_sum = fin.iter().fold(0, |acc, &x| acc * 10 + x);
        sum += temp_sum;
    }
    sum
}

fn main() {
    let contents = file_to_string("input.txt");
    let sum = part1(&contents);
    let sum2 = part2(&contents);
    println!("Part1: Sum of largest numbers: {}", sum);
    println!("Part2: Sum of largest numbers: {}", sum2);
}
