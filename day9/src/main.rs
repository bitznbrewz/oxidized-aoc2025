fn all_pairs(v: &[(u64, u64)]) -> Vec<(u64, u64), (u64, u64))> {
    v.iter()
        .enumerate()
        .flat_map(|(i, &h)| v.iter().skip(i+1).map(move |&x| (h,x)))
        .collect()
}

fn calc_area(points: ((u64, u64), (u64, u64))) -> u64 {
    let ((x1, y1), (x2, y2)) = points;
    x2.abs_diff(x1 + 1) * y2.abs_diff(y1+1)
}
fn genxy(p_str: &str) -> (u64, u64) {
    let (x_s, y_s) = p_str.trim().split_once(',').unwrap();
    let x: u64 = x_s.trim().parse().unwrap();
    let y: u64 = y_s.trim().parse().unwrap();
    (x, y)
}
fn main() {
    let contents = file_to_string("input.txt");
    let mut points = Vec::new();
    contents.lines().map(genxy).for_each(|x| points.push(x));
    let pairs = all_pairs(&points);
    let biggest: u64 = pairs.iter().map(|&x| calc_area(x)).max().unwrap_or(0);
    println!("Biggest area is {}", biggest);
}
