use super::common::*;
use std::{
    error::Error,
    path::PathBuf,
};

pub trait AllRowsFilledValidator
{
    fn all_rows_filled(
        &self,
        column_header: &String,
    ) -> Result<bool, Box<dyn Error>>;
}

impl AllRowsFilledValidator for PathBuf
{
    fn all_rows_filled(
        &self,
        column_header: &String,
    ) -> Result<bool, Box<dyn Error>>
    {
        let header_i = self.find_single_header_index(&column_header)?;

        for r in self.csv()?.records() {
            if (r?[header_i]).eq("") {
                return Ok(false);
            }
        }
        Ok(true)
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
            .all_rows_filled(&"neq".to_string())
            .unwrap();

        assert_eq!(e, true);
    }

    #[test]
    fn test_equal()
    {
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .all_rows_filled(&"eq".to_string())
            .unwrap();

        assert_eq!(e, true);
    }

    #[test]
    fn empty()
    {
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .all_rows_filled(&"empty".to_string())
            .unwrap();

        assert_eq!(e, false);
    }

    #[test]
    fn test_some_empty()
    {
        let e = PathBuf::from_str("./demo/simple_column_tester.csv")
            .unwrap()
            .all_rows_filled(&"some_empty".to_string())
            .unwrap();

        assert_eq!(e, false);
    }
}
