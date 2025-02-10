use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Hash, Eq, PartialEq, Debug, Ord, PartialOrd, Clone)]
pub struct Voter(pub String);

#[derive(Hash, Eq, PartialEq, Debug, Ord, PartialOrd, Clone)]
pub struct Candidate(pub String);

#[derive(Debug, Clone)]
pub struct Score(pub usize);

#[derive(Debug, Clone)]
pub struct AttendanceSheet(pub Set<Voter>);

#[derive(Debug, Clone)]
pub struct Scoreboard{
    pub scores: Map<Candidate, Score>,
    pub blank_score: Score,
    pub invalid_score: Score,
}

pub struct BallotPaper {
    pub voter : Voter,
    pub candidate: Option<Candidate>,
}

pub enum VoteOutcome {
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

#[derive(Debug, Clone)]
pub struct VotingMachine{
    voters : AttendanceSheet,
    scoreboard: Scoreboard,
}

impl Scoreboard {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let mut scores = Map::new();
        for candidate in candidates {
            scores.insert(candidate, Score(0));
        }
        Scoreboard {
            scores,
            blank_score: Score(0),
            invalid_score: Score(0),
        }
    }
}

impl VotingMachine {
    pub fn new(scoreboard1 : Scoreboard) -> Self {
        VotingMachine {
            voters: AttendanceSheet(Set::new()),
            scoreboard: scoreboard1,
        }
    }

    pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
        if self.voters.0.contains(&ballot_paper.voter) {
            return VoteOutcome::HasAlreadyVoted(ballot_paper.voter.clone());
        }
    
        self.voters.0.insert(ballot_paper.voter.clone());
    
        match ballot_paper.candidate {
            None => {
                self.scoreboard.blank_score.0 += 1;
                VoteOutcome::BlankVote(ballot_paper.voter.clone())
            }
            Some(candidate) => {
                if let Some(score) = self.scoreboard.scores.get_mut(&candidate) {
                    score.0 += 1;
                    VoteOutcome::AcceptedVote(ballot_paper.voter.clone(), candidate)
                } else {
                    self.scoreboard.invalid_score.0 += 1;
                    VoteOutcome::InvalidVote(ballot_paper.voter)
                }
            }
        }
    }

    pub fn get_scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    pub fn get_voters(&self) -> &AttendanceSheet {
        &self.voters
    }

    pub fn recover_from(voters: AttendanceSheet, scoreboard: Scoreboard) -> Self {
        Self {voters, scoreboard}
    }
}

#[cfg(test)]
mod tests{
    use super::*;


    fn setup() -> VotingMachine {
        let candidates: Vec<Candidate> = vec![Candidate(String::from("Grahargul le Destructeur de Mondes")), Candidate(String::from("Jean-Marie Bigard"))];
        let scoreboard = Scoreboard::new(candidates);
        let voting_machine = VotingMachine::new(scoreboard);
        voting_machine
    }

    #[test]
    fn accepted_vote(){
        let BallotPaper = BallotPaper {
            voter: Voter(String::from("Claude")),
            candidate: Some(Candidate(String::from("Grahargul le Destructeur de Mondes"))),
        };
        let mut voting_machine = setup();
                let result = voting_machine.vote(BallotPaper);
                assert!(matches!(result, VoteOutcome::AcceptedVote(_, _)));
    }

    #[test]
    fn blank_vote(){
        let BallotPaper = BallotPaper {
            voter: Voter(String::from("Claude")),
            candidate: None,
        };
        let mut voting_machine = setup();
                let result = voting_machine.vote(BallotPaper);
                assert!(matches!(result, VoteOutcome::BlankVote(_)));
    }

    #[test]
    fn invalid_vote(){
        let BallotPaper = BallotPaper {
            voter: Voter(String::from("Claude")),
            candidate: Some(Candidate(String::from("Ouga Bouga"))),
        };
        let mut voting_machine = setup();
                let result = voting_machine.vote(BallotPaper);
                assert!(matches!(result, VoteOutcome::InvalidVote(_)));
    }

    #[test]
    fn has_already_voted() {
        let mut voting_machine = setup();
        let voter = Voter(String::from("Claude"));
        
        let ballot_paper1 = BallotPaper {
            voter: voter.clone(),
            candidate: Some(Candidate(String::from("Grahargul le Destructeur de Mondes"))),
        };
        let result1 = voting_machine.vote(ballot_paper1);
        assert!(matches!(result1, VoteOutcome::AcceptedVote(_, _)));

        let ballot_paper2 = BallotPaper {
            voter: voter.clone(),
            candidate: Some(Candidate(String::from("Jean-Marie Bigard"))),
        };
        let result2 = voting_machine.vote(ballot_paper2);
        assert!(matches!(result2, VoteOutcome::HasAlreadyVoted(_)));

        let scoreboard = voting_machine.get_scoreboard();
        assert_eq!(scoreboard.scores.get(&Candidate(String::from("Grahargul le Destructeur de Mondes"))).unwrap().0, 1);
        assert_eq!(scoreboard.scores.get(&Candidate(String::from("Jean-Marie Bigard"))).unwrap().0, 0);
    }
}