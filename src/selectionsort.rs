use crate::Sorter;

struct SelectionSort;

impl<T> Sorter<T> for SelectionSort {
    fn sort(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            let mut smallest_index = unsorted;
            for j in (unsorted + 1)..slice.len() {
                if slice[j] < slice[smallest_index] {
                    smallest_index = j;
                }
            }

            //only swap if the smallest element index is not the same
            if unsorted != smallest_index {
                slice.swap(unsorted, smallest_index);
            } 
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn selectionsort_works() {
        use super::*;
        let mut slice = vec![7, 8, 1, 5, 9, 6];
        SelectionSort::sort(&mut slice);
        assert_eq!(slice, vec![1, 5, 6, 7, 8, 9]);
    }
}
