use std::{collections::HashMap, fmt::Debug, rc::Rc, borrow::Borrow};

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
        bubble::{BubbleSort, RenderBubbleSort},
        insertion::{InsertionSort, RenderInsertionSort},
        quick::{QuickSort, RenderQuickSort},
        selection::{RenderSelectionSort, SelectionSort},
    },
    Msg,
};

use yew::{
    function_component, html, use_force_update, use_mut_ref, use_state, virtual_dom::VNode,
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
            data: gen_random_data(100),
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

pub struct RootComponent {}

impl Component for RootComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let data = gen_random_data(250);
        html! {
            <div style="margin: -8px; width: 100vw; height: 100vh; display: grid; grid-template-columns: 50% 50%; grid-auto-rows: 50vh 50vh; grid-gap: 5px;">
                <GraphComponent message_type={"insertion"} data={data.clone()}  />
                <GraphComponent message_type={"bubble"} data={data.clone()}  />
                <GraphComponent message_type={"selection"} data={data.clone()}  />
                <GraphComponent message_type={"quick"} {data}  />
            </div>
        }
    }
}

pub struct GraphComponent {
    data: Box<Vec<VNode>>,
    bar_width: f32,
    min: i32,
    max: i32,
    msg_type: String,
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct GraphComponentProperties {
    message_type: String,
    data: Vec<i32>,
}

impl Component for GraphComponent {
    type Message = Msg<i32>;
    type Properties = GraphComponentProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let d = ctx.props().data.clone();
        let bar_width_px = 100 as f32 / d.len() as f32;
        let (min, max) = (
            d.iter().min().unwrap().clone(),
            d.iter().max().unwrap().clone(),
        );
        let nodes = d
            .into_iter()
            .map(|i| {
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
            })
            .collect();

        match ctx.props().message_type.as_ref() {
            "selection" => {
                RenderSelectionSort::sort(ctx.props().data.clone(), ctx.link().callback(Msg::Data))
            }
            "quick" => {
                RenderQuickSort::sort(ctx.props().data.clone(), ctx.link().callback(Msg::Data))
            }
            "insertion" => {
                RenderInsertionSort::sort(ctx.props().data.clone(), ctx.link().callback(Msg::Data))
            }
            "bubble" => {
                RenderBubbleSort::sort(ctx.props().data.clone(), ctx.link().callback(Msg::Data))
            }
            _ => panic!("invalid prop"),
        };

        Self {
            data: Box::new(nodes),
            bar_width: bar_width_px,
            msg_type: ctx.props().message_type.clone(),
            min,
            max,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Data(data) => {
                self.data[data[0].0] = html! {
                    <Bar
                        width={self.bar_width}
                        height={f32::max(
                            0.0,
                            100.0 - (((data[0].1 - self.min) as f32 / (self.max - self.min) as f32) * 100.0) as f32,
                        )}
                        value={data[0].1 as i32}
                    />
                };
                self.data[data[1].0] = html! {
                    <Bar
                        width={self.bar_width}
                        height={f32::max(
                            0.0,
                            100.0 - (((data[1].1 - self.min) as f32 / (self.max - self.min) as f32) * 100.0) as f32,
                        )}
                        value={data[1].1 as i32}
                    />
                };
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div style="outline: 5px solid black; width: 100%; height: 100%; background-color: beige;">
                <p style="margin: 20px;">{self.msg_type.clone()}</p>
                <div style="width: 100%; height: 88%;">
                    {for self.data.clone().into_iter()}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<RootComponent>::new().render();
}
