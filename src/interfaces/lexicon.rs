#[derive(PartialEq, Eq, Clone)]
pub struct Lexicon {
    pub blank: &'static str,
    pub candidate: &'static str,
    pub voter: &'static str,
}