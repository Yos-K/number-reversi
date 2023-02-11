use yew::Properties;

/// Score type represents the total number of pieces on the Board for each color
#[derive(Debug, Properties, Clone, PartialEq, Copy)]
pub struct Score {
    pub black: usize,
    pub white: usize,
}