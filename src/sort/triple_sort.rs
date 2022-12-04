/*
 * Checks if given array can be sorted by left-rotatings of triples
 *
 * Returns 'YES' or 'NO'
 * The function accepts INTEGER_ARRAY A as parameter.
 *
 * Ex:
 * A		rotate
* [1,6,5,2,4,3]	[6,5,2]
* [1,5,2,6,4,3]	[5,2,6]
* [1,2,6,5,4,3]	[5,4,3]
* [1,2,6,3,5,4]	[6,3,5]
* [1,2,3,5,6,4]	[5,6,4]
* [1,2,3,4,5,6]
*
* YES
 */

pub fn sort(original: &[i32]) -> String {
    let mut a = original.to_owned();
    // count of sorted elements
    let mut sorted: usize = 0;
    let mut pos: usize = 0;
    // if only last remained then array is sorted
    while pos < a.len() {
        println!("{:?}, pos {:}", &a, pos);
        // move element to its correct position if it is to the right of it
        if (a[pos] - 1) as usize == sorted {
            match sort_elem(&mut a, pos) {
                Ok(_) => {
                    sorted += 1;
                    pos = sorted;
                    continue;
                }
                Err(_) => return "NO".to_string(),
            }
        }
        pos += 1;
    }
    "YES".to_string()
}

/// rotates-left element on 'from' to a[from - 1] which is its correct position
fn sort_elem(a: &mut [i32], from: usize) -> Result<(), String> {
    let to = (a[from] - 1) as usize;
    if to == from {
        return Ok(());
    };
    if to > from {
        return Err(format!(
            "Can not left-rotate elem {:} from {} to {}",
            a[from], from, to
        ));
    };
    for i in (to..from).rev() {
        let triple_head;
        if i > 0 && (i - 1) >= to {
            triple_head = i - 1
        } else if a.len() - i >= 3 {
            triple_head = i;
        } else {
            return Err(format!(
                "Not enough space to rotate {:} from {} to {}",
                a[from], from, to
            ));
        }
        match rotate(a, triple_head) {
            Ok(_) => continue,
            Err(reason) => return Err(reason.to_owned()),
        }
    }
    Ok(())
}

// rotates left triple [pos, pos+1, pos+2]
fn rotate(a: &mut [i32], pos: usize) -> Result<(), &str> {
    if pos + 2 > a.len() - 1 {
        return Result::Err("Triple exceeds array lenght");
    }
    let first = a[pos];
    let sec = a[pos + 1];
    let third = a[pos + 2];
    a[pos] = sec;
    a[pos + 1] = third;
    a[pos + 2] = first;
    Ok(())
}

#[test]
fn test() {
    assert_eq!(sort(&[1, 6, 5, 2, 4, 3]), "YES");
    assert_eq!(sort(&[3, 1, 2]), "YES");
    assert_eq!(sort(&[1, 3, 4, 2]), "YES");
    assert_eq!(sort(&[1, 2, 3, 5, 4]), "NO");
}
