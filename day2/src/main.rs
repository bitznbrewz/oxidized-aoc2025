fn halves_equal(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len.is_multiple_of(2) {
        let (first, second) = s.split_at(len/2);
        first == second
    } else {
        false
    }
}

fn get_divisors(n: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut i = 1usize;
    while i * i <= n {
        if n % i == 0 {
            result.push(i);
            if i != n / i {
                result.push(n / i);
            }
        }
        i += 1;
    }
    result
}

fn parts_equal(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    let divs: Vec<usize> = get_divisors(len);
    let bytes = s.as_bytes();
    let result = divs
        .iter()
        .filter(|&&d| d > 1usize && len % d == 0)
        .any(|&d| {
            let part_len = s.len() / d;
            let first = &bytes[..part_len];
            bytes.chunks(part_len).take(d).all(|chunk| chunk == first)
        });
    if result {
        println!("Invalid id: {}", n);
    }
    result
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let ranges: Vec<&str> = content.split(",").collect();
    let mut sum: u64 = 0;
    for range in ranges {
        println!("Running for range: {}", range);
        let rs = range.trim();
        let (start_s, end_s) = rs.split_once("-").unwrap();
        let start: u64 = start_s.trim().parse().unwrap();
        let end: u64 = end_s.trim().parse().unwrap();
        let temp_sum: u64 = (start..=end).filter(|&num| parts_equal(num)).sum();
        sum += temp_sum;
    }
    println!("Final number is: {}", sum);
}
