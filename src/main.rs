use std::process;

use args::WikipeArgs;
use clap::Parser;
use client::WikipediaClient;

mod args;
mod wikipe;
mod client;

fn main() {
    // Get Clap arguments
    let args = WikipeArgs::parse();
    let starting_query = &args.search;
    let search_lang_pref = &args.lang;
    
    // Build the wiki client
    let mut client = client::build_wiki_client(search_lang_pref);

    // Start Wikipe
    start_wikipe(starting_query, &mut client);

    exit_wikipe();
}


fn exit_wikipe() {
    println!("Quitting Wikipe...");
    process::exit(1);
}

fn start_wikipe(starting_query: &Option<String>, client: &mut WikipediaClient) {
    wikipe::start(starting_query, client);
}