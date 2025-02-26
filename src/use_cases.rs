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
    pub fn new(store: Store) -> Self{
        Self {
            store
        }
    }

    pub async fn vote(&mut self, vote_form: VoteForm) -> anyhow::Result<VoteOutcome> {
        let ballot_paper: BallotPaper = vote_form.into();
        
        let mut voting_machine = self.store.get_voting_machine().await?;
        
        let outcome = voting_machine.vote(ballot_paper);
        
        self.store.put_voting_machine(voting_machine).await?;
        
        Ok(outcome)
    }

    pub async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        self.store.get_voting_machine().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storages::memory::Memory;

    async fn setup_controller() -> VotingController<Memory> {
        let candidates = vec![
            Candidate("Alice".to_string()),
            Candidate("Bob".to_string()),
        ];
        let scoreboard = Scoreboard::new(candidates);
        let voting_machine = VotingMachine::new(scoreboard);

        let storage = Memory::new(voting_machine).await.unwrap();
        VotingController::new(storage)
    }

    #[tokio::test]
    async fn accepted_vote() {
        let mut controller = setup_controller().await;

        let vote_form = VoteForm {
            voter: String::from("Claude"),
            candidate: String::from("Alice"),
        };

        let result = controller.vote(vote_form).await.unwrap();

        assert!(matches!(result, VoteOutcome::AcceptedVote(_, _)));
        if let VoteOutcome::AcceptedVote(voter, candidate) = result {
            assert_eq!(voter.0, "Claude");
            assert_eq!(candidate.0, "Alice");
        }
    }

    #[tokio::test]
    async fn blank_vote() {
        let mut controller = setup_controller().await;

        let vote_form = VoteForm {
            voter: String::from("Claude"),
            candidate: String::from(""),
        };

        let result = controller.vote(vote_form).await.unwrap();

        assert!(matches!(result, VoteOutcome::BlankVote(_)));
        if let VoteOutcome::BlankVote(voter) = result {
            assert_eq!(voter.0, "Claude");
        }
    }

    #[tokio::test]
    async fn invalid_vote() {
        let mut controller = setup_controller().await;

        let vote_form = VoteForm {
            voter: String::from("Claude"),
            candidate: String::from("Unknown"),
        };

        let result = controller.vote(vote_form).await.unwrap();

        assert!(matches!(result, VoteOutcome::InvalidVote(_)));
        if let VoteOutcome::InvalidVote(voter) = result {
            assert_eq!(voter.0, "Claude");
        }
    }

    #[tokio::test]
    async fn has_already_voted() {
        let mut controller = setup_controller().await;

        let vote_form1 = VoteForm {
            voter: String::from("Claude"),
            candidate: String::from("Alice"),
        };
        let result1 = controller.vote(vote_form1).await.unwrap();
        assert!(matches!(result1, VoteOutcome::AcceptedVote(_, _)));

        let vote_form2 = VoteForm {
            voter: String::from("Claude"),
            candidate: String::from("Bob"),
        };
        let result2 = controller.vote(vote_form2).await.unwrap();
        
        assert!(matches!(result2, VoteOutcome::HasAlreadyVoted(_)));
        if let VoteOutcome::HasAlreadyVoted(voter) = result2 {
            assert_eq!(voter.0, "Claude");
        }
    }
}