use super::path_reader::PathReader;
use crate::csv_processor::error::*;
use core::fmt;
use log::debug;
use std::{
    error::Error,
    path::PathBuf,
};

pub trait HeaderSearcher
{
    fn find_single_header_index(
        &self,
        column_header: &String,
    ) -> Result<usize, Box<dyn Error>>;
}

impl HeaderSearcher for PathBuf
{
    fn find_single_header_index(
        &self,
        column_header: &String,
    ) -> Result<usize, Box<dyn Error>>
    {
        let matched_headers: Vec<usize> = self
            .csv()?
            .headers()?
            .iter()
            .enumerate()
            .filter(|(_, header)| header.eq(&column_header))
            .map(|(i, _)| i)
            .collect();

        match matched_headers.len() {
            | 0 => {
                Err(Box::new(SingleHeaderSearchError {
                    header_name: column_header.clone(),
                    error: SingleSearchError::NotFound,
                }))
            }
            | 1 => {
                let res = *matched_headers.first().unwrap();
                debug!(
                    "Row empty search found header \"{column_header}\" found \
                     at index {res}"
                );
                Ok(res)
            }
            | _ => {
                Err(Box::new(SingleHeaderSearchError {
                    header_name: column_header.clone(),
                    error: SingleSearchError::TooMany {
                        indices: matched_headers,
                    },
                }))
            }
        }
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

#[cfg(test)]
mod tests
{

    use super::*;
    use std::{
        path::PathBuf,
        str::FromStr,
    };

    #[test]
    fn test_nonequal()
    {
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .find_single_header_index(&"neq".to_string())
            .unwrap();

        assert_eq!(e, 0);
    }

    #[test]
    fn test_equal()
    {
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .find_single_header_index(&"eq".to_string())
            .unwrap();

        assert_eq!(e, 1);
    }

    #[test]
    fn empty()
    {
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .find_single_header_index(&"empty".to_string())
            .unwrap();

        assert_eq!(e, 2);
    }

    #[test]
    fn test_some_empty()
    {
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .find_single_header_index(&"some_empty".to_string())
            .unwrap();

        assert_eq!(e, 3);
    }

    #[test]
    fn test_non_existent()
    {
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .find_single_header_index(&"non_existent".to_string())
            .unwrap_err()
            .as_ref()
            .to_string();

        assert_eq!(
            e,
            "An error occurred while searching for the header \
             \"non_existent\": Not found."
        );
    }
}
