use crate::{domain::*, storage::Storage};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use anyhow::Result;

pub struct Memory {
    machine: Arc<Mutex<VotingMachine>>
}

#[async_trait]
impl Storage for Memory {
    async fn new(machine: VotingMachine) -> Result<Self> {
        Ok(Self { 
            machine: Arc::new(Mutex::new(machine))
        })
    }

    async fn get_voting_machine(&self) -> Result<VotingMachine> {
        let locked_machine = self.machine.lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;
        let candidates = locked_machine.get_scoreboard().scores.keys()
            .map(|c| Candidate(c.0.clone()))
            .collect();
        let new_scoreboard = Scoreboard::new(candidates);
        
        Ok(VotingMachine::new(new_scoreboard))
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> {
        let mut locked_machine = self.machine.lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;
        *locked_machine = machine;
        Ok(())
    }
}

mod tests {
    use super::*;

    fn setup() -> VotingMachine {
        let candidates = vec![
            Candidate("Alice".to_string()),
            Candidate("Bob".to_string()),
        ];
        let scoreboard = Scoreboard::new(candidates);
        VotingMachine::new(scoreboard)
    }

    #[tokio::test]
    async fn test_get_put_consistency() {
        let initial_machine = setup();
        let mut storage = Memory::new(initial_machine.clone()).await.unwrap();
        
        let retrieved_machine = storage.get_voting_machine().await.unwrap();
        storage.put_voting_machine(retrieved_machine.clone()).await.unwrap();
        
        let final_machine = storage.get_voting_machine().await.unwrap();
        assert_eq!(
            format!("{:?}", initial_machine),
            format!("{:?}", final_machine)
        );
    }
}