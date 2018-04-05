extern crate clap;
extern crate reqwest;
extern crate serde;
mod about;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate prettytable;

fn print_feed_table<'feed, I: Iterator<Item = &'feed FeedItem>>(items: I) {
    let mut table = prettytable::Table::new();

    table.add_row(row!["Title", "Author", "Link"]);

    for item in items {
        let title = if item.title.len() >= 50 {
            &item.title[0..49]
        } else {
            &item.title
        };

        table.add_row(row![title, item.author.name, item.url]);
    }

    table.printstd();
}

fn print_feed_table2<'feed2, I: Iterator<Item = &'feed2 FeedItem>>(items: I) {
    let mut table = prettytable::Table::new();

    table.add_row(row!["Title", "Author", "Link"]);

    for item in items {
        let title = if item.title.len() >= 50 {
            &item.title[0..49]
        } else {
            &item.title
        };

        table.add_row(row![title, item.author.name, item.url]);
    }

    table.printstd();
}

#[derive(Debug, Deserialize, Serialize)]
struct Author {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FeedItem {
    id: String,
    title: String,
    content_text: String,
    url: String,
    date_published: String,
    author: Author,
}

#[derive(Debug, Deserialize, Serialize)]
struct Feed {
    version: String,
    title: String,
    home_page_url: String,
    feed_url: String,
    description: String,
    author: Author,
    items: Vec<FeedItem>,
}

pub static URL: &str = "http://readrust.net/rust2018/feed.json";
pub static URL2: &str = "https://readrust.net/devops-and-deployment/feed.json";

use clap::App;

fn main() {
    let app = App::new("readrust")
        .version("1.0.1-beta.2")
        .author("Shreyas Lad <shadowtemplates@gmail.com>")
        .about("Reads readrust.net")
        .args_from_usage("-n, --number=[NUMBER] 'Only print the NUMBER most recent posts'
                          -c, --count           'Show the count of posts'
                          -a, --about           'About this project'
                          -d, --devops          'Feed from Devops'
                          -r, --rust2018        'Feed from Rust2018'
                          -ntwo, --numbertwo        'Prints the number of devops posts'");
    
        let matches = app.get_matches();

    let feed = get_feed();

    let feed2 = get_another_feed();

    if matches.is_present("count") {
        print_count1(&feed);
        print_count2(&feed2);
    } else if matches.is_present("about") {
        about::about();
    } else if matches.is_present("devops") {
        let iter = feed2.items.iter();

        if let Some(string) = matches.value_of("numbertwo") {
            let number = string.parse().unwrap();
            print_feed_table2(iter.take(number))
        } else {
            print_feed_table2(iter)
        }
    } else if matches.is_present("rust2018") {
        let iter = feed.items.iter();

        if let Some(string) = matches.value_of("number") {
            let number = string.parse().unwrap();
            print_feed_table(iter.take(number))
        } else {
            print_feed_table(iter)
        }
    } else {
        /*let iter = feed.items.iter();

        if let Some(string) = matches.value_of("number") {
            let number = string.parse().unwrap();
            print_feed_table(iter.take(number))
        } else {
            print_feed_table(iter)
        }*/
        println!("No valid aurguments");
    }

}

fn get_another_feed() -> Feed {
    let client = reqwest::Client::new();
    let mut request = client.get(URL2);

    let mut resp = request.send().unwrap();

    assert!(resp.status().is_success());

    let json = resp.text().unwrap();

    serde_json::from_str(&json).unwrap()
}

fn get_feed() -> Feed {
    let client = reqwest::Client::new();
    let mut request = client.get(URL);

    let mut resp = request.send().unwrap();

    assert!(resp.status().is_success());

    let json = resp.text().unwrap();

    serde_json::from_str(&json).unwrap()
}

fn print_count1(feed: &Feed) {
    println!("Number of posts for Rust2018: {}", feed.items.len());
}

fn print_count2(feed2: &Feed) {
    println!("Number of posts for DevOps and Deployment: {}", feed2.items.len());
}