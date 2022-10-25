use std::{collections::HashMap, fmt::Debug};

use plotlib::{
    page::Page,
    repr::Plot,
    style::{LineJoin, LineStyle},
    view::ContinuousView,
};
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use rsort::sorts::{bubble::BubbleSort, selection::SelectionSort};

struct Runner<T> {
    data_set: Vec<T>,
}

impl<T> Runner<T>
where
    T: PartialOrd + Clone + Debug,
{
    pub fn new(data_set: Vec<T>) -> Self {
        Runner { data_set }
    }
    pub fn run(&self) -> Vec<(String, f64)> {
        vec![
            (
                "selection sort".to_string(),
                self.data_set.clone().time_selection_sort(),
            ),
            (
                "bubble sort".to_string(),
                self.data_set.clone().time_bubble_sort(),
            ),
        ]
    }
}

fn plot(data: HashMap<String, Vec<f64>>) {
    let plots: Vec<Plot> = data
        .into_par_iter()
        .map(|(k, v)| {
            Plot::new(
                v.into_iter()
                    .enumerate()
                    .map(|(i, v)| ((i + 1) as f64, v))
                    .collect::<Vec<(f64, f64)>>(),
            )
            .line_style(
                LineStyle::new()
                    .colour("burlywood")
                    .linejoin(LineJoin::Round),
            )
            .legend(k)
        })
        .collect();

    let mut view = ContinuousView::new();
    let sample_size = plots[0].data.len();
    for p in plots {
        view = view.add(p)
    }

    Page::single(&(view.x_label("Sample #").y_label("Time (ms)")))
        .save(format!("line_{}.svg", sample_size))
        .unwrap();
}

#[test]
fn nums_upto_1000() {
    let res: Vec<Vec<(String, f64)>> = (1..=1000)
        .collect::<Vec<usize>>()
        .into_par_iter()
        .map(|index| {
            let mut data_set = vec![0; index];
            let mut thread_rng = rand::thread_rng();
            for i in 0..index {
                data_set[i] = thread_rng.gen_range(1..2147483647)
            }
            Runner::<i32>::new(data_set).run()
        })
        .collect();

    let mut map: HashMap<String, Vec<f64>> = HashMap::new();
    for v in res {
        for x in v {
            (*map.entry(x.0).or_default()).push(x.1)
        }
    }

    plot(map)
}

fn main() {}
