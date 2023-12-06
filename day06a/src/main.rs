pub fn main() {
    let input = include_bytes!("../input.txt");
    let newline = input.iter().position(|b| b == &b'\n').unwrap();

    println!(
        "{}",
        input[11..newline]
            .split(|b| b == &b' ')
            .flat_map(atoi::atoi::<usize>)
            .zip(
                input[newline + 12..]
                    .split(|b| b == &b' ')
                    .flat_map(atoi::atoi::<usize>),
            )
            .map(|(t, d)| {
                // Modified quadratic formula: <https://en.wikipedia.org/wiki/Quadratic_formula>
                let a = (t - f32::sqrt((t * t - 4 * d) as f32) as usize) / 2;
                let b = t - a;
                b - (b * (t - b) <= d) as usize - a - (a * (t - a) <= d) as usize + 1
            })
            .product::<usize>()
    );
}
