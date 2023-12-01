const NUMS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| b == &b'\n')
            .map(|line| {
                let lhs_d = line
                    .iter()
                    .position(|b| b.is_ascii_digit())
                    .unwrap_or(usize::MAX);
                let lhs_n = (0..line.len() - 2)
                    .find_map(|i| {
                        NUMS.iter()
                            .enumerate()
                            .find(|(_, name)| line[i..].starts_with(name))
                            .map(|(num, _)| (i, num + 1))
                    })
                    .unwrap_or((usize::MAX, 0));
                let lhs = if lhs_d <= lhs_n.0 {
                    (line[lhs_d] - b'0') as usize
                } else {
                    lhs_n.1
                };

                let rhs_n = line
                    .iter()
                    .rev()
                    .position(|b| b.is_ascii_digit())
                    .map(|i| line.len() - i - 1)
                    .unwrap_or_default();
                let rhs_d = (0..line.len() - 2)
                    .rev()
                    .find_map(|i| {
                        NUMS.iter()
                            .enumerate()
                            .find(|(_, name)| line[i..].starts_with(name))
                            .map(|(num, _)| (i, num + 1))
                    })
                    .unwrap_or((0, 0));
                let rhs = if rhs_n >= rhs_d.0 {
                    (line[rhs_n] - b'0') as usize
                } else {
                    rhs_d.1
                };

                lhs * 10 + rhs
            })
            .sum::<usize>()
    );
}
