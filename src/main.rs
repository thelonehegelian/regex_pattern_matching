use lazy_static::lazy_static;
use regex::Regex;

fn extract_login(email: &str) -> String {
    // regex pattern to extract login from email
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<login>[^@]+)@").unwrap();
    }
    // extract login from email
    let login = RE.captures(email).unwrap().name("login").unwrap().as_str();
    // return login
    login.to_string()
}

fn extract_hashtag(text: &str) -> Vec<String> {
    // regex pattern to extract all hashtags from text

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<hashtag>#[a-zA-Z0-9_]+)").unwrap();
    }
    // collect all hashtags from text
    let hashtags = RE
        .captures_iter(text)
        .map(|cap| cap.name("hashtag").unwrap().as_str())
        .collect::<Vec<&str>>();
    // return hashtags
    hashtags.iter().map(|&s| s.to_string()).collect()
}

fn main() {
    let login = extract_login("ron_burgundy.121.310_2132-hellyaaa!@hollywoods_ass.com");
    println!("{}", login);

    let hashtag = extract_hashtag("I love #rust and #regex");
    println!("{:?}", hashtag);
}
