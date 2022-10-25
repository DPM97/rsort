use std::{fmt::Debug, time::Instant};

pub trait BubbleSort {
    fn bubble_sort(&mut self);
    fn time_bubble_sort(&mut self) -> f64;
}

impl<T> BubbleSort for Vec<T>
where
    T: PartialOrd + Clone + Debug + Sized,
{
    fn bubble_sort(&mut self) {
        let mut swap_ct = 0;
        let len = self.len();
        loop {
            for i in 1..len {
                if self[i - 1] > self[i] {
                    let tmp = self[i].clone();
                    self[i] = self[i - 1].clone();
                    self[i - 1] = tmp;
                    swap_ct += 1;
                }
            }

            if swap_ct == 0 {
                break;
            } else {
                swap_ct = 0;
            }
        }
    }

    fn time_bubble_sort(&mut self) -> f64 {
        let start_time = Instant::now();
        self.bubble_sort();
        start_time.elapsed().as_secs_f64() * 1000.0
    }
}
