use std::io;
use crate::configuration::{Configuration, Language, StorageType};
use crate::domain::{Candidate, Voter, BallotPaper, VoteOutcome, Scoreboard, VotingMachine};
use crate::interfaces::cli_interface::handle_line;
use crate::interfaces::lexicon::Lexicon;
use crate::storage::Storage;
use crate::storages::memory::Memory;
use crate::storages::file::FileStore;
use crate::use_cases::*;

fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let candidates: Vec<Candidate> = configuration.candidates.iter().map(|c| Candidate(c.clone())).collect();
    let scoreboard = Scoreboard::new(candidates);
    VotingMachine::new(scoreboard)
}

pub async fn handle_lines<Store: Storage>(configuration: Configuration) -> anyhow::Result<()> {
    let lexicon = match configuration.language {
        Language::en => Lexicon::english(),
        Language::fr => Lexicon::french(),
    };

    println!("Bienvenue sur le serveur de vote !");
    println!("Les commandes valides sont : voter, votants et score");

    let voting_machine = create_voting_machine(&configuration);
    let store = Store::new(voting_machine).await?;
    let mut controller = VotingController::new(store);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match handle_line(&input, &mut controller, &lexicon).await {
            Ok(output) => println!("{}", output),
            Err(e) => println!("Une erreur s'est produite : {}", e),
        }
    }
}

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
    match configuration.storage {
        StorageType::Memory => handle_lines::<Memory>(configuration).await,
        StorageType::File => handle_lines::<FileStore>(configuration).await,
    }
}
