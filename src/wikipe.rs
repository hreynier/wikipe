use inquire::{InquireError, Select, Text};
use crate::client::WikipediaClient;

// TODO: Add enums for select actions

// enum PageMenu {
//     Summary,
//     Contents,
//     // Links,
// }

// enum AppMenu {
//     NewQuery,
//     ViewPage,
//     Quit,
// }


// Starts Wikipe function. Requires a Wikipedia client to fetch necessary article data.
// If given a starting query, will call Wikipedia client with that.
// Wikipe will allow user to sequentially call Wikipedia or move between pages.
pub fn start(starting_query: &Option<String>, client: &mut WikipediaClient) {
    let search_query = get_query(starting_query);

    // Search Wikipedia for titles matching search query.
    let titles = get_titles(&search_query, client);
    // Select title from vector of titles matching query returned from Wikipedia.
    let mut selected_title = get_selected_title(&titles, &search_query);
    while let None = selected_title {
        let search_query = get_query(&None);
        let titles = get_titles(&search_query, client);
        selected_title = get_selected_title(&titles, &search_query);
    }
    let selected_title = selected_title.unwrap();
    // Get Wikipedia Page/Article associated with that title.
    // let page = get_page(&selected_title, client);
    // Show the page's summary, whole contents, or links, depending on what the user wants.
    select_to_show_page_info(&selected_title, client);
    // User's can either submit a new query, view a different aspect of the page, or quit.
    select_menu_action(&selected_title, client);
}

// Get titles from client, given query and client.
fn get_titles(query: &String, client: &mut WikipediaClient) -> Vec<String> {
    return client.search(query).expect("Failed to search Wikipedia for query ");
}

// Interactively select and return the article title to explore.
// If no titles are found for that query, return None to restart search.
fn get_selected_title(titles: &Vec<String>, query: &String) -> Option<String> {
    if titles.is_empty() {
        eprintln!("Nothing found for ({})", query);
        return None;
    }

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

// Let's the user select to either view the page summary, contents, or links.
fn select_to_show_page_info(title: &String, client: &mut WikipediaClient) {
    let options = vec!["Summary", "Contents"];
    // let options = vec!["Summary", "Contents", "Links"];
    let choice: Result<&str, InquireError> = Select::new("Choose to view page summary, whole page contents, or the links on the page", options).prompt();
    match choice {
        Ok(view) => print_selected_view(view, title, client),
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => (),
        Err(_) => {
            println!("Failed to select a choice, please try again.");
            select_to_show_page_info(title, client);
        },
    }
}

// Prints selected information from page.
fn print_selected_view(view: &str, title: &String, client: &mut WikipediaClient) {
    if view.eq("Summary") {
        let summary = get_page_summary(title, client);
        while let None = summary {
            select_to_show_page_info(title, client);
        }
        let summary = summary.unwrap();
        println!("{}", summary);
    }
    if view.eq("Contents") {
        let contents = get_page_contents(title, client);
        while let None = contents {
            select_to_show_page_info(title, client);
        }
        let contents = contents.unwrap();
        println!("{}", contents);
    }
    // TODO Add link supports
    // if view.eq("Links") {
    //     let links = get_page_links(page, title);
    // }
}

// Given current page and title, user's can either view that page again, start a new query, or quit the application.
fn select_menu_action(title: &String, client: &mut WikipediaClient) {
    let options = vec!["New Query", "View Again", "Quit"];
    let choice: Result<&str, InquireError> = Select::new("What do you want to do now?", options).with_help_message("Start a new query, view the current page again, or quit Wikipe").prompt();
    match choice {
        Ok(action) => handle_menu_action(action, title, client),
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => (),
        Err(_) => {
            println!("Failed to select a choice, please try again.");
            select_menu_action(title, client);
        },
    }
}

// Prints selected information from page.
fn handle_menu_action(action: &str, title: &String, client: &mut WikipediaClient) {
    if action.eq("New Query") {
       start(&None, client);
    }
    if action.eq("View Again") {
        select_to_show_page_info(title, client);
    }
    if action.eq("Quit") {
        ()
    }
}

fn get_query(search_query: &Option<String>) -> String {
    match search_query {
        Some(search_query) => search_query.to_owned(),
        None => {
            get_new_query()
        },
    }
}

fn get_new_query() -> String {
    let search_query = Text::new("What do you want to search?").prompt();
            match search_query {
                Ok(search) => search,
                Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => panic!("Quitting..."),
                Err(_err) => panic!("There was an error parsing the search query, please try again"),
            }
}

// Return the page summary from the page. If no summary is available return None.
fn get_page_summary(title: &String, client: &mut WikipediaClient) -> Option<String> {
    let page = client.page_from_title(title.clone());
    let summary = page.get_summary().expect("Failed to fetch page summary");
    if summary.is_empty() {
        eprintln!("No summary available for ({})", title);
        return None;
    }
    return Some(summary.clone());
}
// Return the page contents from the page. If no contents is available return None.
fn get_page_contents(title: &String, client: &mut WikipediaClient) -> Option<String> {
    let page = client.page_from_title(title.clone());
    let contents = page.get_content().expect("Failed to fetch page contents");
    if contents.is_empty() {
        eprintln!("No contents available for ({})", title);
        return None;
    }
    return Some(contents.clone());
}

// TODO add links
// Return all links from the page. If no links are available return None.
// fn get_page_links(page: Page<Client>, title: String) -> Option<Iter<Client, Link>> {
//     let links = page.get_links().expect("Failed to fetch page links");
//     if links.is_empty() {
//         eprintln!("No links available for ({})", title);
//         return None;
//     }
//     return Some(links);
// }