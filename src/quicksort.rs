use super::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }
    let (pivot, slice) = slice.split_first_mut().expect("slice is not empty");
    let pivot = &pivot[0];
    let mut left = 0;
    let mut right = 0;
    for i in 1..slice.len() {
        if slice[i] <= pivot {
            left += 1;
        } else {
        }
    }

    quicksort(left);
    quicksort(right);
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quicksort(slice)
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 3, 1, 8, 5];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5, 8]);
}
