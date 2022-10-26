use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq, Debug)]
pub struct BarProps {
    pub width: f32,
    pub height: f32,
    pub value: i32,
}

#[function_component(Bar)]
pub fn bar(props: &BarProps) -> Html {
    let (width, height, _value) = (props.width, props.height, props.value);
    html! {
        <div style={format!("height: 100%; width: {}%; display:inline; float: left;", width)}>
        <div
            style={format!(
                "height: calc(100% - {}%);",
                100.0 - height,
                )}>
        </div>
        <div
            style={format!(
                "outline: 0.1px solid black; outline-offset: -0.1px; height: calc(100% - {}%); background-color: cyan;",
                 height,
                )}>
        </div>
        </div>
    }
}
