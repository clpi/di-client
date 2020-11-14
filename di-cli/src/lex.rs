use chrono_english::parse_date_string;
use chrono_humanize::{Tense, Humanize, HumanTime};
use chrono::prelude::*;

pub struct Parser {
    rules: Vec<Rule>,
}

pub struct Rule {}

pub enum Element {
    Whitespace,
}
