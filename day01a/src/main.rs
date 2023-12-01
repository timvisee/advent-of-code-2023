pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| b == &b'\n')
            .map(|line| {
                ((line.iter().find(|b| b.is_ascii_digit()).unwrap() - b'0') * 10
                    + line.iter().rev().find(|b| b.is_ascii_digit()).unwrap()
                    - b'0') as usize
            })
            .sum::<usize>()
    );
}
