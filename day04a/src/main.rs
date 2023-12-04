pub fn main() {
    let input = include_bytes!("../input.txt");
    let col = input.iter().position(|&b| b == b':').unwrap();
    let sep = input.iter().position(|&b| b == b'|').unwrap();
    println!(
        "{}",
        input
            .split(|&c| c == b'\n')
            .map(|game| {
                let win = &game[col + 1..sep];
                let wins = game[sep + 1..]
                    .chunks_exact(3)
                    .map(|n| &n[1..])
                    .filter(|n| win.chunks_exact(3).map(|n| &n[1..]).any(|c| &c == n))
                    .count() as u32;
                2usize.pow(wins) >> 1
            })
            .sum::<usize>()
    );
}
