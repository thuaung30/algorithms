use crate::Sorter;

struct QuickSort;

fn quicksort<T>(slice: &mut [T]) where T: Ord {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice [1] {
                slice.swap(0, 1);
            }
            return
        },
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().unwrap();
    let mut left = 0;
    let mut right = rest.len() - 1;

    while left <= right {
        if &rest[left] <= pivot {
            //left already on the right side
            left +=1;
        } else if &rest[right] > pivot {
            //right already on the right side
            //avoid unnecessary swaps
            if right == 0 {
                //we must be done
                break;
            }
            right -= 1;
        } else {
            //swaps left to right
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                //we must be done
                break;
            }
            right -= 1;
        }
    }

    //align left to make up for pivot 
    let left = left + 1;

    slice.swap(0, left - 1);
    let (left, right) = slice.split_at_mut(left - 1);

    //sanity check
    assert!(left.last() <= right.first());

    quicksort(left);
    quicksort(&mut right[1..]);
}


impl<T> Sorter<T> for QuickSort {
    fn sort(slice: &mut [T])
    where
        T: Ord,
    {
        quicksort(slice);
    }
}



#[cfg(test)]
mod tests {
    use crate::Sorter;

    use super::QuickSort;
    #[test]
    fn quicksort_works() {
        let mut slice = vec![7, 8, 1, 5, 9, 6];
        QuickSort::sort(&mut slice);
        assert_eq!(slice, vec![1, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn quicksort_works2() {
        let mut slice = vec![7, 8, 1, 5, 9, 6, 40 , 19, 58];
        QuickSort::sort(&mut slice);
        assert_eq!(slice, vec![1, 5, 6, 7, 8, 9, 19, 40, 58]);
    }
}
