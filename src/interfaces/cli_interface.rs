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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::{Configuration, StorageType, Language};
    use crate::storages::memory::Memory;
    use std::io::Cursor;

    fn setup() -> (VotingController<Memory>, Lexicon) {
        let config = Configuration {
            candidates: vec!["Alice".to_string(), "Bob".to_string()],
            storage: StorageType::Memory,
            language: Language::fr,
        };
        let lexicon = Lexicon::french();
        let voting_machine = create_voting_machine(&config);
        let store = Memory::new(voting_machine);
        let controller = VotingController::new(store);
        (controller, lexicon)
    }

    #[tokio::test]
    async fn test_invalid_command() {
        let (mut controller, lexicon) = setup();
        let result = handle_line("commande_invalide", &mut controller, &lexicon).await.unwrap();
        assert_eq!(result, "invalide ! bulletin de vote : voter, votants ou score");
    }

    #[tokio::test]
    async fn test_show_voters() {
        let (mut controller, lexicon) = setup();
        let result = handle_line("votants", &mut controller, &lexicon).await.unwrap();
        assert_eq!(result, "feuille de présence :\n");
    }

    #[tokio::test]
    async fn test_show_scores() {
        let (mut controller, lexicon) = setup();
        let result = handle_line("score", &mut controller, &lexicon).await.unwrap();
        assert_eq!(result, "tableau des scores :\n• Alice : 0\n• Bob : 0\n• blanc : 0\n• invalide : 0");
    }

    #[tokio::test]
    async fn test_vote() {
        let (mut controller, lexicon) = setup();
        
        let input = Cursor::new("Alice\nBob\n");
        std::io::set_stdin(input);
        
        let result = handle_line("voter", &mut controller, &lexicon).await.unwrap();
        assert_eq!(result, "résultat de vote accepté Bob");
    }

    #[tokio::test]
    async fn test_blank_vote() {
        let (mut controller, lexicon) = setup();
        
        let input = Cursor::new("Alice\n\n");
        std::io::set_stdin(input);
        
        let result = handle_line("voter", &mut controller, &lexicon).await.unwrap();
        assert_eq!(result, "résultat de vote blanc accepté");
    }

    #[tokio::test]
    async fn test_missing_voter() {
        let (mut controller, lexicon) = setup();
        
        let input = Cursor::new("\nBob\n");
        std::io::set_stdin(input);
        
        let result = handle_line("voter", &mut controller, &lexicon).await.unwrap();
        assert_eq!(result, "résultat de vote invalide accepté");
    }
}
