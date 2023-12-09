use lending_iterator::LendingIterator;
use smallvec::SmallVec;

pub fn main() {
    println!(
        "{}",
        include_bytes!("../../input.txt")
            .split(|b| b == &b'\n')
            .map(|line| {
                let mut nums = line
                    .split(|b| b == &b' ')
                    .map(|b| atoi::atoi::<i32>(b).unwrap())
                    .collect::<SmallVec<[_; 21]>>();
                -nums[0]
                    + (0..nums.len())
                        .rev()
                        .map(|len| {
                            lending_iterator::windows_mut(&mut nums[..=len])
                                .for_each(|w: &mut [_; 2]| w[0] = w[1] - w[0]);
                            (nums[len] == 0, nums[0])
                        })
                        .take_while(|(done, _)| !done)
                        .map(|(_, n)| n)
                        .collect::<SmallVec<[_; 20]>>()
                        .into_iter()
                        .rev()
                        .reduce(|acc, n| n - acc)
                        .unwrap()
            })
            .sum::<i32>(),
    );
}
