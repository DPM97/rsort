use std::{collections::HashMap, fmt::Debug};

use lazy_static::lazy_static;
use plotlib::{
    page::Page,
    repr::Plot,
    style::{LineJoin, LineStyle, PointStyle},
    view::ContinuousView,
};
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use rsort::{
    components::bar::Bar,
    sorts::{
        bubble::BubbleSort,
        insertion::InsertionSort,
        quick::QuickSort,
        selection::{RenderSelectionSort, SelectionSort},
    },
    Msg,
};

use web_sys::console;
use yew::{
    function_component, html, platform::spawn_local, use_force_update, use_mut_ref, use_state,
    Component, Context, Html, Properties, UseStateHandle,
};

#[derive(Properties, PartialEq, Debug)]
pub struct Graph {
    name: String,
    vals: Vec<i32>,
}

#[derive(Properties, PartialEq, Debug)]
pub struct AppProps {
    data: Vec<i32>,
}

impl Default for AppProps {
    fn default() -> AppProps {
        AppProps {
            // single graph (selection test)
            data: gen_random_data(250),
        }
    }
}

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
    pub fn _new(data_set: Vec<T>) -> Self {
        Runner { data_set }
    }
    pub fn _run(&self) -> Vec<(String, f64)> {
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

fn _plot(data: HashMap<String, Vec<(f64, f64)>>) {
    let plots: Vec<Plot> = data
        .into_par_iter()
        .map(|(k, v)| {
            Plot::new(
                v.into_iter()
                    .map(|(i, v)| (i, v))
                    .collect::<Vec<(f64, f64)>>(),
            )
            .line_style(
                LineStyle::new()
                    .colour(PLOT_COLORS.get(&(k.as_ref())).unwrap().to_string())
                    .linejoin(LineJoin::Round),
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

#[test]
fn parallelize() {
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
                let res = Runner::<i32>::_new(data_set)._run();
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

    _plot(map)
}

fn gen_random_data(size: usize) -> Vec<i32> {
    let mut data_set = vec![0; size];
    let mut thread_rng = rand::thread_rng();
    for i in 0..size {
        data_set[i] = thread_rng.gen_range(1..2147483647)
    }
    data_set
}

#[function_component(MyApp)]
pub fn app(props: &AppProps) -> Html {
    let render_count: UseStateHandle<u32> = use_state(|| 0);

    let count = *render_count;

    let total_width_px = 1500;

    // single graph (selection test)
    let data = gen_random_data(250);
    let (min, max) = (
        data.iter().min().unwrap().clone(),
        data.iter().max().unwrap().clone(),
    );

    let state = use_force_update();
    let graphs = use_mut_ref(|| data);
    let bar_width_px = 100 as f32 / graphs.borrow().len() as f32;

    let items = (*graphs).clone().into_inner().into_iter().map(|i| {
        let height_from_top = f32::max(
            0.0,
            100.0 - (((i - min) as f32 / (max - min) as f32) * 100.0) as f32,
        );

        html! {
        <Bar
            width={bar_width_px}
            height={height_from_top}
            value={i as i32}
            />
        }
    });

    println!("{}", items.len());

    html! {
        <div style="margin: -8px;">
            {for items}
        </div>
    }
}

pub struct AsyncComponent<T> {
    data: Vec<T>,
}

impl Component for AsyncComponent<i32> {
    type Message = Msg<i32>;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let data = gen_random_data(500);
        let default = data.clone();
        RenderSelectionSort::sort(data, ctx.link().callback(Msg::Data));
        Self { data: default }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        console::log_1(&format!("hi").into());
        match msg {
            Msg::Data(data) => {
                console::log_1(&format!("{:?}", data).into());
                self.data = data;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let d = self.data.clone();

        // single graph (selection test)
        let (min, max) = (
            d.iter().min().unwrap().clone(),
            d.iter().max().unwrap().clone(),
        );

        let bar_width_px = 100 as f32 / d.len() as f32;

        let d = d.into_iter().map(|i| {
            let height_from_top = f32::max(
                0.0,
                100.0 - (((i - min) as f32 / (max - min) as f32) * 100.0) as f32,
            );

            console::log_1(&format!("{} {} {}", bar_width_px, height_from_top, i as i32).into());

            html! {
            <Bar
                width={bar_width_px}
                height={height_from_top}
                value={i as i32}
                />
            }
        });

        html! {
            <div style="margin: -8px;">
                {for d}
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<AsyncComponent<i32>>::new().render();
}
