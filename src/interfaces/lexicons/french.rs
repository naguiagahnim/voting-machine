use crate::interfaces::lexicon::Lexicon;

impl Lexicon {
    fn french() -> Self {
        Self {
            blank: "blanc",
            candidate: "candidate",
            voter: "voteur",
            attendencesheet: "feuille de présence",
            scoreboard: "tableau des scores",
            ballotpaper: "bulletin de vote",
            voteoutcome: "résultat de vote",
            invalid: "invalide",
            votingmachine: "machine à vote",
            accepted: "accepté",
            hasalready: "a déjà",
        }
    }
}