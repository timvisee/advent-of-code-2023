pub fn main() {
    let input = include_bytes!("../input.txt");
    let col = input.iter().position(|&b| b == b':').unwrap();
    let sep = input.iter().position(|&b| b == b'|').unwrap();
    println!(
        "{}",
        input
            .split(|&b| b == b'\n')
            .map(|game| {
                let win_seq = &game[col + 1..sep];
                let win_count = game[sep + 1..]
                    .chunks_exact(3)
                    .map(|n| &n[1..])
                    .filter(|n| win_seq.chunks_exact(3).map(|n| &n[1..]).any(|c| &c == n))
                    .count() as u32;
                2usize.pow(win_count) >> 1
            })
            .sum::<usize>()
    );
}
