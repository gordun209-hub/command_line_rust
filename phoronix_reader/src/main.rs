use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
fn open_testing() -> &'static str {
    include_str!("phoronix.html")
}

struct Article {
    title: String,
    // link: String,
    // detail: String,
    // summary: String,
}
impl Article {
    fn get_articles() -> Vec<Article> {
        Document::from_str(open_testing())
            .find(Name("article"))
            .iter()
            .map(|node| Article::new(&node))
            .collect()
    }
    fn new(node: &node) -> Article {
        let header = node.find(Name("a")).first().unwrap();
        Article {
            title: header.text(),
        }
    }
}
fn main() {
    let phoronix = open_testing();
    println!("{}", phoronix)
}
