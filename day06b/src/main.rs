pub fn main() {
    let input = include_bytes!("../input.txt");
    let newline = input.iter().position(|b| b == &b'\n').unwrap();
    let time = atoi::atoi::<usize>(
        &input[11..newline]
            .iter()
            .filter(|b| b != &&b' ')
            .copied()
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let dist = atoi::atoi::<usize>(
        &input[newline + 12..]
            .iter()
            .filter(|b| b != &&b' ')
            .copied()
            .collect::<Vec<_>>(),
    )
    .unwrap();

    // Modified quadratic formula: <https://en.wikipedia.org/wiki/Quadratic_formula>
    let a = (time - f32::sqrt((time * time - 4 * dist) as f32) as usize) / 2;
    let b = time - a;
    println!(
        "{}",
        b - (b * (time - b) <= dist) as usize - a - (a * (time - a) <= dist) as usize + 1
    );
}
