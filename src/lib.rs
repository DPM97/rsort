pub mod components;
pub mod sorts;

pub enum Msg<T> {
    Data(Vec<T>),
}
