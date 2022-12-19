use clap::{arg, Parser};

#[derive(Debug, Default,Parser)]
#[clap(author="Harvey Reynier", version="0.1.0")]
/// A Wikipedia search tool.
pub struct WikipeArgs{
    /// The search query used to search Wikipedia for articles.
    /// Wikipe will return a list of articles that match the query.
    /// After selecting the article, wikipe will print the article's content.
    pub search: Option<String>,
    
    /// The language to search Wikipedia in.
    /// Defaults to English 'en' if not supplied or language not supported by Wikipedia.
    #[arg(default_value_t=String::from("en"), short, long)]
    pub lang: String,
}