use std::{fmt::Debug, time::Instant};

pub trait SelectionSort {
    fn selection_sort(&mut self);
    fn time_selection_sort(&mut self) -> f64;
}

impl<T> SelectionSort for Vec<T>
where
    T: PartialOrd + Clone + Debug + Sized,
{
    fn selection_sort(&mut self) {
        let mut start = 0 as usize;
        let len = self.len();
        while start < len {
            let mut min_index = start;
            for i in start + 1..len {
                if self[i] < self[min_index] {
                    min_index = i;
                }
            }
            if start != min_index {
                let tmp = self[start].clone();
                self[start] = self[min_index].clone();
                self[min_index] = tmp;
            }
            start += 1;
        }
    }

    fn time_selection_sort(&mut self) -> f64 {
        let start_time = Instant::now();
        self.selection_sort();
        start_time.elapsed().as_secs_f64() * 1000.0
    }
}

/*
pub fn run<T: PartialOrd + Clone + Debug>(mut data_set: Vec<T>) -> Vec<T> {
    let mut start = 0 as usize;
    let len = data_set.len();
    while start < len {
        let mut min_index = start;
        for i in start + 1..len {
            if data_set[i] < data_set[min_index] {
                min_index = i;
            }
        }
        if start != min_index {
            let tmp = data_set[start].clone();
            data_set[start] = data_set[min_index].clone();
            data_set[min_index] = tmp;
        }
        start += 1;
    }
    data_set
}
*/
