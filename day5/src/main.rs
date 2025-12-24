fn main() {
    let contents = file_to_string("input.txt");
    let (upper, lower) = contents.split_once("\r\n").unwrap();
    let ids: Vec<u64> = lower.lines().map(|line| line.trim().parse::<u64>().unwrap()).collect();
    let ranges: Vec<(u64, u64)> = upper.lines().map(|line| {
        let (start, end) = line.split_once('-').unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());
    }).collect::<Vec<(u64, u64)>>();
}
