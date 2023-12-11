const INC: usize = 1;

pub fn main() {
    let map = include_bytes!("../input.txt");
    let size = map.iter().position(|&b| b == b'\n').unwrap();
    let (mut xx, mut yy) = (vec![0; size], vec![0; size]);
    map.iter()
        .enumerate()
        .filter(|(_, &b)| b == b'#')
        .for_each(|(pos, _)| {
            xx[pos % (size + 1)] += 1;
            yy[pos / (size + 1)] += 1;
        });

    println!("{}", dist(&xx) + dist(&yy));
}

// Using insight from <https://redd.it/18fqxuq>
#[inline(always)]
fn dist(counts: &[usize]) -> usize {
    let (mut gaps, mut sum, mut items, mut dist) = (0, 0, 0, 0);
    for (i, count) in counts.iter().enumerate() {
        if *count > 0 {
            let expanded = i + INC * gaps;
            dist += count * (items * expanded - sum);
            sum += count * expanded;
            items += count;
        } else {
            gaps += 1;
        }
    }
    dist
}
