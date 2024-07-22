use serde::Deserialize;

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
    let default_case = Quote {
        message: "grug say prototype early in software making, especially if many big brains"
            .to_string(),
        author: "grugbrain.dev".to_string(),
    };

    println!("{}", default_case.formatted())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let quotes: Vec<Quote> = load_file("quotes.json");
        assert!(quotes.len() > 0)
    }
}
