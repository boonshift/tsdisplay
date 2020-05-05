extern crate chrono;
extern crate regex;

use chrono::{DateTime, Local};
use regex::Captures;
use regex::Regex;
use std::io;
use std::io::prelude::*;

fn main() {
    let re = Regex::new(r"([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))").unwrap();

    for line in io::stdin().lock().lines() {
        let str = &line.unwrap();
        let result = re.replace(str, |c: &Captures| {
                let d = DateTime::parse_from_rfc3339(&c[0]).unwrap();
                d.with_timezone(&Local).to_rfc3339()
        });
        
        println!("{}", result);
    }
}
