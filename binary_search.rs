pub fn bs_list(haystack: &[i32], needal: i32) -> bool {
    let mut start: usize = 0;
    let mut end: usize = haystack.len();
    while start < end {
        let mid: usize = start + (end - start) / 2;
        if haystack[mid] == needal {
            return true;
        } else if haystack[mid] < needal {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bs() {
        let arr = [1, 2, 3, 4, 5, 6];
        assert!(bs_list(&arr, 3));
    }
}
