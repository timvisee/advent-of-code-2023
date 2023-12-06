const SECTIONS: usize = 7;

pub fn main() {
    let input = include_bytes!("../input.txt");

    let mut lines = input.split(|b| b == &b'\n').skip(2);
    let maps: Vec<Vec<(std::ops::Range<u64>, u64)>> = (0..SECTIONS)
        .map(|_| {
            (&mut lines)
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let mut entry = line
                        .splitn(3, |b| b == &b' ')
                        .map(|n| atoi::atoi(n).unwrap());
                    let el: [_; 3] = std::array::from_fn(|_| entry.next().unwrap());
                    (el[1]..el[1] + el[2], el[0])
                })
                .collect()
        })
        .collect();

    println!(
        "{}",
        input[SECTIONS..input.iter().position(|b| b == &b'\n').unwrap()]
            .split(|b| b == &b' ')
            .flat_map(atoi::atoi)
            .map(|seed| {
                maps.iter().fold(seed, |seed, map| {
                    map.iter()
                        .find(|(range, _)| range.contains(&seed))
                        .map(|(range, to)| to + seed - range.start)
                        .unwrap_or(seed)
                })
            })
            .min()
            .unwrap(),
    );
}
