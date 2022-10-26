use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use yew::{
    platform::{spawn_local, time::sleep},
    Callback,
};

pub trait BubbleSort {
    fn sort(&mut self);
    fn time_sort(&mut self) -> f64;
}

pub trait RenderBubbleSort<T> {
    fn sort(self, cb: T);
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

impl<T> RenderBubbleSort<Callback<[(usize, T); 2]>> for Vec<T>
where
    T: PartialOrd + Clone + Debug + Send + Sync + 'static,
{
    fn sort(mut self, cb: Callback<[(usize, T); 2]>) {
        spawn_local(async move {
            let len = self.len();
            if len == 1 {
                return;
            }
            let mut swap_ct = 0;
            loop {
                for i in 1..len {
                    if self[i - 1] > self[i] {
                        self.swap(i - 1, i);
                        cb.emit([(i - 1, self[i - 1].clone()), (i, self[i].clone())]);
                        sleep(Duration::from_nanos(1)).await;
                        swap_ct += 1;
                    }
                }

                if swap_ct == 0 {
                    break;
                }

                swap_ct = 0;
            }
        });
    }
}
