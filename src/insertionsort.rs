use crate::Sorter;

struct InsertionSort;

impl<T> Sorter<T> for InsertionSort {
    fn sort(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 1..slice.len() {
            let mut j = i;
            while j > 0 && slice[j] < slice[j - 1] {
                slice.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn insertionsort_works() {
        use super::*;
        let mut slice = vec![6, 2, 3, 2, 1];
        InsertionSort::sort(&mut slice);
        assert_eq!(slice, [1, 2, 2, 3, 6]);
    }
}
