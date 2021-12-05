pub fn partition<T, P>(data: &mut [T], predicate: P) -> (&mut [T], &mut [T])
where P: Fn(&T) -> bool {
    let idx = partition_index(data, predicate);
    return data.split_at_mut(idx);
}

#[inline]
pub fn partition_index<T, P>(data: &mut [T], predicate: P) -> usize
where P: Fn(&T) -> bool {
    let len = data.len();
    if len == 0 { return 0; }
    let (mut l, mut r) = (0, len - 1);
    loop {
        while l < len && predicate(&data[l]) { l += 1; }
        while r > 0 && !predicate(&data[r]) { r -= 1; }
        if l >= r { return l; }
        data.swap(l, r);
    }
}
