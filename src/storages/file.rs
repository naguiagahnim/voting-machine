pub struct FileStore{
    filepath: String,
}

impl FileStore {
    pub async fn create(machine: VotingMachine, filepath: &str) -> anyhow::Result<Self>
}