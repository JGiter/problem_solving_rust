use std::vec;

/// Given an n x n grid containing only values 0 and 1, where 0 represents water and 1 represents land,
/// find a water cell such that its distance to the nearest land cell is maximized, and return the distance.
/// If no land or water exists in the grid, return -1.
/// The distance used in this problem is the Manhattan distance: the distance between two cells (x0, y0) and (x1, y1) is |x0 - x1| + |y0 - y1|.

/// On each iteration remove coastal cells. Return number of steps token to remove all water
struct Solution {}
#[allow(unused)]
impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut distances = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                distances[i][j] = match grid[i][j] {
                    1 => 0,
                    _ => i32::MAX,
                }
            }
        }

        // coordinates of the water cells, for which distance from land is not yet known
        let mut water: Vec<(usize, usize)> = vec![];
        for i in 0..n {
            for j in 0..n {
                match grid.get(i).and_then(|row| row.get(j)) {
                    Some(v) => {
                        if *v == 0 {
                            water.push((i, j))
                        }
                    }
                    None => (),
                }
            }
        }

        if water.len() == n * n || water.len() == 0 {
            return -1;
        };

        let mut max_distance = 0;
        while water.len() > 0 {
            max_distance += 1;
            for cell in &mut water {
                if Solution::distance_from_land(cell, &distances) == max_distance {
                    distances[cell.0][cell.1] = max_distance;
                    // mark water cell which will be turned into land
                    cell.0 = usize::MAX;
                }
            }
            water.retain(|cell| cell.0 != usize::MAX);
        }

        max_distance
    }

    fn distance_from_land(cell: &(usize, usize), distances: &Vec<Vec<i32>>) -> i32 {
        let n = distances.len();
        let top = distances
            .get(cell.0.saturating_sub(1))
            .and_then(|row| row.get(cell.1))
            .unwrap_or(&i32::MAX);
        let right = distances
            .get(cell.0)
            .and_then(|row| row.get((cell.1 + 1).clamp(0, n)))
            .unwrap_or(&i32::MAX);
        let bottom = distances
            .get((cell.0 + 1).clamp(0, n))
            .and_then(|row| row.get(cell.1))
            .unwrap_or(&i32::MAX);
        let left = distances
            .get(cell.0)
            .and_then(|row| row.get(cell.1.saturating_sub(1)))
            .unwrap_or(&i32::MAX);
        *[*top, *right, *bottom, *left].iter().min().unwrap() + 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let mut grid = vec![vec![1, 0], vec![0, 0]];
        assert_eq!(Solution::max_distance(grid), 2);
        grid = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(Solution::max_distance(grid), 2);
    }
}
