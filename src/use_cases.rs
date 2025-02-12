use serde::Deserialize;

use crate::{domain::*, storage::*};

#[derive(Deserialize)]
pub struct VoteForm {
    pub voter: String,
    pub candidate: String
}

pub struct VotingController<Store> {
    store: Store,
}

impl From<VoteForm> for BallotPaper {
    fn from(voteform: VoteForm) -> Self {
        Self {
            voter: Voter(voteform.voter),
            candidate: if voteform.candidate.is_empty(){
                None
            } else {
                Some(Candidate(voteform.candidate))
            }
        }
    }
}

impl <Store: Storage> VotingController<Store> {
    //pub fn new(store: Store) -> Self

    //pub async fn vote(&mut self, vote_form: VoteForm) -> anyhw::Result<VoteOutcome>

    //pub async fn get_voting_macchine(&self) -> anyhow::Result<VotingMachine>
}