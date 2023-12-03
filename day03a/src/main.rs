pub fn main() {
    let map = include_bytes!("../input.txt");
    let width = map.iter().position(|b| b == &b'\n').unwrap() as isize;
    println!(
        "{}",
        (0..map.len() - 2)
            .filter(|i| {
                map[*i].is_ascii_digit()
                    && !map.get(i.wrapping_sub(1)).map_or(false, u8::is_ascii_digit)
            })
            .map(|i| {
                let d = (i + 1..i + 4)
                    .position(|i| !map[i].is_ascii_digit())
                    .unwrap()
                    + 1;
                (i, d as _, atoi::atoi::<usize>(&map[i..i + d]).unwrap())
            })
            .filter(|(i, d, _n)| {
                (-width - 2..-width + *d)
                    .chain([-1, *d])
                    .chain(width..width + *d + 2)
                    .any(|j| {
                        map.get((*i as isize + j) as usize)
                            .map_or(false, |b| b != &b'.' && b.is_ascii_punctuation())
                    })
            })
            .map(|(_i, _d, n)| n)
            .sum::<usize>()
    );
}
