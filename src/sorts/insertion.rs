use std::{fmt::Debug, time::Instant};

pub trait InsertionSort {
    fn sort(&mut self);
    fn time_sort(&mut self) -> f64;
}

impl<T> InsertionSort for Vec<T>
where
    T: PartialOrd + Clone + Debug,
{
    fn sort(&mut self) {
        let len = self.len();
        if len == 1 {
            return;
        }
        let mut partition = 1;
        while partition < len {
            let mut cur = partition;
            while cur >= 1 && self[cur - 1] > self[cur] {
                self.swap(cur - 1, cur);
                cur -= 1;
            }
            partition += 1;
        }
    }

    fn time_sort(&mut self) -> f64 {
        let start_time = Instant::now();
        self.sort();
        start_time.elapsed().as_secs_f64() * 1000.0
    }
}
