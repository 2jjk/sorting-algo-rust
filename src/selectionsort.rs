use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            #[allow(unused_variables)]
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("slice in not empty");

            // let mut smallest_in_rest = unsorted;
            // for i in (unsorted + 1)..slice.len() {
            //     if slice[i] < slice[smallest_in_rest] {
            //         smallest_in_rest = i;
            //     }
            // }

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 3, 1, 8, 5];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5, 8]);
}
