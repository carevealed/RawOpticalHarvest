use super::common::*;
use core::fmt;
use std::{
    error::Error,
    path::PathBuf,
};

pub trait FirstValueFetcher
{
    fn get_first_value(
        &self,
        column_header: &String,
    ) -> Result<String, Box<dyn Error>>;
}

impl FirstValueFetcher for PathBuf
{
    fn get_first_value(
        &self,
        column_header: &String,
    ) -> Result<String, Box<dyn Error>>
    {
        let header_i = self.find_single_header_index(&column_header)?;

        let first_row = match self.csv()?.records().next() {
            | Some(r) => Ok(r?),
            | None => Err(Box::new(FirstValueFetchErrorOption::NoRecords)),
        }?;

        Ok(first_row[header_i].to_owned())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FirstValueFetchErrorOption
{
    NoRecords,
}

impl Error for FirstValueFetchErrorOption {}

impl fmt::Display for FirstValueFetchErrorOption
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        let desc = match self {
            | FirstValueFetchErrorOption::NoRecords => {
                "There are no records in the CSV.".to_owned()
            }
        };

        write!(f, "{desc}")
    }
}
