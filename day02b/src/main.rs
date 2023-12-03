use atoi::atoi;

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| b == &b'\n')
            .map(|line| {
                let mut rgb = [0, 0, 0];
                line.splitn(2, |b| b == &b':')
                    .nth(1)
                    .unwrap()
                    .split(|b| b == &b',' || b == &b';')
                    .for_each(|item| {
                        let i = match item[1..].splitn(2, |b| *b == b' ').nth(1).unwrap() {
                            b"red" => 0usize,
                            b"green" => 1,
                            b"blue" => 2,
                            _ => unreachable!(),
                        };
                        rgb[i] = rgb[i].max(atoi(&item[1..]).unwrap());
                    });
                rgb.iter().product::<u32>()
            })
            .sum::<u32>()
    );
}
