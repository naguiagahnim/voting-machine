use std::io;
use crate::configuration::Configuration;
use crate::domain::{Candidate, VoteOutcome, Scoreboard, VotingMachine, AttendanceSheet};
use crate::storage::Storage;
use crate::use_cases::*;

use super::lexicon::Lexicon;

fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let candidates: Vec<Candidate> = configuration.candidates.iter().map(|c| Candidate(c.clone())).collect();
    let scoreboard = Scoreboard::new(candidates);
    VotingMachine::new(scoreboard)
}

fn show_vote_outcome(outcome: VoteOutcome, lexicon: &Lexicon) -> String {
    match outcome {
        VoteOutcome::AcceptedVote(_, c) => format!("{} {} {}", lexicon.voteoutcome, lexicon.accepted, c.0),
        VoteOutcome::BlankVote(_) => format!("{} {} {}", lexicon.voteoutcome, lexicon.blank, lexicon.accepted),
        VoteOutcome::InvalidVote(_) => format!("{} {} {}", lexicon.voteoutcome, lexicon.invalid, lexicon.accepted),
        VoteOutcome::HasAlreadyVoted(_) => format!("{} {} {} !", lexicon.voter, lexicon.hasalready, lexicon.voteoutcome),
    }
}

fn show_scoreboard(scoreboard: &Scoreboard, lexicon: &Lexicon) -> String {
    let mut result = format!("{} :\n", lexicon.scoreboard);
    for (candidate, score) in &scoreboard.scores {
        result.push_str(&format!("• {} : {}\n", candidate.0, score.0));
    }
    result.push_str(&format!("• {} : {}\n", lexicon.blank, scoreboard.blank_score.0));
    result.push_str(&format!("• {} : {}", lexicon.invalid, scoreboard.invalid_score.0));
    result
}

fn show_attendance_sheet(attendance_sheet: &AttendanceSheet, lexicon: &Lexicon) -> String {
    let mut result = format!("{} :\n", lexicon.attendencesheet);
    for votant in &attendance_sheet.0 {
        result.push_str(&format!("• {}\n", votant.0));
    }
    result
}

pub async fn handle_line<Store: Storage>(input: &str, controller: &mut VotingController<Store>, lexicon: &Lexicon) -> anyhow::Result<String> {
    match input.trim() {
        "voter" => {
            println!("{} ?", lexicon.voter);
            let mut voter_name = String::new();
            io::stdin().read_line(&mut voter_name)?;

            println!("{} ? ({})", lexicon.candidate, lexicon.blank);
            let mut candidate_name = String::new();
            io::stdin().read_line(&mut candidate_name)?;

            let vote_form = VoteForm {
                voter: voter_name.trim().to_string(),
                candidate: candidate_name.trim().to_string(),
            };

            let outcome = controller.vote(vote_form).await?;
            Ok(show_vote_outcome(outcome, lexicon))
        },
        "votants" => {
            let voting_machine = controller.get_voting_machine().await?;
            Ok(show_attendance_sheet(&voting_machine.get_voters(), lexicon))
        },
        "score" => {
            let voting_machine = controller.get_voting_machine().await?;
            Ok(show_scoreboard(&voting_machine.get_scoreboard(), lexicon))
        },
        _ => Ok(format!("{} ! {} : voter, votants ou score", lexicon.invalid, lexicon.ballotpaper)),
    }
}
