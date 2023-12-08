use hashbrown::HashMap;

pub fn main() {
    let input = include_bytes!("../input.txt");
    let split = input.iter().position(|&c| c == b'\n').unwrap();

    let map = input[split + 2..]
        .split(|&c| c == b'\n')
        .map(|node| {
            (
                node[0..3].try_into().unwrap(),
                (
                    node[7..10].try_into().unwrap(),
                    node[12..15].try_into().unwrap(),
                ),
            )
        })
        .collect::<HashMap<[_; 3], ([_; 3], [_; 3])>>();

    println!(
        "{}",
        map.keys()
            .filter(|node| node[2] == b'Z')
            .cloned()
            .map(|node| {
                input[0..split]
                    .iter()
                    .cycle()
                    .scan(node, |node, step| {
                        let (l, r) = map[&node[0..3]];
                        *node = if step == &b'L' { l } else { r };
                        Some(node[2] == b'Z')
                    })
                    .position(|node| node)
                    .unwrap()
                    + 1
            })
            .fold(1, |acc, x| num_integer::lcm(acc, x))
    );
}
