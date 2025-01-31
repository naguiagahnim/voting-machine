use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Hash, Eq, PartialEq, Debug, Ord, PartialOrd, Clone)]
pub struct Voter(pub String);

#[derive(Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct Candidate(pub String);

pub struct Score(pub usize);

pub struct AttendanceSheet(pub Set<Voter>);

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
}