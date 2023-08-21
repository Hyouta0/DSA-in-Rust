pub fn mearge_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
        arr.to_vec()
    } else {
        let mid = arr.len() / 2;
        let left = mearge_sort(arr[..mid].to_vec());
        let right = mearge_sort(arr[mid..].to_vec());
        marge(&left.to_vec(), &right.to_vec())
    }
}

fn marge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut merged: Vec<i32> = Vec::new();
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i = i + 1;
        } else {
            merged.push(right[j]);
            j = j + 1;
        }
    }
    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }
    }
    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }
    return merged;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge() {
        assert_eq!(
            marge(&vec![1, 3, 5], &vec![2, 4, 6]),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
    #[test]
    fn test_mg_sort() {
        assert_eq!(
            mearge_sort(vec![9, 6, 7, 2, 3, 5, 8, 1, 4]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
