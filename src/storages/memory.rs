use crate::{domain::*, storage::Storage};
use async_trait::async_trait;

struct Memory{
    scoreboard: Scoreboard,
    machine: VotingMachine
}

#[async_trait]
impl Storage for Memory{
    async fn new(machine: VotingMachine) -> anyhow::Result<Self>;
    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine>;
    async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()>;
}