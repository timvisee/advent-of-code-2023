pub fn main() {
    let input = include_bytes!("../input.txt");
    let split = input.iter().position(|&c| c == b'\n').unwrap();

    let (mut map, mut starts) = ([0u32; 0b11001_11001_11001 + 1], Vec::with_capacity(6));
    let enc = |n: &[u8]| {
        ((n[0] - b'A') as u32) << 10 | ((n[1] - b'A') as u32) << 5 | (n[2] - b'A') as u32
    };
    input[split + 2..].split(|&c| c == b'\n').for_each(|node| {
        map[enc(&node[0..3]) as usize] = enc(&node[7..10]) | enc(&node[12..15]) << 16;
        if node[2] == b'A' {
            starts.push(enc(&node[0..3]));
        }
    });

    println!(
        "{}",
        starts
            .into_iter()
            .map(|node| {
                input[0..split]
                    .iter()
                    .cycle()
                    .scan(node, |node, step| {
                        *node = if step == &b'L' {
                            map[*node as usize] & u16::MAX as u32
                        } else {
                            map[*node as usize] >> 16
                        };
                        Some(*node & 0b11111 == (b'Z' - b'A') as u32)
                    })
                    .position(|node| node)
                    .unwrap()
                    + 1
            })
            .fold(1, num_integer::lcm)
    );
}
