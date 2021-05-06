use futures::executor::block_on;
use meilisearch_sdk::{client::*, document::*, progress::*, indexes::*};
use serde::{Deserialize, Serialize};
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

pub fn main() {
    block_on(async move {
        let client = Client::new("http://localhost:7700", "masterKey");
        let words: Index = client.get_index("words").await.unwrap();
        words.delete().await.unwrap();
        let words = client.get_or_create("words").await.unwrap();
        let filename = "../data/thes_fr.dat";
        println!("In file {}", filename);

        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let split_content = contents.split("\n");
        let vec = split_content.collect::<Vec<&str>>();
        for (i, word) in vec.iter().enumerate() {
            if i % 2 != 0 && i + 1 != vec.len() {
                let synonyms = vec[i + 1];
                let cleaned_word = word.split('|').collect::<Vec<&str>>()[0];
                let mut cleaned_synonyms = synonyms.split('|').collect::<Vec<&str>>();
                cleaned_synonyms.remove(0);
                let string_synonyms = cleaned_synonyms.iter().map(|s| s.to_string()).collect();
                println!("word:{}, synonyms: {:#?}", cleaned_word, string_synonyms);
                words
                    .add_documents(
                        &[Word {
                            word_id: i,
                            name: cleaned_word.to_string(),
                        }],
                        Some("word_id"),
                    )
                    .await
                    .unwrap();
                let mut synonyms = std::collections::HashMap::new();
                synonyms.insert(cleaned_word.to_string(), string_synonyms);
                let progress: Progress = words.set_synonyms(&synonyms).await.unwrap();
            }
        }
    }) 
}
