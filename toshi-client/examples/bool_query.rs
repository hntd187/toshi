use serde::Deserialize;

use toshi::*;

#[derive(Clone, Deserialize)]
pub struct Wiki {
    title: String,
    url: String,
    text: String,
    rating: i32,
}

pub fn main() -> Result<()> {
    let client = ToshiClient::new("http://localhost:8080")?;
    let fuzzy_query = FuzzyQueryBuilder::new()
        .for_field("text")
        .with_value("bears")
        .with_distance(2)
        .build();
    let query = BoolQuery::builder().must_match(fuzzy_query).build();

    let search = Search::with_query(query);
    let _results: SearchResults<Wiki> = client.search("wiki", search)?;

    Ok(())
}
