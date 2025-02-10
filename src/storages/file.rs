use std::collections::{BTreeMap as Map, BTreeSet as Set};

use anyhow::Result;
use async_trait::async_trait;
use tokio::{fs::File, io::AsyncWriteExt};
use crate::{domain::*, storage::Storage};
use serde::{Deserialize, Serialize};

const FILEPATH: &str = "machine.json";

#[derive(Serialize, Deserialize)]
struct ScoreboardDAO {
    scores: Map<String, usize>,
    blank_score: usize,
    invalid_score: usize,
}

#[derive(Serialize, Deserialize)]
pub struct VotingMachineDAO{
    voters: Set<String>,
    scoreboard: ScoreboardDAO,
}

pub struct FileStore{
    filepath: String,
}

impl From<Scoreboard> for ScoreboardDAO {
    fn from(scoreboard: Scoreboard) -> Self {
        let mut scores = Map::new();
        for (candidate, score) in &scoreboard.scores {
            scores.insert(
                candidate.0.clone(),
                score.0 as usize,   
            );
        };
        ScoreboardDAO {
            scores,
            blank_score: scoreboard.blank_score.0 as usize,
            invalid_score: scoreboard.invalid_score.0 as usize,
        }
    }
}

impl From<ScoreboardDAO> for Scoreboard {
    fn from(scoreboardDAO : ScoreboardDAO) -> Self {
        let mut scores = Map::new();
        for(candidate, score) in scoreboardDAO.scores {
            scores.insert(
                Candidate(candidate),
                Score(score as usize),
            );
        };
        Scoreboard {
            scores,
            blank_score: Score(scoreboardDAO.blank_score as usize),
            invalid_score: Score(scoreboardDAO.invalid_score as usize),
        }
    }
}

impl From<VotingMachine> for VotingMachineDAO {
    fn from(votingmachine: VotingMachine) -> Self {
        let voters_machine = VotingMachine::get_voters(&votingmachine);
        let scoreboard_machine = VotingMachine::get_scoreboard(&votingmachine);

        let mut voters = Set::new();
        for voter in &voters_machine.0 {
            voters.insert(voter.0.clone());
        };
        let scoreboardnew = ScoreboardDAO::from(scoreboard_machine.clone());
        VotingMachineDAO {
            voters,
            scoreboard: scoreboardnew,
        }
    }
}

impl From<VotingMachineDAO> for VotingMachine {
    fn from(votingmachineDAO: VotingMachineDAO) -> Self {
        let mut voters = Set::new();
        for voter in votingmachineDAO.voters {
            voters.insert(Voter(voter));
        }
        let mut scoreboardnew = Scoreboard::from(votingmachineDAO.scoreboard);
        VotingMachine ::recover_from(AttendanceSheet(voters), scoreboardnew)
    }
}


impl FileStore {
    pub async fn create(machine: VotingMachine, filepath: &str) -> anyhow::Result<Self>{
        let _ = match File::open(filepath).await {
                    Ok(file) => file,
                    Err(_) => {
                        let mut file = File::create(filepath).await?;
                        let machine_dao = VotingMachineDAO::from(machine);
                        let json = serde_json::to_vec(&machine_dao)?;
                        file.write_all(&json).await?;
                        file.flush().await?;
                        File::open(filepath).await?
                    }
                };
        
                Ok(Self {
                    filepath: filepath.to_string(),
                })
        
    }
}

#[async_trait]
impl Storage for FileStore {
    async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
        FileStore::create(machine, FILEPATH).await
    }

    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        let file = File::open(&self.filepath).await?;
        let machine_dao: VotingMachineDAO = serde_json::from_reader(file.into_std().await)?;
        Ok(machine_dao.into())
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
        let machine_dao: VotingMachineDAO = machine.into();
        let json = serde_json::to_vec(&machine_dao)?;
        let mut file = File::create(&self.filepath).await?;
        file.write_all(&json).await?;
        file.flush().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    fn setup_voting_machine() -> VotingMachine {
        let mut scores = Map::new();
        scores.insert(Candidate("Alice".to_string()), Score(10));
        scores.insert(Candidate("Bob".to_string()), Score(5));
        let scoreboard = Scoreboard {
            scores,
            blank_score: Score(2),
            invalid_score: Score(1),
        };

        let voters = AttendanceSheet(
            vec![Voter("John".to_string()), Voter("Jane".to_string())]
                .into_iter()
                .collect(),
        );

        VotingMachine::recover_from(voters, scoreboard)
    }

    #[tokio::test]
    async fn test_get_returns_same_as_put() -> Result<()> {
        let filepath = "test_machine.json";
        let voting_machine = setup_voting_machine();

        let _ = fs::remove_file(filepath).await;

        let mut file_store = FileStore::create(voting_machine.clone(), filepath).await?;
        file_store.put_voting_machine(voting_machine.clone()).await?;
        let retrieved_machine = file_store.get_voting_machine().await?;

        let dao1 = VotingMachineDAO::from(voting_machine.clone());
        let dao2 = VotingMachineDAO::from(retrieved_machine);
        assert_eq!(dao1.voters, dao2.voters, "Les votants ne correspondent pas");
        assert_eq!(dao1.scoreboard.scores, dao2.scoreboard.scores, "Les scores ne correspondent pas");
        assert_eq!(dao1.scoreboard.blank_score, dao2.scoreboard.blank_score, "Les scores blancs ne correspondent pas");
        assert_eq!(dao1.scoreboard.invalid_score, dao2.scoreboard.invalid_score, "Les scores invalides ne correspondent pas");


        let _ = fs::remove_file(filepath).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_file_persistence_between_instances() -> Result<()> {
        let filepath = "test_persistence.json";
        let voting_machine = setup_voting_machine();

        let _ = fs::remove_file(filepath).await;

        {
            let mut file_store_1 = FileStore::create(voting_machine.clone(), filepath).await?;
            file_store_1.put_voting_machine(voting_machine.clone()).await?;
        }

        {
            let file_store_2 = FileStore::create(voting_machine.clone(), filepath).await?;
            let retrieved_machine = file_store_2.get_voting_machine().await?;

            let dao1 = VotingMachineDAO::from(voting_machine.clone());
            let dao2 = VotingMachineDAO::from(retrieved_machine);
            assert_eq!(dao1.voters, dao2.voters, "Les votants ne correspondent pas");
            assert_eq!(dao1.scoreboard.scores, dao2.scoreboard.scores, "Les scores ne correspondent pas");
            assert_eq!(dao1.scoreboard.blank_score, dao2.scoreboard.blank_score, "Les scores blancs ne correspondent pas");
            assert_eq!(dao1.scoreboard.invalid_score, dao2.scoreboard.invalid_score, "Les scores invalides ne correspondent pas");

        }

        let _ = fs::remove_file(filepath).await;
        Ok(())
    }
}

