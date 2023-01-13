use lazy_static::lazy_static;
use regex::Regex;

fn main() {
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

    let login = extract_login("ron_burgundy.121.310_2132-hellyaaa!@hollywoods_ass.com");
    println!("{}", login);
}
