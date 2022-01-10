use std::{collections::HashSet, sync::Mutex};

pub fn frog_jumps(stones: Vec<i32>) -> bool {
    if stones[1] != 1 {
        return false;
    }
    let excluded = Mutex::new(HashSet::new());
    let curr = stones.len() - 1;
    if curr == 1 {
        return true;
    }
    let previous: Vec<usize> = (1..curr).map(|i| i).collect();

    return previous.iter().any(|i| jump(*i, curr, &stones, &excluded));
}

pub fn jump(
    from: usize,
    to: usize,
    stones: &Vec<i32>,
    excluded: &Mutex<HashSet<(usize, usize)>>,
) -> bool {
    if from == 0 && to == 1 {
        return true;
    }
    println!("jump from {:?} to {:?}", from, to);
    println!("excluded: {:?}", excluded.lock().unwrap());

    if let Some((_, _)) = excluded.lock().unwrap().get(&(from, to)) {
        return false;
    }

    let mut previous: Vec<usize> = Vec::new();
    for i in 0..from {
        if ((stones[from] - stones[i]) - (stones[to] - stones[from])).abs() <= 1 {
            previous.push(i);
        }
    }
    println!("from: {}, to: {}, previous: {:?}", &from, &to, &previous);

    let is_reachable = previous
        .iter()
        .any(|prev| jump(*prev, from, stones, excluded));
    if !is_reachable {
        excluded.lock().unwrap().insert((from, to));
    }

    return is_reachable;
}

#[test]
fn test() {
    assert_eq!(frog_jumps(vec![0, 1, 3, 5, 6, 8, 12, 17]), true);

    assert_eq!(frog_jumps(vec![0, 1, 2, 3, 4, 8, 9, 11]), false);

    assert_eq!(frog_jumps(vec![0, 1]), true);

    assert_eq!(
        frog_jumps(vec![
            0, 2, 4, 5, 6, 8, 9, 11, 14, 17, 18, 19, 20, 22, 23, 24, 25, 27, 30
        ]),
        false
    );
}
