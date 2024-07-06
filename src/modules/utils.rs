pub fn combine_vectors<T>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    let mut new_vec = Vec::with_capacity(v1.len() + v2.len());
    for item in v1 {
        new_vec.push(item);
    }
    for item in v2 {
        new_vec.push(item);
    }
    return new_vec;
}
