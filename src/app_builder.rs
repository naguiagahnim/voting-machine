use std::io;
use crate::configuration::{Configuration, StorageType};
use crate::domain::{Candidate, Voter, BallotPaper, VoteOutcome, Scoreboard, VotingMachine};
use crate::storage::Storage;
use crate::storages::memory::Memory;
use crate::storages::file::FileStore;

fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let candidates: Vec<Candidate> = configuration.candidates.iter().map(|c| Candidate(c.clone())).collect();
    let scoreboard = Scoreboard::new(candidates);
    VotingMachine::new(scoreboard)
}

pub async fn handle_lines<Store: Storage>(configuration: Configuration) -> anyhow::Result<()> {
    println!("Bienvenue sur le serveur de vote !");
    println!("Les commandes valides sont : voter, votants ou score");

    let voting_machine = create_voting_machine(&configuration);
    let mut storage = Store::new(voting_machine).await?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "voter" => {
                println!("Quel est votre nom ?");
                let mut votant_name = String::new();
                io::stdin().read_line(&mut votant_name)?;
                let votant = Voter(votant_name.trim().to_string());

                println!("Pour qui voulez-vous voter ? (Laissez vide pour un vote blanc)");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom)?;
                let candidate_name = nom.trim().to_string();
                let candidate = if candidate_name.is_empty() { None } else { Some(Candidate(candidate_name)) };

                let mut voting_machine = storage.get_voting_machine().await?;

                let ballot = BallotPaper { voter: votant.clone(), candidate };
                match voting_machine.vote(ballot) {
                    VoteOutcome::AcceptedVote(_, c) => println!("Vote enregistré pour {}", c.0),
                    VoteOutcome::BlankVote(_) => println!("Vote blanc enregistré"),
                    VoteOutcome::InvalidVote(_) => println!("Vote nul enregistré (candidat non trouvé)"),
                    VoteOutcome::HasAlreadyVoted(_) => println!("Vous avez déjà voté !"),
                }

                storage.put_voting_machine(voting_machine).await?;
            },
            "votants" => {
                let voting_machine = storage.get_voting_machine().await?;
                
                println!("Liste des votants :");
                for votant in &voting_machine.get_voters().0 {
                    println!("• {}", votant.0);
                }
            },
            "score" => {
                let voting_machine = storage.get_voting_machine().await?;

                println!("Scores actuels :");
                for (candidate, score) in &voting_machine.get_scoreboard().scores {
                    println!("• {} : {}", candidate.0, score.0);
                }
                println!("• Blanc : {}", voting_machine.get_scoreboard().blank_score.0);
                println!("• Nul : {}", voting_machine.get_scoreboard().invalid_score.0);
            },
            _ => println!("Commande invalide ! Les commandes valides sont : voter, votants ou score"),
        }
    }
}

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
    match configuration.storage {
        StorageType::Memory => handle_lines::<Memory>(configuration).await,
        StorageType::File => handle_lines::<FileStore>(configuration).await,
    }
}
