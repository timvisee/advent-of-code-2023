pub fn main() {
    let map = include_bytes!("../input.txt");
    let width = map.iter().position(|b| b == &b'\n').unwrap() as isize;
    let mut nums = vec![];
    println!(
        "{}",
        (0..map.len() - 2)
            .filter(|i| map[*i] == b'*')
            .filter_map(|i| {
                nums.clear();
                nums.extend(
                    (-width - 2..=-width)
                        .chain([-1, 1])
                        .chain(width..=width + 2)
                        .map(|pos| (i as isize + pos) as usize)
                        .filter(|pos| map[*pos].is_ascii_digit())
                        .flat_map(|pos| {
                            (pos.saturating_sub(2)..=pos)
                                .rev()
                                .take_while(|i| map[*i].is_ascii_digit())
                                .last()
                        }),
                );
                nums.dedup();
                (nums.len() == 2).then(|| {
                    nums.iter()
                        .map(|i| atoi::atoi::<usize>(&map[*i..*i + 3]).unwrap())
                        .product::<usize>()
                })
            })
            .sum::<usize>()
    );
}
