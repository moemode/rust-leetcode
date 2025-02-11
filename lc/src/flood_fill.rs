/// Your task is to perform a flood fill on the image starting from the pixel
pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let starting_color = image[sr as usize][sc as usize];
    if starting_color != color {
        flood_fill_rec(&mut image, sr as usize, sc as usize, starting_color, color);
    }
    image
}

fn flood_fill_rec(
    image: &mut Vec<Vec<i32>>,
    sr: usize,
    sc: usize,
    starting_color: i32,
    color: i32,
) {
    if image[sr][sc] != starting_color {
        return;
    }
    image[sr][sc] = color;
    if 0 < sr {
        flood_fill_rec(image, sr - 1, sc, starting_color, color);
    }
    if 0 < sc {
        flood_fill_rec(image, sr, sc - 1, starting_color, color);
    }
    if sr + 1 < image.len() {
        flood_fill_rec(image, sr + 1, sc, starting_color, color);
    }
    if sc + 1 < image[0].len() {
        flood_fill_rec(image, sr, sc + 1, starting_color, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(flood_fill(image, 1, 1, 2), expected);
    }

    #[test]
    fn test_flood_fill_no_change() {
        let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let expected = image.clone();
        assert_eq!(flood_fill(image, 0, 0, 0), expected);
    }

    #[test]
    fn test_flood_fill_expected_output() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let result = flood_fill(image, 1, 1, 2);
        let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(result, expected);
    }

    // New test case:
    // Input:
    // image = [[0,0,0],[0,0,0]]
    // sr = 1, sc = 0, color = 2
    // Expected Output: [[2,2,2],[2,2,2]]
    #[test]
    fn test_flood_fill_full_fill() {
        let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let expected = vec![vec![2, 2, 2], vec![2, 2, 2]];
        assert_eq!(flood_fill(image, 1, 0, 2), expected);
    }
}
