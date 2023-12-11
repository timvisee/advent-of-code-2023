pub fn main() {
    let map = include_bytes!("../input.txt");
    let width = map.iter().position(|&b| b == b'\n').unwrap();
    let start = map.iter().position(|&b| b == b'S').unwrap();

    let (mut pos, mut dir) = {
        if matches!(map[start - width - 1], b'|' | b'7' | b'F') {
            (start - width - 1, Dir::Up)
        } else if matches!(map[start + width + 1], b'|' | b'L' | b'J') {
            (start + width + 1, Dir::Down)
        } else {
            (start - 1, Dir::Left)
        }
    };

    println!(
        "{}",
        (1 + std::iter::repeat(())
            .position(|_| unsafe {
                match (map.get_unchecked(pos), dir) {
                    (b'|', Dir::Down) => pos += width + 1,
                    (b'|', Dir::Up) => pos -= width + 1,
                    (b'-', Dir::Left) => pos -= 1,
                    (b'-', Dir::Right) => pos += 1,
                    (b'L', Dir::Down) | (b'F', Dir::Up) => {
                        pos += 1;
                        dir = Dir::Right;
                    }
                    (b'L', Dir::Left) | (b'J', Dir::Right) => {
                        pos -= width + 1;
                        dir = Dir::Up;
                    }
                    (b'7', Dir::Up) | (b'J', Dir::Down) => {
                        pos -= 1;
                        dir = Dir::Left;
                    }
                    (b'7', Dir::Right) | (b'F', Dir::Left) => {
                        pos += width + 1;
                        dir = Dir::Down;
                    }
                    (b'S', _) => return true,
                    (_, _) => unreachable!(),
                }
                false
            })
            .unwrap())
            / 2
    );
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
