/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    // 拷贝左半部分
    let mut temp_left = Vec::with_capacity(mid);
    temp_left.extend_from_slice(&array[..mid]);

    // 拿到右半部分（原地切片）
    let right_slice = &mut array[mid..];

    // 分治
    sort(&mut temp_left);
    sort(right_slice);

    // 合并写回到原始 array
    let merged = merge(&temp_left, right_slice);
    array.copy_from_slice(&merged);
}

fn merge<T: PartialOrd + Copy>(left_sorted: &[T], right_sorted: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left_sorted.len() + right_sorted.len());

    let mut i = 0;
    let mut j = 0;

    while i < left_sorted.len() && j < right_sorted.len() {
        if left_sorted[i] <= right_sorted[j] {
            result.push(left_sorted[i]);
            i += 1;
        } else {
            result.push(right_sorted[j]);
            j += 1;
        }
    }

    if i < left_sorted.len() {
        result.extend_from_slice(&left_sorted[i..]);
    }
    if j < right_sorted.len() {
        result.extend_from_slice(&right_sorted[j..]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}