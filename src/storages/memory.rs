use crate::{domain::*, storage::Storage};
use async_trait::async_trait;
use anyhow::Result;

pub struct Memory {
    machine: VotingMachine
}

#[async_trait]
impl Storage for Memory {
    async fn new(machine: VotingMachine) -> Result<Self> {
        Ok(Self { 
            machine
        })
    }

    async fn get_voting_machine(&self) -> Result<VotingMachine> {
        Ok(self.machine.clone())
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> {
        self.machine = machine;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

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