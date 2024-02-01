use super::all_rows_filled::AllRowsFilledValidator;
use core::fmt;
use std::{
    error::Error,
    path::PathBuf,
};

pub trait PopulatedColumn
{
    fn get_populated_column<'a>(
        &self,
        column_header_one: &'a String,
        column_header_two: &'a String,
    ) -> Result<&'a String, Box<dyn Error>>;
}

impl PopulatedColumn for PathBuf
{
    fn get_populated_column<'a>(
        &self,
        column_header_one: &'a String,
        column_header_two: &'a String,
    ) -> Result<&'a String, Box<dyn Error>>
    {
        let ocn_empty = self.all_rows_filled(&column_header_one)?;
        let oti_empty = self.all_rows_filled(&column_header_two)?;

        match (ocn_empty, oti_empty) {
            | (true, _) => Ok(&column_header_one),
            | (false, true) => Ok(&column_header_two),
            | _ => {
                Err(Box::new(GetPopulatedColumnError::BothColumnsAreEmpty {
                    column_header_one: column_header_one.clone(),
                    column_header_two: column_header_two.clone(),
                }))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GetPopulatedColumnError
{
    BothColumnsAreEmpty
    {
        column_header_one: String,
        column_header_two: String,
    },
}
impl Error for GetPopulatedColumnError {}

impl fmt::Display for GetPopulatedColumnError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        let desc = match &self {
            | &GetPopulatedColumnError::BothColumnsAreEmpty {
                column_header_one,
                column_header_two,
            } => {
                format!(
                    "Tried to find a filled column '{column_header_one}' or \
                     '{column_header_two}' but both were at least partially \
                     empty."
                )
            }
        };

        write!(f, "{desc}")
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_same_col()
    {
        let col_one = "neq".to_string();
        let col_two = "neq".to_string();
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .get_populated_column(&col_one, &col_two)
            .unwrap();

        assert_eq!(e, "neq");
    }

    #[test]
    fn test_two_populated()
    {
        let col_one = "eq".to_string();
        let col_two = "neq".to_string();
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .get_populated_column(&col_one, &col_two)
            .unwrap();

        assert_eq!(e, "eq");
    }

    #[test]
    fn test_two_populated_rev()
    {
        let col_one = "neq".to_string();
        let col_two = "eq".to_string();
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .get_populated_column(&col_one, &col_two)
            .unwrap();

        assert_eq!(e, "neq");
    }

    #[test]
    fn test_one_empty()
    {
        let col_one = "neq".to_string();
        let col_two = "empty".to_string();
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .get_populated_column(&col_one, &col_two)
            .unwrap();

        assert_eq!(e, "neq");
    }

    #[test]
    fn test_one_empty_rev()
    {
        let col_one = "empty".to_string();
        let col_two = "neq".to_string();
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .get_populated_column(&col_one, &col_two)
            .unwrap();

        assert_eq!(e, "neq");
    }

    #[test]
    fn test_mostly_empty()
    {
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .get_populated_column(
                &"empty".to_string(),
                &"some_empty".to_string(),
            )
            .unwrap_err()
            .as_ref()
            .to_string();

        assert_eq!(
            e,
            "Tried to find a filled column 'empty' or 'some_empty' but both \
             were at least partially empty."
        );
    }
}
