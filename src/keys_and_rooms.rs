use std::sync::Mutex;

pub struct Solution {}

impl Solution {
    pub fn keys_and_rooms(&self, rooms: &Vec<Vec<i32>>) -> bool {
        let visited = Mutex::new(vec![false; rooms.len()]);
        struct Visitor<'a> {
            visit: &'a dyn Fn(&Visitor, i32),
        }
        let visitor = Visitor {
            visit: &|visitor, key| {
                visited.lock().unwrap()[key as usize] = true;
                let keys = &rooms[key as usize];
                for key in keys {
                    if !visited.lock().unwrap()[key.to_owned() as usize] {
                        (visitor.visit)(&visitor, key.to_owned())
                    }
                }
            },
        };
        (visitor.visit)(&visitor, 0);
        return visited
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .fold(true, |acc, elem| acc & elem);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let case1 = vec![vec![1], vec![2], vec![3], vec![]];
        let sol = Solution {};
        let case2 = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        assert!(sol.keys_and_rooms(&case1) == true);
        assert!(sol.keys_and_rooms(&case2) == false);
    }
}
