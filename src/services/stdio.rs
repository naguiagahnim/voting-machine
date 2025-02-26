use crate::{interfaces::lexicon::Lexicon, service::Service, storage::Storage, use_cases::VotingController};

pub struct StdioService<Store> {
    lexicon: Lexicon,
    controller: VotingController<Store>,
}

impl<Store: Storage + Send + Sync> Service<Store> for StdioService<Store> {
    ...
}