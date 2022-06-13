/// consumes given vector and returns a new one with orderred elements
pub fn sort(mut data: Vec<u32>) -> Vec<u32> {
    let len = data.len();
    if len == 1 {
        return data;
    }
    let p = match len % 2 {
        0 => len / 2,
        _ => (len - 1) / 2,
    };
    return merge(&sort(data.drain(0..p).collect()), &sort(data));
}

pub fn merge<'a>(a: &'a [u32], b: &'a [u32]) -> Vec<u32> {
    let mut result = a.to_vec();
    result.extend_from_slice(b);
    return result;
}
