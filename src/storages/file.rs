use std::collections::{BTreeMap as Map, BTreeSet as Set};

use anyhow::Ok;
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
        let file = match File::open(filepath).await {
                    Ok(file) => file,
                    Err(_) => {
                        let mut file = File::create(filepath).await?;
                        let json = serde_json::to_vec(&machine)?;
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
    async fn new(machine: VotingMachine) -> Result<Self> {
        FileStore::create(machine, FILEPATH).await
    }

    async fn get_voting_machine(&self) -> Result<VotingMachine> {
        let file = File::open(&self.filepath).await?;
        let machine_dao: VotingMachineDAO = serde_json::from_reader(file.into_std().await)?;
        Ok(machine_dao.into())
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> {
        let machine_dao: VotingMachineDAO = machine.into();
        let json = serde_json::to_vec(&machine_dao)?;
        let mut file = File::create(&self.filepath).await?;
        file.write_all(&json).await?;
        file.flush().await?;
        Ok(())
    }
}
