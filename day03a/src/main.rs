pub fn main() {
    let map = include_bytes!("../input.txt");
    let width = map.iter().position(|b| b == &b'\n').unwrap() as isize;
    println!(
        "{}",
        (0..map.len() - 2)
            .filter(|i| {
                map.get(i.wrapping_sub(1))
                    .map_or(true, |b| !b.is_ascii_digit())
                    && map[*i].is_ascii_digit()
            })
            .map(|i| {
                let digits = (i..i + 4)
                    .position(|blah| !map[blah].is_ascii_digit())
                    .unwrap();
                let num: usize = atoi::atoi(&map[i..i + digits]).unwrap();
                (i, num, digits as isize)
            })
            .filter(|(i, _num, digits)| {
                (-width - 2..-width + *digits)
                    .chain([-1, *digits])
                    .chain(width..width + *digits + 2)
                    .any(|j| {
                        let pos = (*i as isize + j) as usize;
                        map.get(pos)
                            .map_or(false, |b| b != &b'.' && b.is_ascii_punctuation())
                    })
            })
            .map(|(_i, num, _digits)| num)
            .sum::<usize>()
    );
}
