use futures::executor::block_on;
use meilisearch_sdk::{client::*, document::*, search::*};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    book_id: usize,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    word_id: usize,
    name: String,
}

// That trait is required to make a struct usable by an index
impl Document for Word {
    type UIDType = usize;

    fn get_uid(&self) -> &Self::UIDType {
        &self.word_id
    }
}

// pub fn db_init() { block_on(async move {
//     // Add some books in the index
//     books.add_documents(&[
//         Book{book_id: 123,  title: String::from("Pride and Prejudice")},
//         Book{book_id: 456,  title: String::from("Le Petit Prince")},
//         Book{book_id: 1,    title: String::from("Alice In Wonderland")},
//         Book{book_id: 1344, title: String::from("The Hobbit")},
//         Book{book_id: 4,    title: String::from("Harry Potter and the Half-Blood Prince")},
//         Book{book_id: 42,   title: String::from("The Hitchhiker's Guide to the Galaxy")},
//     ], Some("book_id")).await.unwrap();

//     // Query books (note that there is a typo)
//     println!("{:?}", books.search().with_query("harry pottre").execute::<Book>().await.unwrap().hits);
// })}
//-> meilisearch_sdk::search::SearchResult<
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

pub fn load() {
    let filename = "./data/test.dat";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
