use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use yew::{
    platform::{spawn_local, time::sleep},
    Callback,
};

pub trait InsertionSort {
    fn sort(&mut self);
    fn time_sort(&mut self) -> f64;
}

pub trait RenderInsertionSort<T> {
    fn sort(self, cb: T);
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

impl<T> RenderInsertionSort<Callback<[(usize, T); 2]>> for Vec<T>
where
    T: PartialOrd + Clone + Debug + Send + Sync + 'static,
{
    fn sort(mut self, cb: Callback<[(usize, T); 2]>) {
        spawn_local(async move {
            let len = self.len();
            if len == 1 {
                return;
            }
            let mut partition = 1;
            while partition < len {
                let mut cur = partition;
                while cur >= 1 && self[cur - 1] > self[cur] {
                    self.swap(cur - 1, cur);
                    cb.emit([(cur - 1, self[cur - 1].clone()), (cur, self[cur].clone())]);
                    sleep(Duration::from_nanos(0)).await;
                    cur -= 1;
                }
                partition += 1;
            }
        });
    }
}
