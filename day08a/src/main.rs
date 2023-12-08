pub fn main() {
    let input = include_bytes!("../input.txt");
    let split = input.iter().position(|&c| c == b'\n').unwrap();

    let mut map = [(0, 0); 174763];
    let enc = |n: &[u8]| {
        ((n[0] - b'0') as u32) << 12 | ((n[1] - b'0') as u32) << 6 | (n[2] - b'0') as u32
    };
    input[split + 2..].split(|&c| c == b'\n').for_each(|node| {
        map[enc(&node[0..3]) as usize] = (enc(&node[7..10]), enc(&node[12..15]));
    });

    println!(
        "{}",
        input[0..split]
            .iter()
            .cycle()
            .scan(enc(b"AAA"), |node, step| {
                let (l, r) = map[*node as usize];
                *node = if step == &b'L' { l } else { r };
                Some(*node & 63 == (b'Z' - b'0') as u32)
            })
            .position(|node| node)
            .unwrap()
            + 1
    );
}
