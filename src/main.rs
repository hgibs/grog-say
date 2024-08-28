use rand::prelude::*;
use rand::Rng;
use serde::Deserialize;
use std::fmt::Debug;
use std::fs;

#[derive(Deserialize, Clone, Debug)]
struct Quote {
    message: String,
    author: String,
}

impl Quote {
    pub fn formatted(&self) -> String {
        format!("\"{}\" \n - {}", self.message, self.author)
    }
}

fn main() {
    let quotes = load_quotes_file("quotes.json");
    let mut small_rng = SmallRng::from_entropy();
    let selected_quote = pick_element(&quotes, &mut small_rng);
    println!("{}", selected_quote.formatted())
}

fn load_quotes_file(file_path: &str) -> Vec<Quote> {
    let file_contents = fs::read_to_string(file_path)
        .expect(format!("Problem opening the file: {}", file_path).as_str());
    match serde_json::from_str(file_contents.as_str()) {
        Ok(result) => result,
        Err(error) => panic!("Problem deserializing the json: {error:?}"),
    }
}

fn pick_element<T: Clone>(list: &Vec<T>, random_gen: &mut SmallRng) -> T {
    let el_idx = random_gen.gen_range(0..list.len());
    list[el_idx].clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    #[test]
    fn test_simple_derive() {
        let data = r#"
            {
                "message": "something",
                "author": "@hgibs"
            }
        "#;
        let single: Quote = serde_json::from_str(data).expect("bad test data");
        assert_eq!("something", single.message);

        println!("\"{}\" - {}", single.message, single.author);
    }

    #[test]
    fn test_load_file() {
        let quotes: Vec<Quote> = load_quotes_file("quotes.json");
        debug!("{:?}", quotes);
        assert!(!quotes.is_empty());
    }

    #[test]
    #[should_panic(
        expected = "Problem deserializing the json: Error(\"missing field `message`\", line: 5, column: 3)"
    )]
    fn test_bad_file() {
        let _quotes: Vec<Quote> = load_quotes_file("wrongschema.json");
    }
}
