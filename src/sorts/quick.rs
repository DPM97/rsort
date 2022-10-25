use std::{fmt::Debug, time::Instant};

pub trait QuickSort {
    fn sort(&mut self);
    fn time_sort(&mut self) -> f64;
}

impl<T> QuickSort for Vec<T>
where
    T: PartialOrd + Clone + Debug,
{
    fn sort(&mut self) {
        let len = self.len();
        if len == 1 {
            return;
        }

        fn quicksort<T: PartialOrd + Clone + Debug>(vec: &mut Vec<T>, low: usize, high: usize) {
            if low < high {
                let pivot = high as usize;
                let mut l = low;
                let mut h = high;

                loop {
                    while l <= high && vec[l] < vec[pivot] {
                        l += 1;
                    }
                    while h >= low && vec[h] > vec[pivot] {
                        h -= 1;
                    }

                    if l >= h {
                        break;
                    }

                    vec.swap(l, h);
                    l += 1;
                    h -= 1;
                }

                vec.swap(l, pivot);
                quicksort(vec, low, l - 1);
                quicksort(vec, l + 1, high);
            }
        }

        quicksort(self, 0, len - 1);
    }

    fn time_sort(&mut self) -> f64 {
        let start_time = Instant::now();
        self.sort();
        start_time.elapsed().as_secs_f64() * 1000.0
    }
}
