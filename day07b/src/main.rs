pub fn main() {
    let mut hands = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|hand| {
            let (mut ranks, mut power, mut jokers) = ([0u8; 13], 0, 0);
            for i in 0..5 {
                let card = match hand[i] {
                    b'A' => 12,
                    b'K' => 11,
                    b'Q' => 10,
                    b'J' => 0,
                    b'T' => 9,
                    n => n - b'0' - 1,
                };
                ranks[card as usize] += 1 * (card != 0) as u8;
                power |= (card as u32) << 4 * (4 - i);
                jokers += 1 * (card == 0) as u8;
            }
            ranks.sort_unstable_by(|a, b| b.cmp(a));
            power |= match ranks[0] + jokers {
                5 => 6,
                4 => 5,
                3 if ranks[1] == 2 => 4,
                3 => 3,
                2 if ranks[1] == 2 => 2,
                2 => 1,
                _ => 0,
            } << 29;
            (power, atoi::atoi::<u32>(&hand[6..]).unwrap())
        })
        .collect::<Vec<_>>();
    hands.sort_unstable();

    println!(
        "{}",
        hands
            .into_iter()
            .enumerate()
            .map(|(i, (_power, bet))| bet * (i as u32 + 1))
            .sum::<u32>()
    );
}
