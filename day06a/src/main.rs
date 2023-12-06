pub fn main() {
    let input = include_bytes!("../input.txt");
    let newline = input.iter().position(|b| b == &b'\n').unwrap();

    println!(
        "{}",
        input[11..newline]
            .split(|b| b == &b' ')
            .flat_map(atoi::atoi::<u64>)
            .zip(
                input[newline + 1..]
                    .split(|b| b == &b' ')
                    .flat_map(atoi::atoi::<u64>)
            )
            .map(|(time, dist)| {
                (2..time)
                    .map(|hold| (time - hold) * hold)
                    .filter(|travel| *travel > dist)
                    .count()
            })
            .product::<usize>()
    );
}
