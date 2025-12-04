use regex::Regex;

fn main() {
    println!("Hello, world!");
    let re = Regex::new(r"([LR])(\d+)").unwrap();

    let content = std::fs::read_to_string("/Users/aaron/Documents/aoc-2025/input1.txt").unwrap_or_default();

    let lines = content.lines().map(|line|
        line.trim());
    let (end, naughty) = lines.fold((50, 0), |(mut curr_pos, mut clicks), line| {
        if let Some(caps) = re.captures(line) {
            let s: i32 = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
            let d: &str = caps.get(1).unwrap().as_str();
            let s_pos = curr_pos;
            match d {
                "L" => {
                    if s_pos == 0 {
                        clicks += (((100 - curr_pos) + s) / 100) - 1;
                    } else {
                        clicks += ((100 - curr_pos) + s) / 100;
                    }
                    curr_pos = (curr_pos - s).rem_euclid(100);
                }
                "R" => {
                    clicks += (curr_pos + s) / 100;
                    curr_pos = (curr_pos + s).rem_euclid(100);
                }
                _ => {
                    println!("Something is wrong with this line!");
                }
            }
            println!("Start: {}, Move: {:?}, End: {}, Clicks: {}", s_pos, (d,s), curr_pos, clicks);
        }
        (curr_pos, clicks)
    });
    println!("Position: {}, Clicks: {}", end, naughty);
}
