pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news!!! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John DOe"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is actually not falling"),
    };

    // println!("Tweet summary: {}", tweet.summarize());
    // println!("Article summary: {}", article.summarize());
    notify(&article);
}
