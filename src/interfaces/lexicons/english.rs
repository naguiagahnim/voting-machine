use crate::interfaces::lexicon::Lexicon;

impl Lexicon {
    fn english() -> Self {
        Self {
            blank: "blank",
            candidate: "candidate",
            voter: "voter",
            attendencesheet: "attendence sheet",
            scoreboard: "scoreboard",
            ballotpaper: "ballot paper",
            voteoutcome: "vote outcome",
            invalid: "invalid",
            votingmachine: "voting machine",
            accepted: "accepted",
            hasalready: "has already",
        }
    }
}