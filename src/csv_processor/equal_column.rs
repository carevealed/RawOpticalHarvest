use super::common::*;
use core::fmt;
use log::debug;
use std::{
    error::Error,
    path::PathBuf,
};

pub(crate) trait EqualColumnValidator
{
    fn assert_equal_column_values(
        &self,
        column_header: &String,
    ) -> Result<(), Box<dyn Error>>;
}

impl EqualColumnValidator for PathBuf
{
    fn assert_equal_column_values(
        &self,
        column_header: &String,
    ) -> Result<(), Box<dyn Error>>
    {
        let header_i = self.find_single_header_index(&column_header)?;

        let mut csv = self.csv()?;
        let mut all_rows = csv.records().enumerate();

        let first = match all_rows.next() {
            | Some((_, r)) => Ok(r?),
            | None => {
                Err(Box::new(ColumnEqualityCheckError {
                    header_name: column_header.clone(),
                    error: ColumnEqualityCheckErrorOption::NoRecords,
                }))
            }
        }?;

        let b = &first[header_i];

        for (i, r) in all_rows {
            let a = &r?[header_i];

            debug!("Checking '{a}' == '{b}'");

            if !(a.eq(b)) {
                return Err(Box::new(ColumnEqualityCheckError {
                    header_name: column_header.clone(),
                    error: ColumnEqualityCheckErrorOption::UnequalValue {
                        line_number: i + 1,
                    },
                }));
            }
        }

        Ok(())
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

#[cfg(test)]
mod tests
{
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_nonequal()
    {
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .assert_equal_column_values(&"neq".to_string())
            .unwrap_err();
        let e = e.as_ref();

        assert_eq!(
            format! {"{e}"},
            "An error occurred while verifying all values in a \"neq\" are \
             equal: Non-equal value at line 4"
        );
    }

    #[test]
    fn test_equal()
    {
        PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .assert_equal_column_values(&"eq".to_string())
            .unwrap();
    }

    #[test]
    fn empty()
    {
        PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .assert_equal_column_values(&"empty".to_string())
            .unwrap();
    }

    #[test]
    fn test_some_empty()
    {
        PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .assert_equal_column_values(&"some_empty".to_string())
            .unwrap_err();
    }
}
