use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CentipedeArgs{
    /// Search query used to search wikipedia via the wikipedia API.
    /// This will return a summary of the article and link if the article exists.
    pub search_query: String
}