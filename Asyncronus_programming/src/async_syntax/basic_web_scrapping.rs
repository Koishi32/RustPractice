extern crate trpl; // required for mdbook test
use trpl::{Either, Html};
use std::future::Future;
pub fn basic_web_scrapping(){
    race_for_two_sites();
    another_page_example();
}

fn another_page_example(){
    let args: String= String::from("https://web-scraping.dev/");
    trpl::run(async {
        let url = args;
        let element = "nav";
        match page_title(&url,element).await.1 {
            Some(title) => println!("The {} for {url} was {title}",element),
            None => println!("{url} didn't have the element {}",element),
        }
    })
}

fn race_for_two_sites(){
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1],"title");
        let title_fut_2 = page_title(&args[2],"title");

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    });
}

async fn page_title<'a>(url: &'a str, element_to_search:&str) ->  (&'a str, Option<String>)  {
    //let response = trpl::get(url).await;
    let text = trpl::get(url).await.text().await;
    println!("searching for {}",element_to_search);
    let title = Html::parse(&text)
        .select_first(element_to_search)
        .map(|title| title.inner_html());
    (url, title)
}

// this is what page_title actually returns
fn _page_title_full(url: &str) -> impl Future<Output = Option<String>> + '_ {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}