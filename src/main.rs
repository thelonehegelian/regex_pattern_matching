use lazy_static::lazy_static;
use regex::{Regex, RegexSetBuilder};
use std::io::{BufRead, BufReader};

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

// US phone number struct
#[derive(Debug)]
struct PhoneNumber {
    country_code: String,
    area_code: String,
    number: String,
}

impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "+{}-{}-{}",
            self.country_code, self.area_code, self.number
        )
    }
}

// parse the phone number
fn parse_us_phone_number(phone_number: &str) -> PhoneNumber {
    // !Note: the pattern only matches phone numbers in the US of this format: +1-310-213-1212
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(\+1)?[\s-]?(\d{3})[\s-]?(\d{3})[\s-]?(\d{4})$").unwrap();
    }

    // extract phone number parts
    let captures = RE.captures(phone_number).unwrap();
    let country_code = captures.get(1).map_or("", |m| m.as_str());
    let area_code = captures.get(2).map_or("", |m| m.as_str());
    let number_1 = captures.get(3).map_or("", |m| m.as_str());
    let number_2 = captures.get(4).map_or("", |m| m.as_str());
    let number = format!("{}-{}", number_1, number_2);

    PhoneNumber {
        country_code: country_code.to_string(),
        area_code: area_code.to_string(),
        number: number,
    }
}

fn main() {
    let login = extract_login("ron_burgundy.121.310_2132-hellyaaa!@hollywoods_ass.com");
    println!("{}", login);

    let hashtag = extract_hashtag("what a #beautiful #day to #code #rust!");
    println!("{:?}", hashtag);

    let PhoneNumber = parse_us_phone_number("+1-310-213-1212");
    println!("{:?}", PhoneNumber);

    /*************************************************************
     * READ FILE AND OUTPUT ONLY THE FILE WHICH HAS SPECIFIED TEXT
     ************************************************************/
    // read application.log file (there is no application log file)

    let file = std::fs::read_to_string("application.log");
    match (file) {
        Ok(file) => {
            let reader = BufReader::new(file.as_bytes());
            let ip_regex_set = RegexSetBuilder::new(&[
                r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$",
                r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}$",
            ]);
            // read line by line and match the ip address
            for line in reader.lines() {
                let line = line.unwrap();
                // check if the line has ip address
                if ip_regex_set.build().unwrap().is_match(&line) {
                    println!("{}", line);
                }
            }
        }
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                println!("File not found, skipping code block");
            } else {
                panic!("Failed to open file: {:?}", error);
            }
        }
    }

    /**************************
     * REPLACE TEXT WITH REGEX
     *************************/
    // finds all the dates in the string and replaces them with ISO 2014
    fn reformat_dates(dates: &str) -> String {
        lazy_static! {
            static ref DATE_REGEX: Regex = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
        }
        let new_date = DATE_REGEX.replace_all(dates, "2014-01-01T12:00:00Z");

        new_date.to_string()
    }
    let dates_before = "2012-03-14, 2013-01-15 and 2014-07-05";
    let reformatted_dates = reformat_dates(dates_before);
    println!("{}", reformatted_dates);
}
