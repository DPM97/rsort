use std::{fmt::Debug, time::Instant};

pub trait BubbleSort {
    fn sort(&mut self);
    fn time_sort(&mut self) -> f64;
}

impl<T> BubbleSort for Vec<T>
where
    T: PartialOrd + Clone + Debug,
{
    fn sort(&mut self) {
        let len = self.len();
        if len == 1 {
            return;
        }
        let mut swap_ct = 0;
        loop {
            for i in 1..len {
                if self[i - 1] > self[i] {
                    self.swap(i - 1, i);
                    swap_ct += 1;
                }
            }

            if swap_ct == 0 {
                break;
            }

            swap_ct = 0;
        }
    }

    fn time_sort(&mut self) -> f64 {
        let start_time = Instant::now();
        self.sort();
        start_time.elapsed().as_secs_f64() * 1000.0
    }
}
