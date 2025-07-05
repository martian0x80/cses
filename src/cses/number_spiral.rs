pub fn generate_spiral(input: i64) -> Vec<Vec<i64>> {
    if input == 1 {
        return vec![vec![1]];
    }
    let mut spiral: Vec<Vec<i64>> = vec![vec![0; input as usize]; input as usize];
    spiral[0][0] = 1;
    let mut val = 2;
    for layer in 1..input {
        if layer % 2 != 0 {
            let mut x = 0;
            let mut y = layer;
            while x < layer as usize {
                spiral[x][y as usize] = val;
                x += 1;
                val += 1;
            }
            while y >= 0 {
                spiral[x][y as usize] = val;
                y -= 1;
                val += 1;
            }
        } else {
            let mut x = layer;
            let mut y = 0;
            while y < layer as usize {
                spiral[x as usize][y] = val;
                y += 1;
                val += 1;
            }
            while x >= 0 {
                spiral[x as usize][y] = val;
                x -= 1;
                val += 1;
            }
        }
    }
    spiral
}

pub fn get_value(x: i64, y: i64) -> i64 {
    if x == 1 && y == 1 {
        return 1;
    }
    let x = x - 1;
    let y = y - 1;
    let mut layer = x;
    if y > x {
        layer = y;
    }
    let mut sol = layer.pow(2);
    let mut offset = 0;
    if layer % 2 != 0 {
        if x != layer {
            offset = x + 1;
        } else if y != layer {
            offset = 2 * (layer + 1) - y - 1;
        } else {
            offset = layer + 1;
        }
    } else {
        if x != layer {
            offset = 2 * (layer + 1) - x - 1;
        } else if y != layer {
            offset = y + 1;
        } else {
            offset = layer + 1;
        }
    }
    sol += offset;
    sol
}

pub struct Point<T> {
    pub x: T,
    pub y: T,
}
