#[cfg(test)]
mod tests
{
    use crate::csv_processor::{
        error::{
            ColumnEqualityCheckError,
            ColumnEqualityCheckErrorOption,
        },
        CsvProcessor,
    };
    use assert_cmd::Command;
    use log::info;

    #[test]
    fn test_cahuca()
    {
        let mut cmd = Command::cargo_bin("carroh").unwrap();
        let assert = cmd
            .arg(
                "demo/cahuca_2023-2024_DG_checkin_2024-01-26 - \
                 cahuca_2023-2024_DG_checkin_2024-01-26.csv",
            )
            .assert();

        assert.success();
    }
    #[test]
    fn test_casfjazz()
    {
        let mut cmd = Command::cargo_bin("carroh").unwrap();
        let assert = cmd
            .arg(
                "demo/casfjazz_2022-2023_DG_DiscHarvest_2024-01-26 - \
                 casfjazz_2022-2023_DG_DiscHarvest_2024-01-26.csv",
            )
            .assert();

        assert.success();
    }

    #[test]
    fn test_nonequal_marc()
    {
        let abs_csv_path =
            std::path::absolute("./demo/nonequal_marc.csv").unwrap();

        info!("Reading CSV file from \"{abs_csv_path:?}\"");

        let mut cp = CsvProcessor::new(abs_csv_path).unwrap();

        let e = cp
            .assert_equal_column_values(&"marc".to_string())
            .unwrap_err();
        let e = e.as_ref();

        assert_eq!(
            format! {"{e}"},
            "An error occurred while verifying all values in a \"marc\" are \
             equal: Non-equal value at line 2"
        );
    }

    #[test]
    fn test_nonequal_grant_cycle()
    {
        let abs_csv_path =
            std::path::absolute("./demo/nonequal_grant_cycle.csv").unwrap();

        info!("Reading CSV file from \"{abs_csv_path:?}\"");

        let mut cp = CsvProcessor::new(abs_csv_path).unwrap();

        let e = cp
            .assert_equal_column_values(&"obj_grant_cycle".to_string())
            .unwrap_err();
        let e = e.as_ref();

        assert_eq!(
            format! {"{e}"},
            "An error occurred while verifying all values in a \
             \"obj_grant_cycle\" are equal: Non-equal value at line 2"
        );
    }
}
