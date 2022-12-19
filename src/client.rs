extern crate wikipedia;

use wikipedia::{http::default::Client, Wikipedia};

// Wikipedia client being used.
pub type WikipediaClient = Wikipedia<Client>;

pub fn build_wiki_client(search_lang_pref: &String) -> WikipediaClient {
    // Intialise wikipedia client.
    let mut client = WikipediaClient::default();
    // Fetch as many results as possible from each request.
    client.links_results = "max".into();
    // Set client language
    set_wikipedia_client_language(&mut client, search_lang_pref);
    client
}

// Set the language to search Wikipedia in. Defaults to English/en.
fn set_wikipedia_client_language(client: &mut WikipediaClient, search_lang_pref: &String) {
    // Fetch all available languages
    let langs = client.get_languages().expect("Failed to get languages from Wikipedia.");

    let search_lang = get_search_language(&langs, Some(&search_lang_pref.as_str()));
    client.language = search_lang.clone();
}

/// Returns the languages to search wikipedia in.
/// 
/// From the given list of 'langs', fetched from Wikipedia, and a user-supplied 'lang_pref',
/// the associated wikipedia tag for the language is returned. If pref doesn't match,
/// the default language of 'en' english is returned.
fn get_search_language(langs: &[(String, String)], lang_pref: Option<&str>) -> String {
    let def_english = String::from("en");
    // Attempt to select a Wikipedia language based on the language preference.
    if let Some(pref) = lang_pref {
        if let Some((tag, _)) = langs.iter().find(|l| l.0 == pref) {
            return tag.to_owned();
        }
        eprintln!("Unknown language preference: {}", pref);
        return def_english;
    }

    return def_english;
}

