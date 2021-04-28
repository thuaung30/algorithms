use crate::Sorter;

struct BubbleSort;

impl<T> Sorter<T> for BubbleSort {
    fn sort(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bubblesort_works() {
        use super::*;
        let mut slice = [4, 2, 1, 3, 5];
        BubbleSort::sort(&mut slice);
        assert_eq!(slice, [1, 2, 3, 4, 5])
    }
}
