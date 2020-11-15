use std::str::FromStr;
use chrono::{DateTime, Utc};
use clap::{Clap, App, Arg, ArgEnum};

#[derive(Clap, Debug)]
pub struct FieldCmd {
    #[clap(long="name", about="name of field")]
    name: String,
    #[clap(long="name", about="Optional value of field")]
    value: Option<String>,
    #[clap(
        long="type",
        about="Optional type of field (default is String)",
        default_value="FieldType::default",
    )]
    field_type: FieldType,
}

pub fn new() -> App<'static> {
    App::new("field")
        .short_flag("F".chars().nth(0).unwrap())
        .about("Define a new field or update an exisitng field")
}

#[derive(Debug)]
pub enum FieldType {
    Text,
    Int,
    Decimal,
    Choice,
    None,
}

impl FieldType {

}

impl Default for FieldType {
    fn default() -> Self { FieldType::Text }
}

impl FromStr for FieldType {
    type Err = std::str::Utf8Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(FieldType::Text),
            "int" => Ok(FieldType::Int),
            "decimal" => Ok(FieldType::Decimal),
            "choice" => Ok(FieldType::Choice),
            _ => Ok(FieldType::None),
        }
    }
}

impl ToString for FieldType {
    fn to_string(&self) -> String {
        match self {
            FieldType::Text => "text".into(),
            FieldType::Int => "int".into(),
            FieldType::Decimal => "decimal".into(),
            FieldType::Choice => "choice".into(),
            _ => "none".into(),
        }
    }
}
