use std::{fmt::Debug, time::Instant};

pub fn run<T: PartialOrd + Clone + Debug>(mut data_set: Vec<T>) -> f64 {
    let start_time = Instant::now();

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

    start_time.elapsed().as_secs_f64() * 1000.0
}
