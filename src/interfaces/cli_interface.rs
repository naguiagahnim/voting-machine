use std::io;
use crate::configuration::Configuration;
use crate::domain::{Candidate, VoteOutcome, Scoreboard, VotingMachine, AttendanceSheet};
use crate::storage::Storage;
use crate::use_cases::*;

fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let candidates: Vec<Candidate> = configuration.candidates.iter().map(|c| Candidate(c.clone())).collect();
    let scoreboard = Scoreboard::new(candidates);
    VotingMachine::new(scoreboard)
}

fn show_vote_outcome(outcome: VoteOutcome) -> String {
    match outcome {
        VoteOutcome::AcceptedVote(_, c) => format!("Vote enregistré pour {}", c.0),
        VoteOutcome::BlankVote(_) => "Vote blanc enregistré".to_string(),
        VoteOutcome::InvalidVote(_) => "Vote nul enregistré (candidat non trouvé)".to_string(),
        VoteOutcome::HasAlreadyVoted(_) => "Vous avez déjà voté !".to_string(),
    }
}

fn show_scoreboard(scoreboard: &Scoreboard) -> String {
    let mut result = String::from("Scores actuels :\n");
    for (candidate, score) in &scoreboard.scores {
        result.push_str(&format!("• {} : {}\n", candidate.0, score.0));
    }
    result.push_str(&format!("• Blanc : {}\n", scoreboard.blank_score.0));
    result.push_str(&format!("• Nul : {}", scoreboard.invalid_score.0));
    result
}

fn show_attendance_sheet(attendance_sheet: &AttendanceSheet) -> String {
    let mut result = String::from("Liste des votants :\n");
    for votant in &attendance_sheet.0 {
        result.push_str(&format!("• {}\n", votant.0));
    }
    result
}

pub async fn handle_line<Store: Storage>(input: &str, controller: &mut VotingController<Store>) -> anyhow::Result<String> {
    match input.trim() {
        "voter" => {
            println!("Quel est votre nom ?");
            let mut voter_name = String::new();
            io::stdin().read_line(&mut voter_name)?;

            println!("Pour qui voulez-vous voter ? (Laissez vide pour un vote blanc)");
            let mut candidate_name = String::new();
            io::stdin().read_line(&mut candidate_name)?;

            let vote_form = VoteForm {
                voter: voter_name.trim().to_string(),
                candidate: candidate_name.trim().to_string(),
            };

            let outcome = controller.vote(vote_form).await?;
            Ok(show_vote_outcome(outcome))
        },
        "votants" => {
            let voting_machine = controller.get_voting_machine().await?;
            Ok(show_attendance_sheet(&voting_machine.get_voters()))
        },
        "score" => {
            let voting_machine = controller.get_voting_machine().await?;
            Ok(show_scoreboard(&voting_machine.get_scoreboard()))
        },
        _ => Ok("Commande invalide ! Les commandes valides sont : voter, votants ou score".to_string()),
    }
}