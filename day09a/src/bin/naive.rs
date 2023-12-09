use lending_iterator::LendingIterator;
use smallvec::SmallVec;

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| b == &b'\n')
            .map(|line| {
                let mut nums = line
                    .split(|b| b == &b' ')
                    .map(|b| atoi::atoi::<i32>(b).unwrap())
                    .collect::<SmallVec<[_; 21]>>();
                (0..nums.len())
                    .rev()
                    .map(|len| {
                        lending_iterator::windows_mut(&mut nums[..=len])
                            .for_each(|w: &mut [_; 2]| w[0] = w[1] - w[0]);
                        nums[len]
                    })
                    .take_while(|n| *n != 0)
                    .sum::<i32>()
            })
            .sum::<i32>(),
    );
}
