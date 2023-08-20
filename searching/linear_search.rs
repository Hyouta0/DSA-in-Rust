pub fn ls_list(haystack: &[i32], n: i32) -> bool {
    let len: usize = haystack.len();
    for i in 0..len {
        if haystack[i] == n {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ls() {
        let arr = [1, 2, 3, 4, 5, 6];
        assert!(ls_list(&arr, 2));
    }
}
