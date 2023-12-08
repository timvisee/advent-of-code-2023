const RUNS: usize = 1_000_000_000;
const TOTAL: usize = 19_185_263_738_117;

pub fn main() {
    let input = include_bytes!("../../input.txt");
    let split = input.iter().position(|&c| c == b'\n').unwrap();

    let (mut map, mut nodes) = ([0u64; 174763], Vec::with_capacity(6));
    let enc = |n: &[u8]| {
        ((n[0] - b'0') as u64) << 12 | ((n[1] - b'0') as u64) << 6 | (n[2] - b'0') as u64
    };
    input[split + 2..].split(|&c| c == b'\n').for_each(|node| {
        map[enc(&node[0..3]) as usize] = enc(&node[7..10]) | enc(&node[12..15]) << 32;
        if node[2] == b'A' {
            nodes.push(enc(&node[0..3]));
        }
    });

    let took = took::Timer::new();

    let answer = input[0..split]
        .iter()
        .cycle()
        .map(|step| {
            for node in &mut nodes {
                *node = if step == &b'L' {
                    map[*node as usize] & u32::MAX as u64
                } else {
                    map[*node as usize] >> 32
                };
            }
            nodes
                .iter()
                .all(|node| *node & 0b111111 == (b'Z' - b'0') as u64)
        })
        .take(RUNS)
        .position(|node| node);

    let took = took.took();
    took.describe(&format!("Running {RUNS} iterations"));

    let factor = TOTAL / RUNS;
    let extrapolated = took::Took::from_std(took.into_std() * factor as u32);
    extrapolated.describe(&format!("Extrapolated to {TOTAL} iterations"));

    println!("Answer: {:?}", answer,);
}
