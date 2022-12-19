<div align="center">
    <img width="200" src="https://upload.wikimedia.org/wikipedia/commons/2/29/Wikipe-tan_without_body.png" alt="wikipe-tan">
    <h1>Wikipe</h1>
    <p>
        <a href="https://crates.io/crates/wikipe"><img src="https://img.shields.io/crates/v/wikipe?label=Version" alt="Version" /></a>
        <a href="https://crates.io/crates/wikipe"><img src="https://img.shields.io/crates/d/wikipe?label=Downloads" alt="Downloads"></a>
        <a href="https://github.com/hreynier/wikipe/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-green.svg?maxAge=2592000" alt="License" /></a>
    </p>
</div>

:sparkle: Wikipe :sparkle: is a tool to search Wikipedia from the command-line. Given a search query, it returns a list of matching Wikipedia articles, which can be opened to display their contents or directly opened in a browser from the terminal. Articles can be navigated through the terminal. `wikipe` supports sequential searches within a single instance.

## Installation

If you have Rust and Cargo installed on your machine, you can install this app directly from crates.io.
```
 $   cargo install wikipe
```


## Usage

Wikipe currently takes the following arguments.

```
    -l|--lang [String] Wikipedia article language in the form of an ISO language code
```

You can run wikipe directly from the terminal with the `wikipe` command alongside a query to search on Wikipedia.
You can supply a language ISO code with the `-l` flag. For example, by setting the language to `es`, Wikipe will return articles in Spanish.

```
$   wikipe <search_query> -l <language>
```

Wikipe will return a list of relevant articles for that given search term, and ask you to select an article to view.
At the moment, you can choose to view a summary of the article, or the entire article in your terminal.
Check the [roadmap](##Roadmap) for upcoming features.

## Contributing
This is still a WIP so if you find any bugs or have any suggestions for features or better code, please submit either an issue or a pull request ^_^

## Naming
Wikipe is named after Wikipedia's unofficial mascot, [Wikipe-tan](https://en.wikipedia.org/wiki/Wikipedia:Wikipe-tan)! </br>
Wikipe-tan was created in 2006 and since then her popularity as a Wikipedia mascot has sadly dwindled. Hopefully you can have some fun exploring the world's encyclopedia with Wikipe-tan again.

## Roadmap
- **Link Support:** Browse links on a Wikipedia article.
- **Better Navigation:** A better navigation experience, to allow navigation between pages like you would on Wikipedia.
- **TUI:** Convert the app to a full TUI experience, using frameworks such as [Rust-TUI](https://github.com/fdehau/tui-rs) or [Cursive](https://github.com/gyscos/Cursive). This would allow for better navigation and rendering of article information.
- **Image Support:** Render the main image from articles in ASCII within the TUI.

## Motivation
I've been personally wanting to learn Rust for a while now so whilst I had some free time I thought I'd create a simple CLI app.
With that being said, **I'm not a Rust export** so if you have any suggestions or improvements in terms of code quality, please open a pull request or an issue and I'll endeavor to answer and help in any way I can! I'm always looking to improve.
**Why Wikipedia?**
I use Wikipedia every single day. It's by far the most useful and valuable part of the internet for me, and I wanted to celebrate that by creating this app. If this app brought you some value or joy, and you wanted to support it any way, I'd ask if you can instead [donate to Wikipedia directly](https://donate.wikimedia.org/wiki/Ways_to_Give) via their donation page!



