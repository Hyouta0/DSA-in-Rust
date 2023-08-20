fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble() {
        let mut arr = [6, 3, 2, 5, 4, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
