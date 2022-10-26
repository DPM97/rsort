use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use async_recursion::async_recursion;
use yew::{
    platform::{spawn_local, time::sleep},
    Callback,
};

pub trait QuickSort {
    fn sort(&mut self);
    fn time_sort(&mut self) -> f64;
}

pub trait RenderQuickSort<T> {
    fn sort(self, cb: T);
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

impl<T> RenderQuickSort<Callback<Vec<T>>> for Vec<T>
where
    T: PartialOrd + Clone + Debug + Send + Sync + 'static,
{
    fn sort(mut self, cb: Callback<Vec<T>>) {
        spawn_local(async move {
            let len = self.len();
            if len == 1 {
                return;
            }

            #[async_recursion(?Send)]
            async fn quicksort<T: PartialOrd + Clone + Debug>(
                cb: Callback<Vec<T>>,
                vec: &mut Vec<T>,
                low: usize,
                high: usize,
            ) {
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
                    cb.emit(vec.clone());
                    sleep(Duration::from_micros(1)).await;
                    quicksort(cb.clone(), vec, low, l - 1).await;
                    quicksort(cb, vec, l + 1, high).await;
                }
            }

            quicksort(cb, &mut self, 0, len - 1).await;
        });
    }
}
