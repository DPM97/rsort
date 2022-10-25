use std::{collections::HashMap, fmt::Debug};

use lazy_static::lazy_static;
use plotlib::{page::Page, repr::Plot, style::PointStyle, view::ContinuousView};
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use rsort::sorts::{
    bubble::BubbleSort, insertion::InsertionSort, quick::QuickSort, selection::SelectionSort,
};

const DS_COUNT: i32 = 5;

struct Runner<T> {
    data_set: Vec<T>,
}

lazy_static! {
    static ref PLOT_COLORS: HashMap<&'static str, &'static str> = HashMap::from([
        ("selection", "red"),
        ("bubble", "blue"),
        ("insertion", "orange"),
        ("quick", "green"),
    ]);
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
                "selection".to_string(),
                SelectionSort::time_sort(&mut self.data_set.clone()),
            ),
            (
                "bubble".to_string(),
                BubbleSort::time_sort(&mut self.data_set.clone()),
            ),
            (
                "insertion".to_string(),
                InsertionSort::time_sort(&mut self.data_set.clone()),
            ),
            (
                "quick".to_string(),
                QuickSort::time_sort(&mut self.data_set.clone()),
            ),
        ]
    }
}

fn plot(data: HashMap<String, Vec<(f64, f64)>>) {
    let plots: Vec<Plot> = data
        .into_par_iter()
        .map(|(k, v)| {
            Plot::new(
                v.into_iter()
                    .map(|(i, v)| (i, v))
                    .collect::<Vec<(f64, f64)>>(),
            )
            .point_style(
                PointStyle::new().colour(PLOT_COLORS.get(&(k.as_ref())).unwrap().to_string()),
            )
            .legend(k)
        })
        .collect();

    let mut view = ContinuousView::new();

    for p in plots {
        view = view.add(p)
    }

    Page::single(&(view.x_label("Samples").y_label("Time (ms)")))
        .save("output.svg")
        .unwrap();
}

fn main() {
    let res: Vec<(i32, Vec<(String, f64)>)> = [1, 2, 3]
        .map(|x| vec![(10 * i32::pow(10, x)) as usize; DS_COUNT as usize])
        .concat()
        .into_par_iter()
        .map(|index| {
            (index as i32, {
                let mut data_set = vec![0; index];
                let mut thread_rng = rand::thread_rng();
                for i in 0..index {
                    data_set[i] = thread_rng.gen_range(1..2147483647)
                }
                let res = Runner::<i32>::new(data_set).run();
                res
            })
        })
        .collect();

    let mut map: HashMap<String, Vec<(f64, f64)>> = HashMap::new();

    for (k, v) in res {
        for (t, x) in v {
            (*map.entry(t).or_default()).push((k as f64, x))
        }
    }

    plot(map)
}
