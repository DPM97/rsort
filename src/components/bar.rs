use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq, Debug)]
pub struct BarProps {
    pub width: f32,
    pub height: f32,
    pub value: i32,
}

#[function_component(Bar)]
pub fn bar(props: &BarProps) -> Html {
    let color = "#0385ff";

    let (width, height, _value) = (props.width, props.height, props.value);
    html! {
        <div style={format!("height: 100vh; width: {}vw; display:inline; float: left;", width)}>
        <div
            style={format!(
                "height: calc(100vh - {}vh);width: {}vw;",
                100.0 - height,
                width,
                )}>
        </div>
        <div
            style={format!(
                "outline: 0.1px solid black; outline-offset: -0.1px; height: calc(100vh - {}vh);width: {}vw; background-color: {};",
                 height,
                 width,
                 color
                )}>
        </div>
        </div>
    }
}
