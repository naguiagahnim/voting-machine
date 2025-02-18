#[derive(PartialEq, Eq, Clone)]
pub struct Lexicon {
    pub blank: &'static str,
    pub candidate: &'static str,
    pub voter: &'static str,
    pub attendencesheet: &'static str,
    pub scoreboard: &'static str,
    pub ballotpaper: &'static str,
    pub voteoutcome: &'static str,
    pub invalid: &'static str,
    pub votingmachine: &'static str,
    pub accepted: &'static str,
    pub hasalready: &'static str,
}