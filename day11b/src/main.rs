pub fn main() {
    let map = include_bytes!("../input.txt");
    let size = map.iter().position(|&b| b == b'\n').unwrap();

    let mut galaxies = map
        .iter()
        .enumerate()
        .filter(|(_, &b)| b == b'#')
        .map(|(pos, _)| (pos % (size + 1), pos / (size + 1)))
        .collect::<Vec<_>>();

    (0..size)
        .rev()
        .filter(|y| {
            let offset = y * (size + 1);
            !map[offset..offset + size].iter().any(|&b| b == b'#')
        })
        .for_each(|row| {
            galaxies
                .iter_mut()
                .filter(|(_, y)| *y > row)
                .for_each(|(_, y)| *y += 1);
        });

    (0..size)
        .rev()
        .filter(|x| !map.iter().skip(*x).step_by(size + 1).any(|&b| b == b'#'))
        .for_each(|col| {
            galaxies
                .iter_mut()
                .filter(|(x, _)| *x > col)
                .for_each(|(x, _)| *x += 999_999);
        });

    println!(
        "{}",
        (0..galaxies.len() - 1)
            .flat_map(|a| (a + 1..galaxies.len()).map(move |b| (a, b)))
            .map(|(a, b)| {
                let (x1, y1) = galaxies[a];
                let (x2, y2) = galaxies[b];
                x1.abs_diff(x2) + y1.abs_diff(y2)
            })
            .sum::<usize>()
    );
}
