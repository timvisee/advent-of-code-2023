use atoi::atoi;

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .enumerate()
            .filter_map(|(game_id, line)| {
                let mut rgb = [0, 0, 0];
                line.splitn(2, |b| *b == b':')
                    .nth(1)
                    .unwrap()
                    .split(|b| *b == b';')
                    .all(|seq| {
                        seq.split(|b| *b == b',').all(|item| {
                            let i = match item[1..].splitn(2, |b| *b == b' ').nth(1).unwrap() {
                                b"red" => 0,
                                b"green" => 1,
                                b"blue" => 2,
                                _ => unreachable!(),
                            };
                            let channel = &mut rgb[i as usize];
                            let top = (*channel).max(atoi::<u32>(&item[1..]).unwrap());
                            *channel = top;
                            top <= 12 + i
                        })
                    })
                    .then_some(game_id + 1)
            })
            .sum::<usize>()
    );
}
