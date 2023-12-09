use smallvec::{smallvec, SmallVec};

const MAX: usize = 21;

pub fn main() {
    let nums = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            line.split(|b| b == &b' ')
                .map(|b| atoi::atoi::<i64>(b).unwrap())
                .collect::<SmallVec<[_; MAX]>>()
        })
        .collect::<SmallVec<[_; 200]>>();

    // <https://en.wikipedia.org/wiki/Binomial_coefficient#Pascal's_triangle>
    let mut triang: SmallVec<[SmallVec<[i64; MAX + 1]>; MAX]> = smallvec![smallvec![1]];
    for i in 0..MAX {
        let mut next = smallvec![1];
        next.extend(triang[i].windows(2).map(|w| w[0] + w[1]).chain([1]));
        triang.push(next);
    }
    (0..=MAX)
        .flat_map(|row| (0..=row).step_by(2).map(move |col| (row, col)))
        .for_each(|(row, col)| triang[row][col] *= -1);

    let mut answer = 0;
    for nums in nums {
        let row = nums.len();
        answer += nums
            .iter()
            .enumerate()
            .map(|(col, n)| triang[row][col + 1] * n)
            .sum::<i64>();
    }

    println!("{answer}");
}
