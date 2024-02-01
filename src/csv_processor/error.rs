use core::fmt;

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
