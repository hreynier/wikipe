extern crate clap;
extern crate wikipedia;
mod args;

use args::CentipedeArgs;
use inquire::{InquireError, Select};
use wikipedia::{http::default::Client, Wikipedia};
use clap::Parser;

// Wikipedia client being used.
type WikipediaClient = Wikipedia<Client>;

fn main() {
    // Get Clap arguments
    let args = CentipedeArgs::parse();
    let query = &args.search_query;
    
    // Build the wiki client
    let mut client = build_wiki_client();

    // Fetch all available languages
    let langs = client.get_languages().expect("Failed to get languages from Wikipedia.");

    // Fetch user query, return page contents.
    if let Some(res) = centipedia(query, &mut client, &langs) {
        println!("{}", res);
    }
}

fn build_wiki_client() -> WikipediaClient {
    // Intialise wikipedia client.
    let mut client = WikipediaClient::default();
    // Fetch as many results as possible from each request.
    client.links_results = "max".into();
    client
}

fn centipedia(query: &String, client: &mut WikipediaClient, langs: &[(String, String)]) -> Option<String> {
    // Set the language to search Wikipedia in. Defaults to 'en'.
    let def_english = "en";
    let search_lang = get_search_language(&langs, Some(def_english))?;
    client.language = search_lang.clone();

    // Search for page titles.
    let titles = client.search(query).expect("Failed to search for query");

    // Interactively select the proper title and get the page contents
    if titles.is_empty() {
        eprintln!("Nothing found for ({}): {}", search_lang, query);
        return None;
    }
    let selected_page_title = select_page(&titles)?;
    let page = client.page_from_title(selected_page_title.clone());

    // Collect page contents
    let contents = page.get_content().expect("Failed to fetch page contents");
    if contents.is_empty() {
        eprintln!("No contents available for ({})", selected_page_title);
        return None;
    }
    return Some(contents.clone());
}

/// Returns the languages to search wikipedia in.
/// 
/// From the given list of 'langs', fetched from Wikipedia, and a user-supplied 'lang_pref',
/// the associated wikipedia tag for the language is returned. If pref doesn't match,
/// the default language of 'en' english is returned.
fn get_search_language(langs: &[(String, String)], lang_pref: Option<&str>) -> Option<String> {
    let def_english = String::from("en");
    // Attempt to select a Wikipedia language based on the language preference.
    if let Some(pref) = lang_pref {
        if let Some((tag, _)) = langs.iter().find(|l| l.0 == pref) {
            return Some(tag.to_owned());
        }
        eprintln!("Unknown language preference: {}", pref);
        return Some(def_english);
    }

    return Some(def_english);
}

// Show an interactive selection view for the given list of page titles.
// The selected title's page contents is returned. If no title is selected,
// return None.
fn select_page(titles: &Vec<String>) -> Option<String> {
    let titles: Vec<&str> = titles.iter().map(|s| s.as_str()).collect();
    let selected_title: Result<&str, InquireError> = Select::new("What page do you want to select?", titles).prompt();

    match selected_title {
        Ok(selected_title) => Some(selected_title.to_string()),
        Err(_) => {
            eprintln!("Error selecting page title, please try again");
            return None;
        },
    }
}