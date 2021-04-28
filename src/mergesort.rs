use crate::Sorter;

struct MergeSort;

fn merge_sort<T>(slice: &mut [T]) where T: Ord + Copy {
    //base case
    if slice.len() == 1 {
        return;
    } 

    //get mid point for splitting 
    let mid = slice.len()/2;


    //left split
    merge_sort(&mut slice[0..mid]);
    //right split
    merge_sort(&mut slice[mid..]);

    //copy slice into vec to pass in as placeholder for merging 
    let mut result = slice.to_vec();
    
    //copied slice passed in here
    merge(&slice[0..mid], &slice[mid..], &mut result[..]);


    //copy back the result
    slice.copy_from_slice(&result)
}

fn merge<T>(left: &[T], right: &[T], result: &mut [T]) where T: Ord + Copy {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut result_index = 0;

    let left_max_len = left.len();
    let right_max_len = right.len();

    while left_index < left_max_len && right_index < right_max_len {
        if left[left_index] <= right[right_index] {
            result[result_index] = left[left_index];
            left_index += 1;
        }
        else {
            result[result_index] = right[right_index];
            right_index += 1;
        }
        result_index += 1;
    }

    //copy the rest into result if elements still left in left slice
    if left_index < left_max_len {
        result[result_index..].copy_from_slice(&left[left_index..]);
    }

    //copy the rest into result if elements still left in right slice
    if right_index < right_max_len {
        result[result_index..].copy_from_slice(&right[right_index..]);
    }
}

impl<T> Sorter<T> for MergeSort {
    fn sort(slice: &mut [T])
    where
        T: Ord + Copy
    {
        merge_sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mergesort_works() {
        let mut slice = vec![7, 8, 1, 5, 9, 6];
        MergeSort::sort(&mut slice);
        assert_eq!(slice, vec![1, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn mergesort_works2() {
        let mut slice = vec![30, 10, 8, 12, 1, 2, 17];
        MergeSort::sort(&mut slice);
        assert_eq!(slice, vec![1, 2, 8, 10, 12, 17, 30]);
    }
}
