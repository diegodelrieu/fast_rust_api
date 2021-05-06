use futures::executor::block_on;
use meilisearch_sdk::{client::*, document::*, search::*};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    word_id: usize,
    name: String,
}

impl Document for Word {
    type UIDType = usize;

    fn get_uid(&self) -> &Self::UIDType {
        &self.word_id
    }
}

pub fn search(word: String) -> Vec<SearchResult<Word>> {
    block_on(async move {
        let client = Client::new("http://localhost:7700", "masterKey");
        let words = client.get_or_create("words").await.unwrap();
        return words
            .search()
            .with_query(&word)
            .execute::<Word>()
            .await
            .unwrap()
            .hits;
    })
}