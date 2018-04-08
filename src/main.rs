extern crate clap;
extern crate colored;
extern crate reqwest;
extern crate serde;
mod about;
use colored::*;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate prettytable;

// Table for Rust2018
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

// Table for DevOps
fn print_feed_table2<'feed2, I: Iterator<Item = &'feed2 FeedItem>>(items: I) {
    let mut table = prettytable::Table::new();

    table.add_row(row!["Title".red().bold(), "Author".red().bold(), "Link".red().bold()]);

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

fn print_feed_table3<'feed3, I: Iterator<Item = &'feed3 FeedItem>>(items: I) {
    let mut table = prettytable::Table::new();

    table.add_row(row!["Title".red().bold(), "Author".red().bold(), "Link".red().bold()]);

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
pub static URL3: &str = "https://readrust.net/performance/feed.json";

use clap::App;

fn main() {
    let app = App::new("readrust")
        .version("1.0.2-beta.3")
        .author("Shreyas Lad <shadowtemplates@gmail.com>")
        .about("Reads readrust.net")
        .args_from_usage("-c, --count           'Show the count of posts'
                          -a, --about           'About this project'
                          -d, --devops          'Feed from Devops and Deployment'
                          -r, --rust2018        'Feed from Rust2018'
                          -p, --performance     'Feed from Performance'");
    
        let matches = app.get_matches();
    let feed = get_feed();

    let feed2 = devops_feed();

    let feed3 = performance_feed();

    if matches.is_present("count") {
        println!("{}\n\n", "The count of posts under each topic".red().bold());
        print_count1(&feed);
        print_count2(&feed2);
        print_count3(&feed3);
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
    } else if matches.is_present("performance") {
        let iter = feed3.items.iter();
        if let Some(string) = matches.value_of("number") {
            let number = string.parse().unwrap();
            print_feed_table3(iter.take(number))
        } else {
            print_feed_table3(iter)
        }
    } 
    
    else {
        /*let iter = feed.items.iter();

        if let Some(string) = matches.value_of("number") {
            let number = string.parse().unwrap();
            print_feed_table(iter.take(number))
        } else {
            print_feed_table(iter)
        }*/
        println!("No valid aurguments.\n\nDo {} or {} for more information on usage", "--help".blue().bold(), "-h".blue().bold());
    }

}

// Function for Rust2018 feed
fn get_feed() -> Feed {
    let client = reqwest::Client::new();
    let mut request = client.get(URL);

    let mut resp = request.send().unwrap();

    assert!(resp.status().is_success());

    let json = resp.text().unwrap();

    serde_json::from_str(&json).unwrap()
}

// Function for DevOps feed
fn devops_feed() -> Feed {
    let client = reqwest::Client::new();
    let mut request = client.get(URL2);

    let mut resp = request.send().unwrap();

    assert!(resp.status().is_success());

    let json = resp.text().unwrap();

    serde_json::from_str(&json).unwrap()
}

fn performance_feed() -> Feed {
    let client = reqwest::Client::new();
    let mut request = client.get(URL3);

    let mut resp = request.send().unwrap();

    assert!(resp.status().is_success());

    let json = resp.text().unwrap();

    serde_json::from_str(&json).unwrap()
}

fn print_count1(feed: &Feed) {
    println!("Posts for Rust2018: {}", feed.items.len());
}

fn print_count2(feed2: &Feed) {
    println!("Posts for DevOps and Deployment: {}", feed2.items.len());
}

fn print_count3(feed3: &Feed) {
    println!("Posts for Performance: {}", feed3.items.len());
}