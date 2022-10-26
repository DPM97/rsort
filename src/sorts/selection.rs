use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use yew::{
    platform::{spawn_local, time::sleep},
    Callback,
};

pub trait SelectionSort {
    fn sort(&mut self);
    fn time_sort(&mut self) -> f64;
}

pub trait RenderSelectionSort<T> {
    fn sort(self, cb: T);
}

impl<T> SelectionSort for Vec<T>
where
    T: PartialOrd + Clone + Debug,
{
    fn sort(&mut self) {
        let len = self.len();
        if len == 1 {
            return;
        }
        let mut start = 0 as usize;
        while start < len {
            let mut min_index = start;
            for i in start + 1..len {
                if self[i] < self[min_index] {
                    min_index = i;
                }
            }
            if start != min_index {
                self.swap(start, min_index);
            }
            start += 1;
        }
    }

    fn time_sort(&mut self) -> f64 {
        let start_time = Instant::now();
        // self.sort();
        start_time.elapsed().as_secs_f64() * 1000.0
    }
}

impl<T> RenderSelectionSort<Callback<[(usize, T); 2]>> for Vec<T>
where
    T: PartialOrd + Clone + Debug + Send + Sync + 'static,
{
    fn sort(mut self, cb: Callback<[(usize, T); 2]>) {
        spawn_local(async move {
            let len = self.len();
            if len == 1 {
                return;
            }
            let mut start = 0 as usize;
            while start < len {
                let mut min_index = start;
                for i in start + 1..len {
                    if self[i] < self[min_index] {
                        min_index = i;
                    }
                }
                if start != min_index {
                    self.swap(start, min_index);
                    cb.emit([
                        (start, self[start].clone()),
                        (min_index, self[min_index].clone()),
                    ]);
                    sleep(Duration::from_nanos(1)).await;
                }
                start += 1;
            }
        });
    }
}
