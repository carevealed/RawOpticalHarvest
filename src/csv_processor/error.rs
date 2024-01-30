use core::fmt;
use std::error::Error;

pub struct RowError
{
    pub row_number: usize,
    pub error_description: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SingleSearchError
{
    NotFound,
    TooMany
    {
        indices: Vec<usize>,
    },
}

impl fmt::Display for SingleSearchError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        let desc = match self {
            | SingleSearchError::NotFound => "Not found.".to_string(),
            | SingleSearchError::TooMany { indices } => {
                format!("Multiple results at {indices:?}")
            }
        };

        write!(f, "{desc}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SingleHeaderSearchError
{
    pub header_name: String,
    pub error: SingleSearchError,
}

impl Error for SingleHeaderSearchError {}

impl fmt::Display for SingleHeaderSearchError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        let header_name = &self.header_name;
        let post = &self.error;

        write!(
            f,
            "An error occurred while searching for the header \
             \"{header_name}\": {post}"
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ColumnEqualityCheckErrorOption
{
    UnequalValue
    {
        line_number: usize,
    },
    NoRecords,
}

impl fmt::Display for ColumnEqualityCheckErrorOption
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        let desc = match self {
            | ColumnEqualityCheckErrorOption::UnequalValue { line_number } => {
                format!("Non-equal value at line {line_number}")
            }
            | ColumnEqualityCheckErrorOption::NoRecords => {
                "There are no records in the CSV.".to_owned()
            }
        };

        write!(f, "{desc}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ColumnEqualityCheckError
{
    pub(crate) header_name: String,
    pub(crate) error: ColumnEqualityCheckErrorOption,
}

impl Error for ColumnEqualityCheckError {}

impl fmt::Display for ColumnEqualityCheckError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        let header_name = &self.header_name;
        let post = &self.error;

        write!(
            f,
            "An error occurred while verifying all values in a \
             \"{header_name}\" are equal: {post}"
        )
    }
}
